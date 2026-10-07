mod error;
use chacha20poly1305::{KeyInit, XChaCha20Poly1305, XNonce, aead::Aead};
pub use error::Error;

use argon2::Argon2;

type Salt = [u8; SALT_LEN];
type DerivedKey = [u8; KEY_LEN];
type NonceBytes = [u8; NONCE_LEN];

const SALT_LEN: usize = 16;
const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 24;

fn derive_key(password: &str, salt: &[u8]) -> Result<DerivedKey, Error> {
    let mut key: DerivedKey = [0; KEY_LEN];

    Argon2::default().hash_password_into(password.as_bytes(), salt, &mut key)?;

    Ok(key)
}

pub fn encrypt(plaintext: &[u8], password: &str) -> Result<Vec<u8>, Error> {
    let mut salt: Salt = [0; SALT_LEN];
    getrandom::fill(&mut salt)?;

    let key: DerivedKey = derive_key(password, &salt)?;

    let mut nonce_bytes: NonceBytes = [0; NONCE_LEN];
    getrandom::fill(&mut nonce_bytes)?;

    let nonce: XNonce = nonce_bytes.into();

    let cipher = XChaCha20Poly1305::new(&key.into());
    let ciphertext = cipher.encrypt(&nonce, plaintext)?;

    let mut blob = Vec::with_capacity(SALT_LEN + NONCE_LEN + ciphertext.len());

    blob.extend_from_slice(&salt);
    blob.extend_from_slice(&nonce_bytes);
    blob.extend_from_slice(&ciphertext);

    Ok(blob)
}

pub fn decrypt(blob: &[u8], password: &str) -> Result<Vec<u8>, Error> {
    if blob.len() < SALT_LEN + NONCE_LEN {
        return Err(Error::InvalidBlob);
    }

    let (salt, rest) = blob.split_at(SALT_LEN);
    let (nonce_bytes, ciphertext) = rest.split_at(NONCE_LEN);

    let key: DerivedKey = derive_key(password, salt)?;
    let nonce: XNonce = XNonce::try_from(nonce_bytes).map_err(|_| Error::InvalidBlob)?;

    let cipher = XChaCha20Poly1305::new(&key.into());
    let plaintext = cipher.decrypt(&nonce, ciphertext)?;

    Ok(plaintext)
}
