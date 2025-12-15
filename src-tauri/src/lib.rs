mod state;
mod hash;
mod process;
mod commands;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      app.handle().plugin(tauri_plugin_dialog::init());
      Ok(())
    })
    .manage(crate::state::AppState::default())
    .invoke_handler(tauri::generate_handler![
      crate::commands::extract_hash,
      crate::commands::start_attack,
      crate::commands::control_attack,
      crate::commands::restore_session,
      crate::commands::list_devices
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
