use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::{process::Stdio, thread};
use tauri::{AppHandle, Emitter, Manager};

use crate::hash::{ensure_hashcat, resolve_hashcat_dist_root};

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AttackRequest {
  pub hash: String,
  pub mode: u32,
  pub attack: u8,
  pub dict: Option<String>,
  pub mask: Option<String>,
  pub increment_min: Option<u32>,
  pub increment_max: Option<u32>,
  pub custom_sets: Option<Vec<(String, String)>>,
  pub devices: Option<Vec<u32>>,
  pub device_type: Option<u8>,
  pub session: Option<String>,
  pub custom_hashcat: Option<String>,
}

pub struct HashcatProcess {
  pub child: std::process::Child,
}

fn build_command(app: &AppHandle, req: &AttackRequest, hash_file: &PathBuf) -> Result<std::process::Command, String> {
  let mut cmd = if let Some(p) = &req.custom_hashcat {
    let b = PathBuf::from(p);
    if !b.exists() { return Err("hashcat 路径不存在".into()); }
    let mut c = std::process::Command::new(&b);
    if let Some(root) = b.parent().and_then(|d| d.parent()) { c.current_dir(root); }
    c
  } else if let Some(b) = ensure_hashcat(app) {
    let mut c = std::process::Command::new(&b);
    if let Some(parent) = b.parent() { if let Some(root) = parent.parent() { c.current_dir(root); } }
    c
  } else {
    std::process::Command::new("hashcat")
  };
  cmd.arg("-m").arg(req.mode.to_string());
  cmd.arg("--status").arg("--status-json").arg("--status-timer=1");
  if let Some(s) = &req.session { cmd.arg("--session").arg(s); }
  if let Some(dt) = req.device_type { cmd.arg("-D").arg(dt.to_string()); }
  if let Some(ds) = &req.devices { for d in ds { cmd.arg("-d").arg(d.to_string()); } }
  if let Some(sets) = &req.custom_sets {
    for (idx, val) in sets {
      cmd.arg(format!("-{}", idx)).arg(val);
    }
  }
  cmd.arg(hash_file);
  match req.attack {
    0 => {
      cmd.arg("-a").arg("0");
      if let Some(dict) = &req.dict { cmd.arg(dict); }
      else { return Err("字典文件未提供".into()); }
    }
    3 => {
      cmd.arg("-a").arg("3");
      if let Some(mask) = &req.mask { cmd.arg(mask); }
      else { return Err("掩码未提供".into()); }
      if let (Some(min), Some(max)) = (req.increment_min, req.increment_max) {
        cmd.arg("--increment");
        cmd.arg("--increment-min").arg(min.to_string());
        cmd.arg("--increment-max").arg(max.to_string());
      }
    }
    _ => return Err("不支持的攻击模式".into()),
  }
  cmd.stdout(Stdio::piped()).stderr(Stdio::piped()).stdin(Stdio::piped());
  Ok(cmd)
}

pub fn start(app: AppHandle, req: AttackRequest) -> Result<HashcatProcess, String> {
  let tmp = app.path().app_cache_dir().map_err(|_| "无法获取缓存目录".to_string())?;
  std::fs::create_dir_all(&tmp).ok();
  let hash_path = tmp.join("epcc_hash.txt");
  std::fs::write(&hash_path, req.hash.as_bytes()).map_err(|e| format!("写入哈希失败: {}", e))?;
  let mut cmd = build_command(&app, &req, &hash_path)?;
  let mut child = cmd.spawn().map_err(|e| format!("启动失败: {}", e))?;
  let _ = app.emit("hashcat-status", "启动 hashcat 任务...");
  let app_handle = app.clone();
  let stdout = child.stdout.take();
  thread::spawn(move || {
    if let Some(out) = stdout {
      let reader = std::io::BufReader::new(out);
      for line in reader.lines() {
        if let Ok(l) = line { let _ = app_handle.emit("hashcat-status", l); }
      }
    }
  });
  let app_handle_err = app.clone();
  let stderr = child.stderr.take();
  thread::spawn(move || {
    if let Some(err) = stderr {
      let reader = std::io::BufReader::new(err);
      for line in reader.lines() {
        if let Ok(l) = line { let _ = app_handle_err.emit("hashcat-status", l); }
      }
    }
  });
  Ok(HashcatProcess { child })
}

pub fn send_ctrl(proc: &mut HashcatProcess, key: char) -> Result<(), String> {
  if let Some(mut stdin) = proc.child.stdin.take() {
    stdin.write_all(&[key as u8]).map_err(|e| format!("写入失败: {}", e))?;
    stdin.flush().ok();
    Ok(())
  } else { Err("无法访问子进程 stdin".into()) }
}

pub fn restore(app: AppHandle, session: String) -> Result<(), String> {
  let status = if let Some(bin) = ensure_hashcat(&app) {
    std::process::Command::new(bin)
      .arg("--restore")
      .arg("--session").arg(session)
      .status()
      .map_err(|e| format!("恢复失败: {}", e))?
  } else {
    std::process::Command::new("hashcat")
      .arg("--restore")
      .arg("--session").arg(session)
      .status()
      .map_err(|e| format!("恢复失败: {}", e))?
  };
  if !status.success() { return Err("恢复命令执行失败".into()); }
  Ok(())
}