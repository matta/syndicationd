use std::{borrow::Borrow, io, path::PathBuf};

use crate::{
    auth::{Credential, CredentialError, Unverified},
    config,
};

pub struct Cache {
    dir: PathBuf,
}

impl Cache {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    /// Persist credential in filesystem.
    /// This is blocking operation.
    pub fn persist_credential(&self, cred: impl Borrow<Credential>) -> Result<(), CredentialError> {
        let cred = cred.borrow();
        let path = self.credential_file();

        std::fs::create_dir_all(self.dir.as_path()).map_err(|err| {
            CredentialError::PersistCredential {
                io_err: err,
                path: path.clone(),
            }
        })?;

        let mut file = std::fs::File::create(&path)
            .map_err(|err| CredentialError::PersistCredential { io_err: err, path })?;

        serde_json::to_writer(&mut file, cred).map_err(CredentialError::Serialize)
    }

    /// Load credential from filesystem.
    /// This is blocking operation.
    pub fn load_credential(&self) -> Result<Unverified<Credential>, CredentialError> {
        let path = self.credential_file();

        let mut file = std::fs::File::open(&path)
            .map_err(|err| CredentialError::Open { io_err: err, path })?;

        serde_json::from_reader::<_, Credential>(&mut file)
            .map_err(CredentialError::Deserialize)
            .map(Unverified::from)
    }

    /// Remove all cache files
    pub(crate) fn clean(&self) -> io::Result<()> {
        // User can specify any directory as the cache
        // so instead of deleting the entire directory with `remove_dir_all`, delete files individually.
        match std::fs::remove_file(self.credential_file()) {
            Ok(()) => Ok(()),
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => Ok(()),
                _ => Err(err),
            },
        }
    }

    fn credential_file(&self) -> PathBuf {
        self.dir.join(config::cache::CREDENTIAL_FILE)
    }
}

// impl Default for Cache {
//     fn default() -> Self {
//         Self::new(config::cache::dir())
//     }
// }

#[cfg(test)]
mod tests {

    use crate::auth::Credential;

    use super::*;

    #[test]
    fn persist_then_load_credential() {
        let tmp = temp_dir();
        let cache = Cache::new(tmp);
        let cred = Credential::Github {
            access_token: "rust is fun".into(),
        };
        assert!(cache.persist_credential(&cred).is_ok());

        let loaded = cache.load_credential().unwrap();
        assert_eq!(loaded, Unverified::from(cred),);
    }

    fn temp_dir() -> PathBuf {
        tempfile::TempDir::new().unwrap().into_path()
    }
}
