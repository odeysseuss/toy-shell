use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[cfg(unix)]
pub fn is_exec<P: AsRef<Path>>(path: P) -> bool {
    use std::os::unix::fs::PermissionsExt;

    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
pub fn is_exec<P: AsRef<Path>>(path: P) -> bool {
    true
}

pub fn check_ext_cmd(cmd: &str) -> (bool, Option<PathBuf>) {
    if let Some(paths) = env::var_os("PATH") {
        for dir in env::split_paths(&paths) {
            let full_path: PathBuf = dir.join(cmd);
            if full_path.exists() && full_path.is_file() && is_exec(&full_path) {
                return (true, Some(full_path));
            }
        }
    }
    return (false, None);
}
