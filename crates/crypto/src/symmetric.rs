#[derive(Clone)]
pub struct Crypto {
    inner: hmac::Hmac<sha2::Sha512>,
}

impl Crypto {
    pub fn create(secret: &str) -> crate::Result<Self> {
        Self::from_slice(secret.as_bytes())
    }

    pub fn sign_once<P: AsRef<[u8]>>(secret: &str, payload: P) -> crate::Result<String> {
        let mut instance = Self::create(secret)?;

        Ok(instance.sign(payload))
    }

    pub fn from_slice<S: AsRef<[u8]>>(slice_secret: S) -> crate::Result<Self> {
        let inner = <hmac::Hmac<sha2::Sha512> as hmac::digest::Mac>::new_from_slice(slice_secret.as_ref())
            .map_err(|_| crate::CryptoError::InvalidSecretLength)?;

        Ok(Self { inner })
    }

    pub fn sign<P: AsRef<[u8]>>(&mut self, payload: P) -> String {
        hmac::digest::Update::update(&mut self.inner, payload.as_ref());
        let signature = hmac::digest::Mac::finalize_reset(&mut self.inner)
            .into_bytes()
            .to_vec();

        base64::Engine::encode(&base64::prelude::BASE64_STANDARD, &signature)
    }

    pub fn verify<S, P>(&mut self, signature: S, payload: P) -> crate::Result<()>
    where
        S: AsRef<str>,
        P: AsRef<[u8]>,
    {
        let signature = base64::Engine::decode(&base64::prelude::BASE64_STANDARD, signature.as_ref())
            .map_err(|_| crate::error::Error::BadSignatureFormat)?;
        hmac::digest::Mac::update(&mut self.inner, payload.as_ref());
        hmac::digest::Mac::verify_slice_reset(&mut self.inner, signature.as_ref())
            .map_err(|_| crate::error::Error::SignatureVerificationFailed)?;

        Ok(())
    }
}
