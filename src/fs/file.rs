use crate::fs::path::Path;
use std::fs;
use std::io;
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

    pub fn from_str(path: &str) -> Self {
        Self::new(Path::from_str(path))
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

    // ファイルを作成する。
    pub fn create(&self) -> io::Result<bool> {
        if self.exists() {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "File already exists",
            ));
        }
        let inner = fs::File::create(self.path.get());
        match inner {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_file_from_str() {
        let file = File::from_str("/foo/test.txt");
        assert_eq!(file.path.get(), "/foo/test.txt");
    }

    #[test]
    fn test_get_file_name() {
        let file = File::from_str("/foo/bar/test.txt");
        assert_eq!(file.get_file_name(), Some("test.txt"));
    }

    #[test]
    fn test_exists_true() {
        let tmp = NamedTempFile::new().unwrap();
        let file = File::from_str(tmp.path().to_str().unwrap_or_default());
        assert!(file.exists());
    }

    #[test]
    fn test_create_true() {
        // let tmpdir = TempDir::new().unwrap();
        // let file_path = Path::new(tmpdir.path()).join("test.txt");
        // let file = File::new(file_path);
    }
}
