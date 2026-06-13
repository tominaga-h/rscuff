use crate::fs::path::Path;
use std::fs;
// use std::fs::File as StdFile;

// ファイル構造体
pub struct File {
    path: Path,
}

#[allow(dead_code)]
impl File {
    pub fn new(path: Path) -> Self {
        Self { path }
    }

    pub fn from(path: &str) -> Self {
        Self::new(Path::from(path))
    }

    // ファイル名を返す
    pub fn get_file_name(&self) -> Option<&str> {
        self.path.basename()
    }

    // ファイルが存在するかどうかを返す
    pub fn exists(&self) -> bool {
        fs::exists(self.path.get()).unwrap_or_default()
        // note::
        //   - unwrap_or_defaultは設定できる引数がない
        //   - しかしRustではすべての型にデフォルト値が設定されていて、boolの場合はfalse
        //   - そのため unwrap_or_defaultを使用するとエラーの場合にfalseを返してくれる
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_from() {
        let file = File::from("/foo/test.txt");
        assert_eq!(file.path.get(), "/foo/test.txt");
    }

    #[test]
    fn test_get_file_name() {
        let file = File::from("/foo/bar/test.txt");
        assert_eq!(file.get_file_name(), Some("test.txt"));
    }

    #[test]
    fn test_exists_true() {
        let tmp = NamedTempFile::new().unwrap();
        let file = File::from(tmp.path().to_str().unwrap_or_default());
        assert!(file.exists());
    }
}
