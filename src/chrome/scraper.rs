use std::io::{self, ErrorKind, Read};
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn scrape(executable_path: PathBuf, url: String) -> io::Result<String> {
    let chrome_args = ["--headless", "--disable-gpu", "--dump-dom", &url];

    let mut output: String = String::new();

    let mut command = Command::new(executable_path);
    command.args(&chrome_args);
    command.stdout(Stdio::piped());

    let mut child = command.spawn()?;
    if let Some(ref mut stdout) = child.stdout {
        stdout.read_to_string(&mut output)?;
    }

    let status = child.wait()?;
    if !status.success() {
        return Err(io::Error::new(
            ErrorKind::Other,
            format!("Chrome failed with status: {}", status),
        ));
    }

    Ok(output)
}
