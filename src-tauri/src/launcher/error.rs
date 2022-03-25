#[derive(Clone)]
pub enum RunLauncherError {
  AnotherInstanceIsAlreadyRunning,
  ErrorLaunching(String),
}
