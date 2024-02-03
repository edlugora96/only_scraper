use std::env;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};

pub fn find_executable() -> io::Result<PathBuf> {
    if let Ok(path) = env::var("CHROME_SCRIPT") {
        if Path::new(&path).exists() {
            print!("stored, env var");
            return Ok(PathBuf::from(path));
        }
    }

    let path_var = match env::var_os("PATH") {
        Some(paths) => paths,
        None => {
            return Err(io::Error::new(
                ErrorKind::NotFound,
                "PATH environment variable not found".to_string(),
            ))
        }
    };

    for app in possible_executable_names().iter() {
        for path in env::split_paths(&path_var) {
            let exe_path = path.join(app);
            if exe_path.is_file() && is_executable(&exe_path) {
                return Ok(exe_path);
            }
        }
    }

    Err(io::Error::new(
        ErrorKind::NotFound,
        "Could not find a Chrome executable".to_string(),
    ))
}

fn possible_executable_names() -> Vec<&'static str> {
    vec![
        "google-chrome",
        "google-chrome-stable",
        "google-chrome-beta",
        "google-chrome-dev",
        "google-chrome-unstable",
        "chromium",
        "chromium-browser",
        "microsoft-edge-stable",
        "microsoft-edge-beta",
        "microsoft-edge-dev",
        "chrome",
        "chrome-browser",
        "msedge",
        "microsoft-edge",
    ]
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = path.metadata() {
            let permissions = metadata.permissions();
            permissions.mode() & 0o111 != 0
        } else {
            false
        }
    }
    #[cfg(not(unix))]
    {
        path.is_file()
    }
}