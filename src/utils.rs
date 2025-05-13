use std::path::PathBuf;

pub async fn save_scraped_content(
    title: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Use a simple timestamp
    let timestamp = {
        use std::time::{SystemTime, UNIX_EPOCH};
        let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        duration.as_secs()
    };

    // Create output directory if it doesn't exist
    let output_dir = PathBuf::from("output");
    if !output_dir.exists() {
        println!("Creating output directory: {}", output_dir.display());
        std::fs::create_dir_all(&output_dir)?;
    }

    // Save the content to a file
    let sanitized_title = sanitize_filename(title);
    let content_filename = output_dir.join(format!("content_{sanitized_title}_{timestamp}.txt"));
    std::fs::write(&content_filename, content)?;
    println!("Full content saved to {}", content_filename.display());

    Ok(())
}

//pub async fn save_screenshot(page: &Page, title: &str) -> Result<(), Box<dyn std::error::Error>> {
    //// Use a simple timestamp
    //let timestamp = {
        //use std::time::{SystemTime, UNIX_EPOCH};
        //let duration = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        //duration.as_secs()
    //};

    //// Create output directory if it doesn't exist
    //let output_dir = PathBuf::from("output");
    //if !output_dir.exists() {
        //std::fs::create_dir_all(&output_dir)?;
    //}

    //// Take and save screenshot
    //let screenshot_data = scraper::take_screenshot(page).await?;
    //let sanitized_title = sanitize_filename(title);
    //let screenshot_path = output_dir.join(format!("screenshot_{sanitized_title}_{timestamp}.png"));
    //std::fs::write(&screenshot_path, &screenshot_data)?;
    //println!("Screenshot saved to {}", screenshot_path.display());

    //Ok(())
//}

fn sanitize_filename(filename: &str) -> String {
    // Replace invalid filename characters with underscores
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|', ' '];
    let mut sanitized = filename.to_string();
    for c in invalid_chars {
        sanitized = sanitized.replace(c, "_");
    }

    // Limit length to avoid overly long filenames
    if sanitized.len() > 50 {
        sanitized.truncate(50);
    }

    sanitized
}
