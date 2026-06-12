use camino::Utf8PathBuf;

// パス構造体
#[allow(dead_code)]
pub struct Path {
    path: Utf8PathBuf,
}

#[allow(dead_code)]
impl Path {
    pub fn new(path: Utf8PathBuf) -> Self {
        Self { path }
    }

    // 文字列からPathを作成する
    pub fn from(path: &str) -> Self {
        Self::new(camino::Utf8PathBuf::from(path))
    }

    // パスを取得する
    pub fn get(&self) -> &str {
        self.path.as_str()
    }

    // パスを結合する
    pub fn join(&self, other: &str) -> Utf8PathBuf {
        self.path.join(other)
    }

    // basename=パスのファイル名を取得する
    // ディレクトリの場合はNoneを返す
    pub fn basename(&self) -> Option<&str> {
        self.path.file_name()
    }

    // dirname=パスのディレクトリ名を取得する
    pub fn dirname(&self) -> Option<&str> {
        match self.path.parent() {
            Some(parent) => Some(parent.as_str()),
            None => None,
        }
    }
}

#[test]
fn test_from() {
    let path = Path::from(".");
    assert_eq!(path.get(), ".");

    let joined = path.join("foo");
    assert_eq!(joined.as_str(), "./foo");
}

#[test]
fn test_join() {
    let path = Path::from(".");
    let joined = path.join("foo");
    assert_eq!(joined.as_str(), "./foo");
}

#[test]
fn test_basename() {
    let path = Path::from("./foo/test.txt");
    assert_eq!(path.basename(), Some("test.txt"));

    let path = Path::from("./foo/");
    assert_eq!(path.basename(), Some("foo"));
}

#[test]
fn test_dirname() {
    let path = Path::from("./foo/test.txt");
    assert_eq!(path.dirname(), Some("./foo"));

    let path = Path::from("./foo/bar/test.txt");
    assert_eq!(path.dirname(), Some("./foo/bar"));

    let path = Path::from("/test.txt");
    assert_eq!(path.dirname(), Some("/"));

    let path = Path::from("/");
    assert_eq!(path.dirname(), None);
}
