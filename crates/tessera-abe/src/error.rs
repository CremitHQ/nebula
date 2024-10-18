use tessera_policy::error::PolicyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ABEError {
    #[error("Policy error: {0}")]
    PolicyError(#[from] PolicyError),

    #[error("Does not satisfy the policy")]
    PolicyNotSatisfied,

    #[error("Invalid Policy: {0}")]
    InvalidPolicy(String),

    #[error("AES-GCM module error: {0}")]
    AESGCMError(#[from] AESGCMError),

    #[error("ABE encryption error: {0}")]
    EncryptionError(String),

    #[error("ABE decryption error: {0}")]
    DecryptionError(String),
}

#[derive(Error, Debug)]
pub enum AESGCMError {
    #[error("AES-GCM encryption error: {0}")]
    EncryptionError(aes_gcm::Error),

    #[error("AES-GCM decryption error: {0}")]
    DecryptionError(aes_gcm::Error),

    #[error("AES-GCM nonce size mismatch")]
    NonceSizeMismatch,
}