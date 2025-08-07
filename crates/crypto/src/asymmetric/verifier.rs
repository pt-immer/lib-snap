#[derive(Clone)]
pub struct Crypto {
    inner: rsa::pkcs1v15::VerifyingKey<rsa::sha2::Sha256>,
}

impl Crypto {
    pub fn create(pk_pem: &str) -> crate::Result<Self> {
        let inner =
            <rsa::pkcs1v15::VerifyingKey<rsa::sha2::Sha256> as rsa::pkcs8::DecodePublicKey>::from_public_key_pem(
                pk_pem,
            ).map_err(|_| crate::CryptoError::InvalidPEMPublicKey)?;

        Ok(Self { inner })
    }

    pub fn verify_base64<S, P>(&self, signature_base64: S, payload: P) -> crate::Result<()>
    where
        S: AsRef<str> + std::fmt::Display + std::fmt::Debug,
        P: AsRef<[u8]>,
    {
        let signature_decoded =
            base64::Engine::decode(&base64::prelude::BASE64_STANDARD, signature_base64.as_ref())
                .map_err(|_| crate::CryptoError::BadSignatureFormat)?;
        let payload = payload.as_ref();
        let signature = rsa::pkcs1v15::Signature::try_from(signature_decoded.as_slice())
            .map_err(|_| crate::CryptoError::BadSignatureFormat)?;
        rsa::signature::Verifier::verify(&self.inner, payload, &signature)
            .map_err(|_| crate::CryptoError::SignatureVerificationFailedAsymmetric)?;

        Ok(())
    }
}
