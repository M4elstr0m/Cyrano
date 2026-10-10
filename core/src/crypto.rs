mod error;
use chacha20poly1305::{KeyInit, XChaCha20Poly1305, XNonce, aead::Aead};
pub use error::Error;

use argon2::Argon2;
use zeroize::Zeroizing;

pub(crate) type Salt = [u8; SALT_LEN];
pub(crate) type DerivedKey = [u8; KEY_LEN];
type NonceBytes = [u8; NONCE_LEN];

pub(crate) const SALT_LEN: usize = 16;
const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 24;

pub(crate) fn generate_salt() -> Result<Salt, Error> {
    let mut salt: Salt = [0; SALT_LEN];
    getrandom::fill(&mut salt)?;

    Ok(salt)
}

pub(crate) fn derive_key(password: &str, salt: &[u8]) -> Result<Zeroizing<DerivedKey>, Error> {
    let mut key: Zeroizing<DerivedKey> = Zeroizing::new([0; KEY_LEN]);

    Argon2::default().hash_password_into(password.as_bytes(), salt, &mut key[..])?;

    Ok(key)
}

pub(crate) fn encrypt_with_key(key: &DerivedKey, plaintext: &[u8]) -> Result<Vec<u8>, Error> {
    let mut nonce_bytes: NonceBytes = [0; NONCE_LEN];
    getrandom::fill(&mut nonce_bytes)?;
    let nonce: XNonce = nonce_bytes.into();

    let cipher = XChaCha20Poly1305::new(&(*key).into());
    let ciphertext = cipher.encrypt(&nonce, plaintext)?;

    let mut blob = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    blob.extend_from_slice(&nonce_bytes);
    blob.extend_from_slice(&ciphertext);

    Ok(blob)
}

pub(crate) fn decrypt_with_key(key: &DerivedKey, blob: &[u8]) -> Result<Vec<u8>, Error> {
    if blob.len() < NONCE_LEN {
        return Err(Error::InvalidBlob);
    }

    let (nonce_bytes, ciphertext) = blob.split_at(NONCE_LEN);
    let nonce: XNonce = XNonce::try_from(nonce_bytes).map_err(|_| Error::InvalidBlob)?;

    let cipher = XChaCha20Poly1305::new(&(*key).into());
    let plaintext = cipher.decrypt(&nonce, ciphertext)?;

    Ok(plaintext)
}
