use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::path::PathBuf;

pub struct PathUtils;

impl PathUtils {
    pub fn is_executable(file_path: &Path) -> bool {
        file_path.exists() && file_path.metadata().unwrap().permissions().mode() & 0o111 != 0
    }

    pub fn all_files() -> Vec<PathBuf> {
        let mut files = Vec::new();
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                files.push(entry.path());
            }
        }
        files
    }

    pub fn all_entries_rec_here() -> Vec<PathBuf> {
        Self::all_entries_rec(Path::new("."))
    }

    pub fn all_entries_rec(dir: &Path) -> Vec<PathBuf> {
        let mut entries = Vec::new();
        entries.push(dir.to_path_buf());
        if let Ok(dir_entries) = fs::read_dir(dir) {
            for entry in dir_entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    entries.extend(Self::all_entries_rec(&path));
                } else {
                    entries.push(path);
                }
            }
        }
        entries
    }

    pub fn all_executables_in_path() -> Vec<PathBuf> {
        let mut executables = Vec::new();
        if let Ok(path_str) = env::var("PATH") {
            for path in path_str.split(':') {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries.flatten() {
                        let file_path = entry.path();
                        if Self::is_executable(&file_path) {
                            executables.push(file_path);
                        }
                    }
                }
            }
        }
        executables
    }

    pub fn get_filename(path: &PathBuf) -> Option<String> {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
    }

    pub fn get_fullpath(path: &PathBuf) -> Option<String> {
        path.canonicalize()
            .ok()
            .and_then(|p| p.to_str().map(|s| s.to_string()))
    }

    pub fn get_relative_path(full_path: &PathBuf) -> Option<String> {
        let current_dir = env::current_dir().ok()?;
        full_path.strip_prefix(current_dir).ok().and_then(|p| p.to_str().map(|s| s.to_string()))
    }
}
