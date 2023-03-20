use std::env;
use std::path::{Path, PathBuf};

///
/// ```
/// use std::env;
/// use test_helpers::fixture::get_git_root;
/// env::set_current_dir(env::var("CARGO_MANIFEST_DIR").unwrap());
/// let r = get_git_root().unwrap();
/// assert!(r.ends_with("test_helpers"));
/// env::set_current_dir(r.join("test_utils/src/fixtures"));
/// let r = get_git_root().unwrap();
/// assert!(r.ends_with("test_helpers"));
/// ```
pub fn get_git_root() -> Option<PathBuf> {
    let mut dir = env::current_dir().ok()?;
    while dir.exists() {
        let p = Path::new(&dir).join(".git");
        if p.exists() {
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
/// assert!(r.ends_with("test_helpersV"));
/// env::set_current_dir(r.join("test_utils/src/fixtures"));
/// let r = path_from_git_root("./foo_bar/").unwrap();
/// assert!(r.ends_with("test_helpers/foo_bar/"));
/// ```
pub fn path_from_git_root(path: &str) -> Option<PathBuf> {
    Some(get_git_root()?.join(path))
}
