#[derive(Clone)]
pub struct Crypto {
    inner: rsa::pkcs1v15::SigningKey<sha2::Sha256>,
}

impl Crypto {
    pub fn create(sk_pem: &str) -> crate::Result<Self> {
        let inner =
            <rsa::pkcs1v15::SigningKey<sha2::Sha256> as rsa::pkcs8::DecodePrivateKey>::from_pkcs8_pem(sk_pem)
                .map_err(|_| crate::CryptoError::InvalidPEMSecretKey)?;

        Ok(Self { inner })
    }

    pub fn sign_as_base64<P: AsRef<[u8]>>(&mut self, payload: P) -> String {
        let payload = payload.as_ref();
        let signature = rsa::signature::SignerMut::sign(&mut self.inner, payload);
        let signature_bytes = rsa::signature::SignatureEncoding::to_bytes(&signature);

        base64::Engine::encode(&base64::prelude::BASE64_STANDARD, signature_bytes)
    }
}
