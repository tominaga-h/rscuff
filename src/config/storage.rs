use crate::config::Config;
use std::fs;
use std::io::{Error, ErrorKind, Result};

#[allow(dead_code)]
pub struct ConfigStorage {
    pub config: Config,
}

#[allow(dead_code)]
impl ConfigStorage {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // 設定フォルダが存在するかどうかを返す
    pub fn exists(&self) -> bool {
        self.config.get_storage_path().exists()
    }

    // 設定フォルダを作成する。設定フォルダが既に存在する場合はエラーを返す
    pub fn create(&self) -> Result<()> {
        if self.exists() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "storage already exists",
            ));
        }

        // 設定フォルダの作成
        fs::create_dir(self.config.get_storage_path())?;

        // 設定フォルダ配下のgroupsフォルダの作成
        fs::create_dir(self.config.get_groups_path())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_exists_flase() {
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = Utf8PathBuf::from_path_buf(tempdir.path().to_path_buf()).unwrap();
        let config = Config::new("test_rscuff".to_string(), tempdir_path);
        let storage = ConfigStorage::new(config);
        assert!(!storage.exists());
    }

    #[test]
    fn test_exists_true() {
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = Utf8PathBuf::from_path_buf(tempdir.path().to_path_buf()).unwrap();
        let config = Config::new("test_rscuff".to_string(), tempdir_path);
        let storage = ConfigStorage::new(config);
        storage.create().unwrap();

        assert!(storage.exists());
    }

    #[test]
    fn test_create() {
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = Utf8PathBuf::from_path_buf(tempdir.path().to_path_buf()).unwrap();
        let config = Config::new("test_rscuff".to_string(), tempdir_path);
        let storage = ConfigStorage::new(config);

        assert!(storage.create().is_ok());
    }

    #[test]
    fn test_create_error() {
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = Utf8PathBuf::from_path_buf(tempdir.path().to_path_buf()).unwrap();
        let config = Config::new("test_rscuff".to_string(), tempdir_path);
        let storage = ConfigStorage::new(config);

        storage.create().unwrap();
        assert!(storage.create().is_err());
    }
}
