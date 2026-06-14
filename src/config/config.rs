use camino::Utf8PathBuf;

// 設定値を持つ構造体
#[allow(dead_code)]
pub struct Config {
    storage_name: String,
    // 設定フォルダを置く場所のパス。設定フォルダのパスは`<storage_path>/<storage_name>` というパスになる
    storage_dir: Utf8PathBuf,
}
#[allow(dead_code)]
impl Config {
    pub fn new(storage_name: String, storage_dir: Utf8PathBuf) -> Self {
        Self {
            storage_name,
            storage_dir,
        }
    }

    pub fn get_storage_path(&self) -> Utf8PathBuf {
        self.storage_dir.join(&self.storage_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_path() {
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = Utf8PathBuf::from_path_buf(tempdir.path().to_path_buf()).unwrap();
        let config = Config::new(".rscuff".to_string(), tempdir_path.clone());
        assert_eq!(config.get_storage_path(), tempdir_path.join(".rscuff"));
    }
}
