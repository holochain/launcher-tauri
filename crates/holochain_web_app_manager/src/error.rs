use holochain_manager::error::LaunchHolochainError;

pub enum LaunchWebAppManagerError {
    LaunchHolochainError(LaunchHolochainError),
    CouldNotGetAppPort(String)
}
