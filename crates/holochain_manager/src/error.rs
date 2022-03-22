use lair_keystore_manager::error::LaunchLairKeystoreError;

pub enum LaunchHolochainError {
    BinaryNotFound,
    FailedToExecute(String),
    CouldNotConnectToConductor(String),
    KeystoreError(LaunchLairKeystoreError),
}
