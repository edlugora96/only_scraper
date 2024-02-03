use std::io::{self, Result};
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str;
use std::{env, fs};

const APP_NAME: &str = "only_scrape";
const DEFAULT_HOST: &str = "https://storage.googleapis.com";

pub fn download_and_install_chrome() -> io::Result<PathBuf> {
    let install_dir = guess_install_dir()?;
    let (platform, archive_name, chrome_path) = platform_details();
    let app_path = install_dir.join(archive_name).join(&chrome_path);

    if app_path.exists() {
        return Ok(app_path);
    }

    let revision = latest_revision(platform)?;

    let url = format!(
        "{DEFAULT_HOST}/chromium-browser-snapshots/{platform}/{revision}/{archive_name}.zip"
    );
    let output_path = install_dir.join(format!("{platform}-{revision}.zip"));

    fs::create_dir_all(&install_dir)?;

    download_file(&url, &output_path)?;
    unzip_file(&output_path, &install_dir)?;
    fs::remove_file(&output_path).unwrap_or_else(|_| println!("Failed to delete zip"));

    Ok(app_path)
}

fn guess_install_dir() -> Result<PathBuf> {
    env::var("HOME")
        .or_else(|_| env::var("USERPROFILE"))
        .map(PathBuf::from)
        .map(|dir| dir.join(format!(".{}", APP_NAME)))
        .map_err(|_| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))
}

fn platform_details() -> (&'static str, &'static str, PathBuf) {
    let (platform, archive_name) = match (cfg!(target_os = "linux"), cfg!(target_arch = "aarch64"))
    {
        (true, _) => ("Linux_x64", "chrome-linux"),
        (_, true) => ("Mac_arm", "chrome-mac"),
        _ if cfg!(target_os = "macos") => ("Mac", "chrome-mac"),
        _ if cfg!(target_os = "windows") => ("Win_x64", "chrome-win"),
        _ => panic!("Unsupported platform"),
    };
    let chrome_path = match platform {
        "Linux_x64" => PathBuf::from("chrome"),
        "Win_x64" => PathBuf::from("chrome.exe"),
        "Mac" | "Mac_arm" => PathBuf::from("Chromium.app/Contents/MacOS/Chromium"),
        _ => panic!("Unsupported platform"),
    };

    (platform, archive_name, chrome_path)
}

fn latest_revision(platform: &str) -> Result<String> {
    let url = format!("{DEFAULT_HOST}/chromium-browser-snapshots/{platform}/LAST_CHANGE");
    let output = Command::new("curl").arg(&url).output()?;

    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8 sequence"))?;
        Ok(stdout.trim().to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to get latest revision",
        ))
    }
}

fn download_file(url: &str, output_path: &PathBuf) -> io::Result<()> {
    Command::new("curl")
        .arg("-o")
        .arg(output_path)
        .arg(url)
        .status()
        .and_then(|status| {
            if status.success() {
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to download file",
                ))
            }
        })
}

fn unzip_file(zip_path: &Path, destination: &Path) -> io::Result<()> {
    // Determine the command based on the operating system.
    #[cfg(target_os = "windows")]
    let mut command = {
        let mut cmd = Command::new("tar");
        cmd.args(&[
            "-xf",
            zip_path.to_str().unwrap(),
            "-C",
            destination.to_str().unwrap(),
        ]);
        cmd
    };

    #[cfg(not(target_os = "windows"))]
    let mut command = {
        let mut cmd = Command::new("unzip");
        cmd.args(&[
            "-o",
            zip_path.to_str().unwrap(),
            "-d",
            destination.to_str().unwrap(),
        ]);
        cmd
    };

    // Execute the command.
    let mut child = command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()?;

    // Wait for the command to complete.
    let output = child.wait()?;

    if output.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Failed to extract ZIP file",
        ))
    }
}
