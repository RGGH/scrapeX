use chromiumoxide::Page;
use chromiumoxide::cdp::browser_protocol::page::{CaptureScreenshotParams, NavigateParams};
use std::time::Duration;

pub async fn scrape_website(
    page: &Page,
    url: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    // Navigate to website
    println!("Navigating to {url}...");
    let navigate_params = NavigateParams::new(url);
    let _timeout = Duration::from_secs(30); // Not used directly but kept for reference

    match page.goto(navigate_params).await {
        Ok(_) => println!("Successfully loaded {url}"),
        Err(e) => {
            println!("Error loading {url}: {e:?}");
            return Err(e.into());
        }
    }

    // Wait for the page to be fully loaded
    page.wait_for_navigation().await?;

    // Get the page title using a more robust evaluation
    let title = page
        .evaluate(
            r#"
        (() => {
            return document.title || "No title found";
        })()
    "#,
        )
        .await?;

    // Extract the title string more safely
    let title_text = title
        .value()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "Could not extract title".to_string());

    // Extract main content more effectively
    let content_js = r#"
        (() => {
            // Try to find main content container first
            const mainContent = document.querySelector('main') || 
                               document.querySelector('#main-content') || 
                               document.querySelector('.main-content');
            
            if (mainContent) {
                return mainContent.innerText;
            }
            
            // Fall back to body text but try to exclude navigation and other non-content areas
            const bodyText = document.body.innerText;
            return bodyText ? bodyText.substring(0, 5000) + "..." : "No content found";
        })()
    "#;

    let content = page.evaluate(content_js).await?;

    // Extract the content text safely
    let content_text = content
        .value()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "Could not extract content".to_string());

    // Take a screenshot
    let _screenshot_data = take_screenshot(page).await?;

    // Return the extracted data and let calling code handle saving
    Ok((title_text, content_text))
}

pub async fn take_screenshot(page: &Page) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let screenshot_params = CaptureScreenshotParams::default();
    let screenshot_data = page.screenshot(screenshot_params).await?;
    Ok(screenshot_data)
}
