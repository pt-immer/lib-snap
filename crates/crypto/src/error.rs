#[derive(Debug, Clone)]
#[derive(thiserror::Error)]
pub enum Error {
    #[error("Bad signature format")]
    BadSignatureFormat,
    #[error("Signature verification failed (asymmetric)")]
    SignatureVerificationFailedAsymmetric,
    #[error("Signature verification failed (symmetric)")]
    SignatureVerificationFailedSymmetric,
    #[error("Invalid PEM (Public Key")]
    InvalidPEMPublicKey,
    #[error("Invalid PEM (Secret Key)")]
    InvalidPEMSecretKey,
    #[error("Invalid secret length")]
    InvalidSecretLength,
}

impl From<Error> for kamu_snap_response::ResponseError {
    fn from(value: Error) -> Self {
        match value {
            Error::BadSignatureFormat => {
                kamu_snap_response::ResponseError::Unathorized("Signature".to_owned())
            }
            Error::SignatureVerificationFailedAsymmetric => {
                kamu_snap_response::ResponseError::Unathorized("Signature".to_owned())
            }
            Error::SignatureVerificationFailedSymmetric => {
                kamu_snap_response::ResponseError::Unathorized("Verify Client Secret Fail.".to_owned())
            }
            Error::InvalidPEMPublicKey => kamu_snap_response::ResponseError::InternalServerError,
            Error::InvalidPEMSecretKey => kamu_snap_response::ResponseError::InternalServerError,
            Error::InvalidSecretLength => kamu_snap_response::ResponseError::InternalServerError,
        }
    }
}
