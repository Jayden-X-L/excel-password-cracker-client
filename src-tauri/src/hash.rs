use std::io::Read;
use std::path::PathBuf;
use std::os::unix::fs::PermissionsExt;
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;
fn unzip_to(zip_path: &PathBuf, dest: &PathBuf) -> Result<(), String> {
  let file = std::fs::File::open(zip_path).map_err(|e| format!("打开资源失败: {}", e))?;
  let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("解压失败: {}", e))?;
  for i in 0..archive.len() {
    let mut entry = archive.by_index(i).map_err(|e| format!("读取压缩条目失败: {}", e))?;
    let outpath = dest.join(entry.mangled_name());
    if entry.is_dir() {
      std::fs::create_dir_all(&outpath).ok();
    } else {
      if let Some(parent) = outpath.parent() { std::fs::create_dir_all(parent).ok(); }
      let mut outfile = std::fs::File::create(&outpath).map_err(|e| format!("写入文件失败: {}", e))?;
      std::io::copy(&mut entry, &mut outfile).map_err(|e| format!("复制文件失败: {}", e))?;
      #[cfg(unix)] {
        if let Some(mode) = entry.unix_mode() { let _ = std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode)); }
      }
    }
  }
  Ok(())
}

pub fn map_office_mode(hash: &str) -> Option<u32> {
  let h = hash.to_ascii_lowercase();
  if h.contains("$office$") {
    if h.contains("2007") { return Some(9400); }
    if h.contains("2010") { return Some(9500); }
    if h.contains("2013") || h.contains("2016") || h.contains("2019") || h.contains("2021") { return Some(9600); }
  }
  None
}

pub fn resolve_office2hashcat(app: &AppHandle) -> Option<PathBuf> {
  let resolver = app.path();
  #[cfg(target_os = "macos")]
  {
    #[cfg(target_arch = "aarch64")]
    { return resolver.resolve("resources/office2hashcat/macos-arm64/office2hashcat.py", BaseDirectory::Resource).ok(); }
    #[cfg(target_arch = "x86_64")]
    { return resolver.resolve("resources/office2hashcat/macos-x64/office2hashcat.py", BaseDirectory::Resource).ok(); }
  }
  #[cfg(target_os = "windows")]
  { return resolver.resolve("resources/office2hashcat/windows-x64/office2hashcat.exe", BaseDirectory::Resource).ok(); }
  None
}

pub fn resolve_hashcat(app: &AppHandle) -> Option<PathBuf> {
  let resolver = app.path();
  #[cfg(target_os = "macos")]
  {
    #[cfg(target_arch = "aarch64")]
    { return resolver.resolve("resources/hashcat/macos-arm64/hashcat", BaseDirectory::Resource).ok(); }
    #[cfg(target_arch = "x86_64")]
    { return resolver.resolve("resources/hashcat/macos-x64/hashcat", BaseDirectory::Resource).ok(); }
  }
  #[cfg(target_os = "windows")]
  { return resolver.resolve("resources/hashcat/windows-x64/hashcat.exe", BaseDirectory::Resource).ok(); }
  None
}

pub fn resolve_hashcat_dist_root(app: &AppHandle) -> Option<PathBuf> {
  let resolver = app.path();
  #[cfg(target_os = "macos")]
  {
    #[cfg(target_arch = "aarch64")]
    { return resolver.resolve("resources/hashcat/macos-arm64/hashcat-dist", BaseDirectory::Resource).ok(); }
    #[cfg(target_arch = "x86_64")]
    { return resolver.resolve("resources/hashcat/macos-x64/hashcat-dist", BaseDirectory::Resource).ok(); }
  }
  None
}

pub fn ensure_hashcat(app: &AppHandle) -> Option<PathBuf> {
  if let Some(root) = resolve_hashcat_dist_root(app) {
    let candidates = [root.join("bin/hashcat"), root.join("hashcat")];
    for exe in candidates.iter() {
      if exe.exists() { let _ = std::fs::set_permissions(exe, std::fs::Permissions::from_mode(0o755)); return Some(exe.clone()); }
    }
  }
  if let Some(p) = resolve_hashcat(app) { if p.exists() { return Some(p); } }
  // zip fallback to cache
  let resolver = app.path();
  #[cfg(target_os = "macos")]
  {
    #[cfg(target_arch = "aarch64")]
    {
      if let Ok(zip) = resolver.resolve("resources/hashcat/macos-arm64/hashcat.zip", BaseDirectory::Resource) {
        if zip.exists() {
          if let Ok(cache) = app.path().app_cache_dir() {
            let dest = cache.join("vendor/hashcat/macos-arm64");
            let _ = std::fs::create_dir_all(&dest);
            if unzip_to(&zip, &dest).is_ok() {
              let candidates = [dest.join("bin/hashcat"), dest.join("hashcat")];
              for exe in candidates.iter() { if exe.exists() { let _ = std::fs::set_permissions(exe, std::fs::Permissions::from_mode(0o755)); return Some(exe.clone()); } }
            }
          }
        }
      }
    }
  }
  None
}

pub fn extract_office_hash(app: &AppHandle, file: &str) -> Result<String, String> {
  let bin = resolve_office2hashcat(app).ok_or_else(|| "未找到 office2hashcat 资源".to_string())?;
  if !bin.exists() { return Err("office2hashcat 资源缺失，请在设置中指定路径".into()); }
  let mut cmd = if bin.extension().and_then(|e| e.to_str()) == Some("py") {
    let mut c = std::process::Command::new("python3");
    c.arg(bin);
    c
  } else {
    std::process::Command::new(bin)
  };
  cmd.arg(file);
  cmd.stdout(std::process::Stdio::piped());
  let mut child = cmd.spawn().map_err(|e| format!("执行失败: {}", e))?;
  let mut out = String::new();
  if let Some(mut s) = child.stdout.take() {
    let _ = s.read_to_string(&mut out);
  }
  let status = child.wait().map_err(|e| format!("等待进程失败: {}", e))?;
  if !status.success() { return Err("office2hashcat 执行失败".into()); }
  let trimmed = out.trim();
  if trimmed.is_empty() { return Err("未提取到哈希".into()); }
  Ok(trimmed.to_string())
}

pub fn extract_office_hash_with(app: &AppHandle, file: &str, bin_path: &str) -> Result<String, String> {
  let bin = PathBuf::from(bin_path);
  if !bin.exists() { return Err("office2hashcat 路径不存在".into()); }
  let mut cmd = std::process::Command::new(bin);
  cmd.arg(file);
  cmd.stdout(std::process::Stdio::piped());
  let mut child = cmd.spawn().map_err(|e| format!("执行失败: {}", e))?;
  let mut out = String::new();
  if let Some(mut s) = child.stdout.take() {
    let _ = s.read_to_string(&mut out);
  }
  let status = child.wait().map_err(|e| format!("等待进程失败: {}", e))?;
  if !status.success() { return Err("office2hashcat 执行失败".into()); }
  let trimmed = out.trim();
  if trimmed.is_empty() { return Err("未提取到哈希".into()); }
  Ok(trimmed.to_string())
}

pub fn list_devices(app: &AppHandle, custom_hashcat: Option<String>) -> Result<String, String> {
  if let Some(p) = custom_hashcat {
    let bin = PathBuf::from(p);
    if !bin.exists() { return Err("hashcat 路径不存在".into()); }
    let output = std::process::Command::new(bin)
      .arg("-I")
      .output()
      .map_err(|e| format!("执行失败: {}", e))?;
    if !output.status.success() { return Err("hashcat -I 执行失败".into()); }
    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
  }
  if let Some(bin) = ensure_hashcat(app) {
    let mut cmd = std::process::Command::new(&bin);
    if let Some(parent) = bin.parent() { if let Some(root) = parent.parent() { cmd.current_dir(root); } }
    let output = cmd
      .arg("-I")
      .output()
      .map_err(|e| format!("执行失败: {}", e))?;
    if !output.status.success() { return Err("hashcat -I 执行失败".into()); }
    return Ok(String::from_utf8_lossy(&output.stdout).to_string());
  }
  let output = std::process::Command::new("hashcat")
    .arg("-I")
    .output()
    .map_err(|e| format!("执行失败: {}", e))?;
  if !output.status.success() { return Err("hashcat -I 执行失败".into()); }
  Ok(String::from_utf8_lossy(&output.stdout).to_string())
}