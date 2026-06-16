use camino::Utf8PathBuf;

pub mod storage;

use storage::ConfigStorage;

// 設定値を持つ構造体
#[allow(dead_code)]
pub struct Config {
    // 設定フォルダの名称
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

    // 設定フォルダのパスを返す
    pub fn get_storage_path(&self) -> Utf8PathBuf {
        self.storage_dir.join(&self.storage_name)
    }

    // 設定フォルダ内の `groups` フォルダのパスを返す
    pub fn get_groups_path(&self) -> Utf8PathBuf {
        self.get_storage_path().join("groups")
    }
}

pub fn get_storage() -> ConfigStorage {
    let home_dir = shellexpand::tilde("~");
    let config = Config::new(
        ".rscuff".to_string(),
        Utf8PathBuf::from(home_dir.to_string()),
    );
    ConfigStorage::new(config)
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

    #[test]
    fn test_groups_path() {
        let tempdir = TempDir::new().unwrap();
        let tempdir_path = Utf8PathBuf::from_path_buf(tempdir.path().to_path_buf()).unwrap();
        let config = Config::new(".rscuff".to_string(), tempdir_path.clone());
        assert_eq!(
            config.get_groups_path(),
            tempdir_path.join(".rscuff").join("groups")
        );
    }

    #[test]
    fn test_groups_path_string() {
        let homedir_path = Utf8PathBuf::from(shellexpand::tilde("~").to_string());
        let config = Config::new(".rscuff".to_string(), homedir_path.clone());
        assert_eq!(
            config.get_groups_path().to_string(),
            shellexpand::tilde("~/.rscuff/groups").to_string()
        );
    }
}
