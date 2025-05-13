# ScrapeX Web Scraper

A robust web scraping utility built with Rust that uses headless Chrome/Chromium to extract content from any website.

## Features

- 🌐 **Cross-platform compatibility** - Works on Windows and Linux with platform-specific optimizations
- 🔍 **Automatic browser detection** - Finds installed Chrome/Chromium browser instances
- 📄 **Content extraction** - Captures main article content from web pages
- 📸 **Screenshot capability** - Takes screenshots of pages for verification
- 💾 **Automatic file saving** - Stores content and screenshots with timestamps
- 🛠️ **Error handling** - Comprehensive error handling for browser operations

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- Chrome or Chromium browser installed on your system

## Installation

1. Clone this repository:
   ```
   git clone https://github.com/RGGH/scrapeX
   cd scrapeX
   ```

2. Build the project:
   ```
   cargo build --release
   ```

## Usage

Run the application with:

```
cargo run --release
```

The application will:
1. Automatically detect your Chrome/Chromium browser
2. Launch a browser instance with appropriate settings for your OS
3. Navigate to a chosen website (hardcoded at present)
4. Extract the page title and main content
5. Save the content to a text file in the `output` directory
6. Take a screenshot and save it to the `output` directory

## Output

All outputs are saved in the `output` directory with timestamp-based filenames:
- Content: `output/_content_[timestamp].txt`
- Screenshots: `output/_screenshot_[timestamp].png`

## Technical Details

This project utilizes:
- `chromiumoxide` - For browser automation
- `tokio` - For asynchronous runtime
- Custom Chrome/Chromium detection logic

## Configuration

The browser window is visible by default for debugging purposes. To make it headless, modify the `BrowserConfig::builder()` line in `main.rs` by removing the `.with_head()` call.

## Troubleshooting

- **Browser not found**: Ensure Chrome or Chromium is installed and accessible in the standard installation locations
- **Permission errors on Linux**: The application uses `--no-sandbox` flag on Linux to avoid permission issues in certain environments
- **Slow performance**: Increase the navigation timeout in the code if needed

## License

[MIT License](LICENSE)
