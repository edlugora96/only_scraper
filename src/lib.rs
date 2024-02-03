use std::io;

mod chrome;
use chrome::{application, executable, scraper};

pub fn scrape(url: String) -> io::Result<String> {
    let executable_path;
    match application::download_and_install_chrome() {
        Ok(path) => executable_path = path,
        Err(_) => executable_path = executable::find_executable()?,
    }

    let output = scraper::scrape(executable_path, url)?;

    Ok(output)
}
