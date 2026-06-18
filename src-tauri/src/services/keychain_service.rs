use keyring::Entry;
use crate::error::AppError;

#[allow(dead_code)]
pub struct KeychainService;

#[allow(dead_code)]
impl KeychainService {
    pub fn store_credential(service: &str, username: &str, password: &str) -> Result<(), AppError> {
        let entry = Entry::new(service, username)
            .map_err(|_e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to access credential store.".into(),
            })?;

        entry.set_password(password)
            .map_err(|_e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to store credential.".into(),
            })?;

        Ok(())
    }

    pub fn get_credential(service: &str, username: &str) -> Result<Option<String>, AppError> {
        let entry = Entry::new(service, username)
            .map_err(|_e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to access credential store.".into(),
            })?;

        match entry.get_password() {
            Ok(password) => Ok(Some(password)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(_e) => Err(AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to retrieve credential.".into(),
            }),
        }
    }

    pub fn delete_credential(service: &str, username: &str) -> Result<(), AppError> {
        let entry = Entry::new(service, username)
            .map_err(|_e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to access credential store.".into(),
            })?;

        entry.delete_credential()
            .map_err(|_e| AppError::Internal {
                code: "ERR_INTERNAL".into(),
                message: "Failed to delete credential.".into(),
            })?;

        Ok(())
    }
}
