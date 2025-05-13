mod browser;
mod find_chrome;
mod scraper;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Setting up browser...");

    // Initialize browser
    let (mut browser, browser_handle) = browser::setup_browser().await?;

    // Create a new page
    let page = browser.new_page("about:blank").await?;

    // Scrape content
    let content_result = scraper::scrape_website(&page, "https://www.motorsport.com").await;

    // Close browser regardless of whether scraping succeeded
    println!("Closing browser...");
    match browser.close().await {
        Ok(_) => println!("Browser closed successfully"),
        Err(e) => eprintln!("Error closing browser: {e}"),
    }

    // Ensure the browser handler task completes
    match browser_handle.await {
        Ok(_) => (),
        Err(e) => eprintln!("Error waiting for browser handler: {e}"),
    }

    // Handle scraping result after browser is closed
    match content_result {
        Ok((title, content)) => {
            println!("Title: {title}");
            println!(
                "Content Preview (first 200 chars): {}",
                if content.len() > 200 {
                    format!("{}...", &content[..200])
                } else {
                    content.clone()
                }
            );

            // Save content to files
            utils::save_scraped_content(&title, &content).await?;
        }
        Err(e) => return Err(e),
    }

    println!("Scraping completed successfully");
    Ok(())
}
