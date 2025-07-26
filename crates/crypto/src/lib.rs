pub mod asymmetric;
pub mod error;
pub mod symmetric;

pub use asymmetric::{AsymmetricCryptoSigner, AsymmetricCryptoVerifier};
pub use error::Error as CryptoError;
pub use symmetric::Crypto as SymmetricCrypto;

pub type Result<T> = core::result::Result<T, CryptoError>;
