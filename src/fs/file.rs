use crate::fs::path::Path;

// ファイル構造体
#[allow(dead_code)]
pub struct File {
    path: Path,
}

#[allow(dead_code)]
impl File {
    pub fn new(path: Path) -> Self {
        Self { path }
    }

    // 文字列からFileを作成
    pub fn from(path: &str) -> Self {
        Self::new(Path::from(path))
    }

    // ファイル名を返す
    pub fn get_file_name(&self) -> Option<&str> {
        self.path.basename()
    }
}

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
