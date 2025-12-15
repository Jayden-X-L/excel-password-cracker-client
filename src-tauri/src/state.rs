use std::sync::Mutex;

pub struct AppState {
  pub process: Mutex<Option<crate::process::HashcatProcess>>,
  pub session: Mutex<Option<String>>,
}

impl Default for AppState {
  fn default() -> Self {
    Self {
      process: Mutex::new(None),
      session: Mutex::new(None),
    }
  }
}