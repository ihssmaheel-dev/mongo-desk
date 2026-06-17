use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use aes_gcm::aead::rand_core::RngCore;

use crate::error::AppError;

pub struct EncryptionService {
    key: [u8; 32],
}

impl EncryptionService {
    pub fn new() -> Result<Self, AppError> {
        let key = Self::derive_key()?;
        Ok(Self { key })
    }

    fn derive_key() -> Result<[u8; 32], AppError> {
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "default".to_string());
        
        let username = std::env::var("USERNAME")
            .or_else(|_| std::env::var("USER"))
            .unwrap_or_else(|_| "default".to_string());

        let mut hasher = sha2::Sha256::new();
        use sha2::Digest;
        hasher.update(format!("{}-{}", hostname, username));
        let result = hasher.finalize();
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        Ok(key)
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, AppError> {
        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to initialize encryption.".into(),
            })?;

        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Encryption failed.".into(),
            })?;

        let mut result = nonce_bytes.to_vec();
        result.extend(ciphertext);
        Ok(result)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<String, AppError> {
        if ciphertext.len() < 12 {
            return Err(AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Invalid ciphertext.".into(),
            });
        }

        let cipher = Aes256Gcm::new_from_slice(&self.key)
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to initialize encryption.".into(),
            })?;

        let nonce = Nonce::from_slice(&ciphertext[..12]);
        let plaintext = cipher.decrypt(nonce, &ciphertext[12..])
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Decryption failed.".into(),
            })?;

        String::from_utf8(plaintext)
            .map_err(|e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Invalid UTF-8 in decrypted data.".into(),
            })
    }
}
