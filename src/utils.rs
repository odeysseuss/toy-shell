use std::{
    env, fs,
    io::Write,
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

pub fn write_to_file(output: Vec<u8>, filename: String) {
    let mut file = match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(filename.clone())
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Could not write to {}: {}", filename.clone(), e);
            return;
        }
    };

    if let Err(e) = file.write_all(&output) {
        eprintln!("Could not write to {}: {}", filename.clone(), e);
    }
}
