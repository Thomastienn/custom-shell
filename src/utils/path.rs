use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn is_executable(file_path: &Path) -> bool {
    file_path.exists() && file_path.metadata().unwrap().permissions().mode() & 0o111 != 0
}

pub fn find_executable(command: &str) -> Option<String> {
    if let Ok(path_str) = std::env::var("PATH") {
        for path in path_str.split(':') {
            let full_path = format!("{}/{}", path, command);
            let file_path = Path::new(&full_path);
            if is_executable(file_path) {
                return Some(full_path);
            }
        }
    }
    None
}
