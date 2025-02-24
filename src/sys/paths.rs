use std::path::Path;

pub fn is_dir(string: &str) -> bool {
    Path::new(string).is_dir()
}
pub fn is_file(path: &str) -> bool {
    Path::new(path).is_file()
}
pub fn exists(path: &str) -> bool {
    Path::new(path).exists()
}
pub fn basename(path: &str) -> Option<String> {
    match Path::new(path).file_name() {
        Some(base) => Some(base.to_string_lossy().into_owned()),
        None => None,
    }
}
