use std::env;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

///
/// ```
/// use std::env;
/// use test_helpers::fixture::get_git_root;
/// env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap());
/// let r = get_git_root().unwrap();
/// assert!(r.ends_with("rust-any2feed"));
/// env::set_current_dir(r.join("test_utils/src/fixtures"));
/// let r = get_git_root().unwrap();
/// assert!(r.ends_with("rust-any2feed"));
/// ```
pub fn get_git_root() -> Option<PathBuf> {
    let mut dir = env::current_dir().ok()?;
    while dir.exists() {
        let p = Path::new(&dir).join(".git");
        if p.exists() && p.is_dir() {
            return Some(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    None
}

///
/// ```
/// use std::env;
/// use test_helpers::fixture::{get_git_root, path_from_git_root};
/// env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap());
/// let r = get_git_root().unwrap();
/// assert!(r.ends_with("rust-any2feed"));
/// env::set_current_dir(r.join("test_utils/src/fixtures"));
/// let r = path_from_git_root("./foo_bar/").unwrap();
/// assert!(r.ends_with("rust-any2feed/foo_bar/"));
/// ```
pub fn path_from_git_root(path: &str) -> Option<PathBuf> {
    Some(get_git_root()?.join(path))
}
