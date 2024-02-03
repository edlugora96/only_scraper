# Only Scraper 🚀

## Overview 📖

Only Scraper is a minimalist, high-performance web scraping tool written in Rust 🦀, focusing solely on the task of scraping web pages. It stands out by not using any external libraries, thereby providing a streamlined and efficient solution for developers. This project is tailored for those who seek a simple and direct approach to web scraping, without the overhead of additional functionalities that are often bundled in other libraries.

## Why Only Scraper? 🤔

With a plethora of web scraping tools available, many of which offer extensive features like end-to-end testing and browser automation, Only Scraper cuts through the noise by offering a no-frills, focused solution. By eliminating the dependency on external libraries, Only Scraper ensures that your scraping tasks are not bogged down by unnecessary complexities, making it a swift and straightforward option for retrieving web page data.

## Features 🌟

- **Zero External Dependencies** 📦: Maximizes efficiency and minimizes setup time by relying solely on Rust's standard library.
- **Optimized for Speed** ⚡: Takes full advantage of Rust's performance and safety to offer quick and reliable web scraping.
- **Singular Focus** 🔍: Provides just what you need for scraping web pages, nothing more, nothing less.

## Getting Started 🚀

### Installation 🛠️

To integrate Only Scraper into your Rust project, add the following to your `Cargo.toml` file:

```toml
[dependencies]
only_scraper = "0.1.0"
```

This will fetch the library directly from the GitHub repository and incorporate it into your project.

### Usage 📝

Only Scraper is designed to be as simple as possible, with only one method needed to perform web scraping. Here's a minimal example to demonstrate its usage:

```rust
use only_scraper;
use std::io::Result;

fn main() -> Result<()> {
    let html = only_scraper::scrape("https://example.com")?;
    println!("{}", html);
    Ok(())
}
```

This code snippet fetches the HTML content of the specified URL and prints it out, showcasing the simplicity and efficiency of Only Scraper.

## Contributing 🤝

We welcome contributions to make Only Scraper even better. Whether it's feature requests, bug reports, or code contributions, please feel free to reach out by opening an issue or a pull request on GitHub.

## License 📄

Only Scraper is released under the MIT License. For more details, please refer to the LICENSE file in the repository.