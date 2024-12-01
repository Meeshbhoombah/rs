use std::env;
use std::path::Path;
use std::process::Command;
use std::string::FromUtf8Error;
use std::error::Error;

use aes::Aes256;
use cipher::{KeyIvInit, StreamCipher};
use rusqlite::Connection;
use keyring::Entry;


fn get_encryption_key3() -> Result<Vec<u8>, Box<dyn Error>> {
    // Define the command to run
    let output = Command::new("security")
        .arg("find-generic-password")
        .arg("-w") // Return only the password
        .arg("-s") // Search by service
        .arg("Chrome Safe Storage")
        .output()?; // Execute the command and capture the output

    if output.status.success() {
        // Convert the output to a UTF-8 string and then to bytes
        let key = String::from_utf8(output.stdout)?.trim().as_bytes().to_vec();
        Ok(pad_or_truncate_key(key, 32)) // Ensure the key is exactly 32 bytes
    } else {
        // Capture the error message from stderr
        let error_message = String::from_utf8(output.stderr)?;
        Err(format!("Failed to retrieve encryption key: {}", error_message).into())
    }
}

/// Ensures the key is the correct length for AES-256 (32 bytes)
fn pad_or_truncate_key(mut key: Vec<u8>, length: usize) -> Vec<u8> {
    if key.len() < length {
        // Pad with zeros if the key is too short
        key.resize(length, 0);
    } else if key.len() > length {
        // Truncate if the key is too long
        key.truncate(length);
    }
    key
}

/// Fetches the Chrome Safe Storage encryption key using macOS security command
fn get_encryption_key2() -> Result<String, Box<dyn Error>> {
    // Define the command to run
    let output = Command::new("security")
        .arg("find-generic-password")
        .arg("-w") // Return only the password
        .arg("-s") // Search by service
        .arg("Chrome Safe Storage")
        .output()?; // Execute the command and capture the output

    if output.status.success() {
        // Convert the output to a UTF-8 string
        Ok(String::from_utf8(output.stdout)?.trim().to_string())
    } else {
        // Capture the error message from stderr
        let error_message = String::from_utf8(output.stderr)?;
        Err(format!("Failed to retrieve encryption key: {}", error_message).into())
    }
}


type Aes256Ctr = ctr::Ctr64BE<Aes256>; // Define the AES-256-CTR cipher type

fn get_encryption_key() -> Vec<u8> {
    let service = "Chrome Safe Storage";
    let user = whoami::username(); // Get the current username
    
    match Entry::new(service, &user) {
        Ok(entry) => match entry.get_password() {
            Ok(password) => password.as_bytes().to_vec(),
            Err(e) => {
                eprintln!("Failed to retrieve encryption key from keychain: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Failed to access keyring entry: {}", e);
            std::process::exit(1);
        }
    }
}


fn decrypt_cookie4(encrypted_value: &[u8], key: &[u8]) -> String {
    if encrypted_value.starts_with(b"v10") {
        // Chrome's cookie encryption uses AES-256-CTR with a 12-byte nonce
        let nonce_size = 12;
        let (nonce, ciphertext) = encrypted_value[3..].split_at(nonce_size);

        // Extend the 12-byte nonce to 16 bytes by padding with zeros
        let mut nonce_padded = [0u8; 16];
        nonce_padded[..nonce_size].copy_from_slice(nonce);

        // Initialize the cipher with the key and padded nonce
        let mut cipher = Aes256Ctr::new(key.into(), (&nonce_padded).into());
        let mut decrypted = ciphertext.to_vec();
        cipher.apply_keystream(&mut decrypted); // Decrypt in-place

        // Debug: Print the raw decrypted bytes
        println!("Decrypted raw bytes (hex): {}", hex::encode(&decrypted));

        // Attempt to decode as UTF-8
        match String::from_utf8(decrypted.clone()) {
            Ok(decoded) => decoded, // Successfully decoded as UTF-8
            Err(_) => {
                // Attempt Base64 decoding
                if let Ok(base64_decoded) = base64::decode(&decrypted) {
                    if let Ok(decoded_str) = String::from_utf8(base64_decoded.clone()) {
                        return decoded_str;
                    }
                }

                // Attempt JSON parsing (if relevant)
                if let Ok(decoded_json) = serde_json::from_slice::<serde_json::Value>(&decrypted) {
                    return decoded_json.to_string();
                }

                // If all else fails, return hex representation
                format!("Decrypted raw bytes (hex): {}", hex::encode(&decrypted))
            }
        }
    } else {
        // Handle legacy cookies
        match String::from_utf8(encrypted_value.to_vec()) {
            Ok(decoded) => decoded,
            Err(_) => format!("Legacy cookie raw bytes (hex): {}", hex::encode(encrypted_value)),
        }
    }
}

fn decrypt_cookie3(encrypted_value: &[u8], key: &[u8]) -> String {
    if encrypted_value.starts_with(b"v10") {
        // Chrome's cookie encryption uses AES-256-CTR with a 12-byte nonce
        let nonce_size = 12;
        let (nonce, ciphertext) = encrypted_value[3..].split_at(nonce_size);

        // Extend the 12-byte nonce to 16 bytes by padding with zeros
        let mut nonce_padded = [0u8; 16];
        nonce_padded[..nonce_size].copy_from_slice(nonce);

        // Initialize the cipher with the key and padded nonce
        let mut cipher = Aes256Ctr::new(key.into(), (&nonce_padded).into());
        let mut decrypted = ciphertext.to_vec();
        cipher.apply_keystream(&mut decrypted); // Decrypt in-place

        // Attempt to decode as UTF-8
        match String::from_utf8(decrypted) {
            Ok(decoded) => decoded,
            Err(err) => {
                eprintln!("Failed to decode UTF-8: {:?}", err);
                format!(
                    "Decrypted raw bytes (hex): {}",
                    hex::encode(err.into_bytes())
                )
            }
        }
    } else {
        match String::from_utf8(encrypted_value.to_vec()) {
            Ok(decoded) => decoded,
            Err(err) => {
                eprintln!("Failed to decode legacy cookie as UTF-8: {:?}", err);
                format!(
                    "Legacy cookie raw bytes (hex): {}",
                    hex::encode(err.into_bytes())
                )
            }
        }
    }
}

fn decrypt_cookie2(encrypted_value: &[u8], key: &[u8]) -> String {
    if encrypted_value.starts_with(b"v10") {
        // Chrome's cookie encryption uses AES-256-CTR with a 12-byte nonce
        let nonce_size = 12;
        let (nonce, ciphertext) = encrypted_value[3..].split_at(nonce_size);

        // Extend the 12-byte nonce to 16 bytes by padding with zeros
        let mut nonce_padded = [0u8; 16];
        nonce_padded[..nonce_size].copy_from_slice(nonce);

        // Initialize the cipher with the key and padded nonce
        let mut cipher = Aes256Ctr::new(key.into(), (&nonce_padded).into());
        let mut decrypted = ciphertext.to_vec();
        cipher.apply_keystream(&mut decrypted); // Decrypt in-place

        String::from_utf8(decrypted).expect("Failed to decode UTF-8")
    } else {
        String::from_utf8(encrypted_value.to_vec()).expect("Failed to decode legacy cookie")
    }
}

fn decrypt_cookie(encrypted_value: &[u8], key: &[u8]) -> String {
    if encrypted_value.starts_with(b"v10") {
        // Chrome's cookie encryption uses AES-256-CTR with a 12-byte nonce
        let nonce_size = 12;
        let (nonce, ciphertext) = encrypted_value[3..].split_at(nonce_size);

        // Initialize the cipher with the key and nonce
        let mut cipher = Aes256Ctr::new(key.into(), nonce.into());
        let mut decrypted = ciphertext.to_vec();
        cipher.apply_keystream(&mut decrypted); // Decrypt in-place

        String::from_utf8(decrypted).expect("Failed to decode UTF-8")
    } else {
        String::from_utf8(encrypted_value.to_vec()).expect("Failed to decode legacy cookie")
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    // Path to Chrome's Cookies SQLite database
    let db_path = Path::new("/Users/Meeshbhoombah/Library/Application Support/Google/Chrome/Default/Cookies");

    // Open the SQLite database
    let conn = Connection::open(db_path).expect("Failed to open SQLite database");

    // Query for cookies
    let mut stmt = conn
        .prepare("SELECT host_key, name, encrypted_value FROM cookies WHERE host_key LIKE '%adventofcode.com%'")
        .expect("Failed to prepare SQL query");

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,   // host_key
                row.get::<_, String>(1)?,   // name
                row.get::<_, Vec<u8>>(2)?,  // encrypted_value
            ))
        })
        .expect("Failed to execute SQL query");

    // Get the encryption key
    let key = get_encryption_key3().unwrap();
    /*
    let key = get_encryption_key2().unwrap();
    println!("{:?}", key);
    */
    // let key = get_encryption_key();

    // Decrypt and print the cookies
    for row in rows {
        let (host_key, name, encrypted_value) = row.expect("Failed to parse row");
        let decrypted_value = decrypt_cookie4(&encrypted_value, &key);
        println!("Host: {}, Cookie: {} = {}", host_key, name, decrypted_value);
    }
}
