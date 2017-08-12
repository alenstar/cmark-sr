use std::env;
use std::path::PathBuf;
pub fn relative_path() -> Option<&str> {
    let relative_path = PathBuf::from(filename);
    let mut absolute_path = std::env::current_dir().unwrap();
    absolute_path.push(relative_path);
    absolute_path.to_str()
}
