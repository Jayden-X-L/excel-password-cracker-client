use tauri::{AppHandle, State};

use crate::{hash, process, state::AppState};

#[tauri::command]
pub fn extract_hash(app: AppHandle, paths: Vec<String>, customOffice2: Option<String>) -> Result<Vec<(String, String, u32)>, String> {
  let mut out = Vec::new();
  for p in paths {
    let h = match &customOffice2 {
      Some(bin) => hash::extract_office_hash_with(&app, &p, bin)?,
      None => hash::extract_office_hash(&app, &p)?,
    };
    let m = hash::map_office_mode(&h).ok_or_else(|| format!("无法识别哈希模式: {}", &h.chars().take(120).collect::<String>()))?;
    out.push((p, h, m));
  }
  Ok(out)
}

#[tauri::command]
pub fn start_attack(app: AppHandle, state: State<AppState>, req: process::AttackRequest) -> Result<(), String> {
  let mut proc = process::start(app.clone(), req.clone())?;
  {
    let mut s = state.process.lock().map_err(|_| "状态锁定失败")?;
    *s = Some(proc);
  }
  if let Some(sess) = req.session.clone() {
    let mut ss = state.session.lock().map_err(|_| "状态锁定失败")?;
    *ss = Some(sess);
  }
  Ok(())
}

#[tauri::command]
pub fn control_attack(state: State<AppState>, action: String) -> Result<(), String> {
  let mut g = state.process.lock().map_err(|_| "状态锁定失败")?;
  if let Some(ref mut p) = *g {
    match action.as_str() { "pause" => process::send_ctrl(p, 'p'), "resume" => process::send_ctrl(p, 'r'), "stop" => process::send_ctrl(p, 'q'), _ => Err("未知操作".into()) }
  } else { Err("没有正在运行的任务".into()) }
}

#[tauri::command]
pub fn restore_session(app: AppHandle, state: State<AppState>) -> Result<(), String> {
  let s = state.session.lock().map_err(|_| "状态锁定失败")?;
  if let Some(sess) = s.clone() {
    process::restore(app, sess)
  } else { Err("没有可恢复的会话".into()) }
}

#[tauri::command]
pub fn list_devices(app: AppHandle, custom_hashcat: Option<String>) -> Result<String, String> {
  hash::list_devices(&app, custom_hashcat)
}