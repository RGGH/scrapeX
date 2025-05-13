use std::path::PathBuf;
use std::process::Command;

pub fn find_chrome_or_chromium() -> Option<PathBuf> {
    // Check common locations based on operating system
    if cfg!(target_os = "windows") {
        find_chrome_windows()
    } else if cfg!(target_os = "macos") {
        find_chrome_macos()
    } else {
        find_chrome_linux()
    }
}

fn find_chrome_windows() -> Option<PathBuf> {
    // Common Windows Chrome/Chromium locations
    let common_locations = [
        r"C:\Program Files\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
    ];

    for location in &common_locations {
        let path = PathBuf::from(location);
        if path.exists() {
            return Some(path);
        }
    }

    // Try searching in registry (simplified version)
    if let Ok(output) = Command::new("cmd")
        .args([
            "/c",
            r#"reg query "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\chrome.exe" /ve"#,
        ])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines() {
            if line.contains("REG_SZ") {
                if let Some(path_str) = line.split("REG_SZ").nth(1) {
                    let chrome_path = PathBuf::from(path_str.trim());
                    if chrome_path.exists() {
                        return Some(chrome_path);
                    }
                }
            }
        }
    }

    None
}

fn find_chrome_macos() -> Option<PathBuf> {
    // Common macOS Chrome/Chromium locations
    let common_locations = [
        "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
        "/Applications/Chromium.app/Contents/MacOS/Chromium",
        "/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge",
    ];

    for location in &common_locations {
        let path = PathBuf::from(location);
        if path.exists() {
            return Some(path);
        }
    }

    None
}

fn find_chrome_linux() -> Option<PathBuf> {
    // Common Linux Chrome/Chromium locations
    let common_locations = [
        "/usr/bin/google-chrome",
        "/usr/bin/chromium",
        "/usr/bin/chromium-browser",
        "/snap/bin/chromium",
        "/usr/bin/microsoft-edge",
    ];

    for location in &common_locations {
        let path = PathBuf::from(location);
        if path.exists() {
            return Some(path);
        }
    }

    // Try to find using which command
    let browsers = [
        "google-chrome",
        "chromium",
        "chromium-browser",
        "microsoft-edge",
    ];
    for browser in &browsers {
        if let Ok(output) = Command::new("which").arg(browser).output() {
            if output.status.success() {
                let path_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let path = PathBuf::from(path_str);
                if path.exists() {
                    return Some(path);
                }
            }
        }
    }

    None
}
