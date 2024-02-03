//! ## Overview 📖
//!
//! Only Scraper is a minimalist, high-performance web scraping tool written in Rust 🦀, focusing solely on the task of scraping web pages. It stands out by not using any external libraries, thereby providing a streamlined and efficient solution for developers. This project is tailored for those who seek a simple and direct approach to web scraping, without the overhead of additional functionalities that are often bundled in other libraries.
//!
//! ## Why Only Scraper? 🤔
//!
//! With a plethora of web scraping tools available, many of which offer extensive features like end-to-end testing and browser automation, Only Scraper cuts through the noise by offering a no-frills, focused solution. By eliminating the dependency on external libraries, Only Scraper ensures that your scraping tasks are not bogged down by unnecessary complexities, making it a swift and straightforward option for retrieving web page data.
//!
//! ## Features 🌟
//!
//! - **Zero External Dependencies** 📦: Maximizes efficiency and minimizes setup time by relying solely on Rust's standard library.
//! - **Optimized for Speed** ⚡: Takes full advantage of Rust's performance and safety to offer quick and reliable web scraping.
//! - **Singular Focus** 🔍: Provides just what you need for scraping web pages, nothing more, nothing less.
//!
//! ### Usage 📝
//!
//! Only Scraper is designed to be as simple as possible, with only one method needed to perform web scraping. Here's a minimal example to demonstrate its usage:
//!
//! ```rust
//! use only_scraper;
//! use std::io::Result;
//!
//! fn main() -> Result<()> {
//!     let html = only_scraper::scrape("https://example.com")?;
//!     println!("{}", html);
//!     Ok(())
//! }
//! ```
//!
//! This code snippet fetches the HTML content of the specified URL and prints it out, showcasing the simplicity and efficiency of Only Scraper.
use std::io;

mod chrome;
use chrome::{application, executable, scraper};

/// The `scrape` method automates web scraping by taking a URL as input and returning the webpage's HTML content. Initially, it checks for Chrome's installation, installing the latest version if necessary, to ensure a modern browser environment is available. The method uses Chrome in headless mode to efficiently load and scrape the webpage, including dynamically generated content. This process is fully automated, providing an easy and resource-efficient way to obtain the complete HTML source code of a webpage for further processing.
pub fn scrape(url: String) -> io::Result<String> {
    let executable_path;
    match application::download_and_install_chrome() {
        Ok(path) => executable_path = path,
        Err(_) => executable_path = executable::find_executable()?,
    }

    let output = scraper::scrape(executable_path, url)?;

    Ok(output)
}
