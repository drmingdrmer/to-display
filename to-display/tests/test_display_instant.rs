#![cfg(feature = "std-time")]

use std::time::Instant;

use to_display::DisplayConfig;
use to_display::ToDisplay;

#[test]
fn test_display_instant() {
    let now = Instant::now();

    let display = now.display().to_string();
    println!("{}", display);
    // 20:31:18.645778
    let re = regex::Regex::new(r"^\d{2}:\d{2}:\d{2}\.\d{6}$").unwrap();
    assert!(re.is_match(&display),);

    let display = now.display().use_utc_time().to_string();
    println!("{}", display);
    // 20:31:18.645778
    let re = regex::Regex::new(r"^\d{2}:\d{2}:\d{2}\.\d{6}$").unwrap();
    assert!(re.is_match(&display),);

    let display = now.display().use_full_time().to_string();
    println!("{}", display);
    // 2024-12-28T23:37:31.646201Z+0800
    let re = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}\.\d{6}Z\+\d{4}$").unwrap();
    assert!(re.is_match(&display),);
}

#[cfg(feature = "tokio-time")]
#[test]
fn test_display_tokio_instant() {
    let now = tokio::time::Instant::now();

    let display = now.display().to_string();
    println!("{}", display);
    // 20:31:18.645778
    let re = regex::Regex::new(r"^\d{2}:\d{2}:\d{2}\.\d{6}$").unwrap();
    assert!(re.is_match(&display),);

    let display = now.display().use_utc_time().to_string();
    println!("{}", display);
    // 20:31:18.645778
    let re = regex::Regex::new(r"^\d{2}:\d{2}:\d{2}\.\d{6}$").unwrap();
    assert!(re.is_match(&display),);
}
