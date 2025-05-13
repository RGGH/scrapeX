use crate::find_chrome::find_chrome_or_chromium;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;
use tokio::task::JoinHandle;

pub async fn setup_browser() -> Result<(Browser, JoinHandle<()>), Box<dyn std::error::Error>> {
    // Detect OS for platform-specific settings
    let is_windows = cfg!(target_os = "windows");

    // Check if Chrome/Chromium is installed
    let chrome_path = find_chrome_or_chromium();

    if let Some(path) = &chrome_path {
        println!("Found browser at: {}", path.display());
    } else {
        println!("Warning: Could not find Chrome or Chromium browser installed.");
        println!("You may need to install Chrome or Chromium for this application to work.");
        println!("Attempting to proceed with default browser discovery...");
    }

    // Create browser config with platform-specific settings
    let mut config_builder = BrowserConfig::builder().window_size(1280, 800).with_head(); // Makes browser visible for debugging

    // Add platform-specific options
    if is_windows {
        // Windows sometimes needs these extra flags
        config_builder = config_builder.args(["--disable-gpu", "--disable-web-security"]);
    } else {
        // Linux often needs no-sandbox in certain environments (like Docker)
        config_builder = config_builder.args(["--disable-web-security"]).no_sandbox();
    }

    // If we found a Chrome path, we can add it to the env log for debugging
    if let Some(path) = chrome_path {
        println!("Using browser at: {}", path.display());
        // Note: We can't set the path directly with this API,
        // but the BrowserConfig will try to find it automatically
    }

    // Build the final config
    let browser_config = config_builder.build()?;

    // Launch the browser with proper error handling
    let (browser, mut handler) = Browser::launch(browser_config).await?;

    // Spawn the browser handler task
    let browser_handle = tokio::spawn(async move {
        while let Some(_event) = handler.next().await {
            // You could log browser events here if needed
            // println!("Browser event: {:?}", event);
        }
    });

    Ok((browser, browser_handle))
}
