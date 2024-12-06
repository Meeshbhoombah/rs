use aes_gcm::{Aes256Gcm, Key, Nonce}; // Aes256Gcm
use aes_gcm::aead::{Aead, KeyInit};
use base64::decode;
use rusqlite::{params, Connection};
use std::error::Error;
use std::fs;
use std::process::Command;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Retrieves the AES encryption key from Chrome's "Local State" file
fn get_aes_key() -> Result<Vec<u8>, Box<dyn Error>> {
    let local_state_path = dirs::home_dir()
        .unwrap()
        .join("Library/Application Support/Google/Chrome/Local State");
    let local_state: Value = serde_json::from_reader(fs::File::open(local_state_path)?)?;

    let encrypted_key = local_state["os_crypt"]["encrypted_key"]
        .as_str()
        .ok_or("Failed to read encrypted key")?;
    let encrypted_key = decode(encrypted_key)?;
    let encrypted_key = &encrypted_key[5..]; // Remove "DPAPI" prefix

    let key = Command::new("security")
        .arg("decrypt")
        .arg("-k")
        .arg("-w")
        .arg("-s")
        .arg(std::str::from_utf8(encrypted_key).unwrap_or_default()) // Convert bytes to string
        .output()?;

    if key.status.success() {
        Ok(key.stdout)
    } else {
        Err(format!(
            "Failed to retrieve AES key: {}",
            String::from_utf8_lossy(&key.stderr)
        )
        .into())
    }
}

/// Decrypts Chrome cookies using AES-GCM
fn decrypt_cookie(encrypted_value: &[u8], key: &[u8]) -> Result<String, Box<dyn Error>> {
    if encrypted_value.starts_with(b"v10") {
        let nonce = &encrypted_value[3..15];
        let ciphertext = &encrypted_value[15..encrypted_value.len() - 16];
        let tag = &encrypted_value[encrypted_value.len() - 16..];

        let mut complete_ciphertext = Vec::from(ciphertext);
        complete_ciphertext.extend_from_slice(tag);

        let cipher_key = Key::<Aes256Gcm>::from_slice(key); // Explicit key size
        let cipher = Aes256Gcm::new(cipher_key);
        let plaintext = cipher
            .decrypt(Nonce::from_slice(nonce), complete_ciphertext.as_ref())
            .map_err(|e| format!("Decryption failed: {}", e))?; // Convert error to a string

        Ok(String::from_utf8(plaintext)?)
    } else {
        Err("Unsupported cookie encryption format".into())
    }
}

/// Retrieves and decrypts Chrome cookies (_ga, _ga_MHSNPJKWC7, _gid, session)
fn get_cookies(db_path: &str, aes_key: &[u8]) -> Result<(String, String, String, String), Box<dyn Error>> {
    let conn = Connection::open(db_path)?;
    let mut stmt = conn.prepare("SELECT name, encrypted_value FROM cookies")?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, String>(0)?, row.get::<_, Vec<u8>>(1)?)))?;

    let mut _ga = String::new();
    let mut _ga_mhsnpjkwc7 = String::new();
    let mut _gid = String::new();
    let mut session = String::new();

    for row in rows {
        let (name, encrypted_value) = row?;
        match name.as_str() {
            "_ga" => _ga = decrypt_cookie(&encrypted_value, aes_key)?,
            "_ga_MHSNPJKWC7" => _ga_mhsnpjkwc7 = decrypt_cookie(&encrypted_value, aes_key)?,
            "_gid" => _gid = decrypt_cookie(&encrypted_value, aes_key)?,
            "session" => session = decrypt_cookie(&encrypted_value, aes_key)?,
            _ => (),
        }
    }

    Ok((_ga, _ga_mhsnpjkwc7, _gid, session))
}

/// Makes the GET request to Advent of Code using the decrypted cookies
fn fetch_puzzle_input(cookies: (&str, &str, &str, &str)) -> Result<String, Box<dyn Error>> {
    let url = "https://adventofcode.com/2024/day/1/input";
    let client = Client::new();

    let cookie_header = format!(
        "_ga={}; _ga_MHSNPJKWC7={}; _gid={}; session={}",
        cookies.0, cookies.1, cookies.2, cookies.3
    );

    let response = client
        .get(url)
        .header(
            "accept",
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
        )
        .header("accept-language", "en-US,en;q=0.9")
        .header("cache-control", "no-cache")
        .header("cookie", cookie_header)
        .header("pragma", "no-cache")
        .header("sec-ch-ua", r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#)
        .header("sec-ch-ua-mobile", "?1")
        .header("sec-ch-ua-platform", r#""Android""#)
        .header("sec-fetch-dest", "document")
        .header("sec-fetch-mode", "navigate")
        .header("sec-fetch-site", "none")
        .header("sec-fetch-user", "?1")
        .header("upgrade-insecure-requests", "1")
        .header(
            "user-agent",
            "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Mobile Safari/537.36",
        )
        .send()?;

    let status = response.status();
    let body = response.text()?;

    if status.is_success() {
        Ok(body)
    } else {
        Err(format!(
            "Failed to fetch input: {}. Server response: {}",
            status, body
        )
        .into())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let db_path = "/Users/meeshbhoombah/Library/Application Support/Google/Chrome/Default/Cookies";
    let aes_key = get_aes_key()?;
    println!("AES Key (hex): {}", hex::encode(&aes_key));

    let cookies = get_cookies(db_path, &aes_key)?;
    println!("Decrypted Cookies: {:?}", cookies);

    let puzzle_input = fetch_puzzle_input((&cookies.0, &cookies.1, &cookies.2, &cookies.3))?;
    println!("Puzzle Input:\n{}", puzzle_input);

    Ok(())
}
