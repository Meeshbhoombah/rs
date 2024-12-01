use std::process::Command;
use std::error::Error;
use std::fs;
use std::path::Path;
use serde_json::Value;

fn get_aes_key() -> Result<Vec<u8>, Box<dyn Error>> {
    // Define the expected path to Chrome's Local State file
    let local_state_path = dirs::home_dir()
        .unwrap()
        .join("Library/Application Support/Google/Chrome/Local State");

    // Check if the file exists
    if !local_state_path.exists() {
        return Err(format!(
            "Local State file not found at {}",
            local_state_path.display()
        )
        .into());
    }

    // Read the file content
    let file_content = fs::read_to_string(&local_state_path)?;
    println!("Local State file content:\n{}", file_content); // Debugging

    // Parse JSON content
    let local_state: Value = serde_json::from_str(&file_content)?;

    // Ensure the `os_crypt` key exists
    if !local_state.get("os_crypt").is_some() {
        return Err("Missing 'os_crypt' key in Local State file".into());
    }

    // Ensure the `encrypted_key` key exists
    let encrypted_key = local_state["os_crypt"]["encrypted_key"]
        .as_str()
        .ok_or("Missing 'encrypted_key' in Local State")?;
    println!("Encrypted Key (Base64): {}", encrypted_key); // Debugging

    // Decode the base64-encoded key
    let encrypted_key = base64::decode(encrypted_key)?;
    println!("Encrypted Key (Decoded): {:?}", encrypted_key); // Debugging

    // Strip the "DPAPI" prefix (first 5 bytes)
    if encrypted_key.len() < 5 {
        return Err("Encrypted key is too short".into());
    }
    let encrypted_key = &encrypted_key[5..];
    println!("Encrypted Key (Stripped): {:?}", encrypted_key); // Debugging

    // Call the security command to decrypt the key
    let output = Command::new("security")
        .arg("decrypt")
        .arg("-k")
        .arg("-w")
        .arg("-s")
        .arg(std::str::from_utf8(encrypted_key)?)
        .output()?;

    // Check if the command was successful
    if output.status.success() {
        Ok(output.stdout)
    } else {
        Err(format!(
            "Failed to decrypt AES key: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into())
    }
}

fn main() {
    match get_aes_key() {
        Ok(aes_key) => println!("AES Key (hex): {}", hex::encode(aes_key)),
        Err(e) => eprintln!("Error retrieving AES key: {}", e),
    }
}

