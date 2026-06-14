use crate::config::Config;

#[allow(dead_code)]
pub struct ConfigStorage {
    config: Config,
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use camino::Utf8PathBuf;
    use std::fs;
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
        fs::create_dir(config.get_storage_path()).unwrap();
        let storage = ConfigStorage::new(config);

        assert!(storage.exists());
    }
}
