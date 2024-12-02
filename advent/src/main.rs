use decrypt_cookies::{Browser, ChromiumBuilder};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize ChromiumBuilder to decrypt cookies
    let chromium = ChromiumBuilder::new(Browser::Chromium)
        .build()
        .await?;
    let all_cookies = chromium.get_cookies_all().await?;

    // Manually build the cookie string from specific cookies
    let mut cookie_string = String::new();
    for cookie in all_cookies {
        match cookie.name.as_str() {
            "_ga" | "session" | "_gid" | "_ga_MHSNPJKWC7" => {
                if !cookie_string.is_empty() {
                    cookie_string.push_str("; ");
                }
                cookie_string.push_str(&format!("{}={}", cookie.name, cookie.value));
            }
            _ => {}
        }
    }

    if cookie_string.is_empty() {
        eprintln!("No matching cookies found. Please log in to the website first.");
        return Ok(());
    }

    // Set custom headers
    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"));
    headers.insert("accept-language", HeaderValue::from_static("en-US,en;q=0.9"));
    headers.insert("cache-control", HeaderValue::from_static("no-cache"));
    headers.insert("cookie", HeaderValue::from_str(&cookie_string)?); // Set the cookies
    headers.insert("pragma", HeaderValue::from_static("no-cache"));
    headers.insert("sec-fetch-dest", HeaderValue::from_static("document"));
    headers.insert("sec-fetch-mode", HeaderValue::from_static("navigate"));
    headers.insert("sec-fetch-site", HeaderValue::from_static("none"));
    headers.insert("sec-fetch-user", HeaderValue::from_static("?1"));
    headers.insert("upgrade-insecure-requests", HeaderValue::from_static("1"));
    headers.insert("sec-ch-ua", HeaderValue::from_static("\"Google Chrome\";v=\"131\", \"Chromium\";v=\"131\", \"Not_A Brand\";v=\"24\""));
    headers.insert("sec-ch-ua-mobile", HeaderValue::from_static("?1"));
    headers.insert("sec-ch-ua-platform", HeaderValue::from_static("\"Android\""));
    headers.insert(
        "user-agent",
        HeaderValue::from_static(
            "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36",
        ),
    );

    // Create a reqwest Client with default headers
    let client = Client::builder()
        .default_headers(headers)
        .build()?;

    // Define the target URL
    let url = "https://adventofcode.com/2024/day/1/input";

    // Make the request
    let response = client.get(url).send().await?;

    // Check and print the response
    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response:\n{}", body);
    } else {
        eprintln!("Failed to fetch data. Status: {}", response.status());
    }

    Ok(())
}

