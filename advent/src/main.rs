use std::path::Path;
use std::fs::File;

use keyring::Entry;
use aes::Aes128;
use cipher::stream::Ctr128BE;
use rusqlite::{Connection};


fn get_encryption_key() -> Vec<u8> {
    let service = "Chrome Safe Storage";
    let user = whoami::username(); // Get the current username
    let entry = Entry::new(service, &user).expect("Failed to access keychain");
    let password = entry.get_password().expect("Failed to retrieve encryption key");
    password.as_bytes().to_vec()
}

fn decrypt_cookie(encrypted_value: &[u8], key: &[u8]) -> String {
    // Chrome's cookie encryption uses AES-128-CTR
    let nonce_size = 12; // First 12 bytes of the encrypted_value are the nonce
    let (nonce, ciphertext) = encrypted_value.split_at(nonce_size);

    // Create the AES-128-CTR cipher
    let cipher = Ctr128BE::<Aes128>::new(key.into(), nonce.into());
    let mut plaintext = ciphertext.to_vec();

    cipher.apply_keystream(&mut plaintext); // Decrypt in-place
    String::from_utf8(plaintext).expect("Failed to decode UTF-8")
}


fn main() -> Result<(), Box<dyn Error>> {


    // Path to Chrome's Cookies SQLite database
    let db_path = Path::new("/Users/<YourUsername>/Library/Application Support/Google/Chrome/Default/Cookies");

    // Open the SQLite database
    let conn = Connection::open(db_path).expect("Failed to open SQLite database");

    // Query for cookies
    let mut stmt = conn
        .prepare("SELECT host_key, name, encrypted_value FROM cookies WHERE host_key LIKE '%adventofcode.com%'")
        .expect("Failed to prepare SQL query");

    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?, // host_key
                row.get::<_, String>(1)?, // name
                row.get::<_, Vec<u8>>(2)?, // encrypted_value
            ))
        })
        .expect("Failed to execute SQL query");

    // Get the encryption key
    let key = get_encryption_key();

    // Decrypt and print the cookies
    for row in rows {
        let (host_key, name, encrypted_value) = row.expect("Failed to parse row");
        let decrypted_value = decrypt_cookie(&encrypted_value, &key);
        println!("Host: {}, Cookie: {} = {}", host_key, name, decrypted_value);
    }


}

