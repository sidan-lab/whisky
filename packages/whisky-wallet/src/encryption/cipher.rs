use aes_gcm::{aead::AeadMut, Aes256Gcm, KeyInit, Nonce}; // AES-GCM encryption
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2_hmac; // PBKDF2 key derivation
use rand::RngCore; // Random number generation
use serde_json::json;
use sha2::Sha256;
use whisky_common::WError; // New base64 encoding

const IV_LENGTH: usize = 16;

pub fn encrypt_with_cipher(
    data: &str,
    key: &str,
    initialization_vector_size: Option<usize>,
) -> Result<String, WError> {
    // Validate the initialization vector size
    let initialization_vector_size = initialization_vector_size.unwrap_or(IV_LENGTH);

    // Generate a random salt for PBKDF2 key derivation
    let mut salt = vec![0u8; initialization_vector_size];
    rand::thread_rng().fill_bytes(&mut salt);

    let mut derived_key = vec![0u8; 32]; // AES-256 requires a 256-bit key (32 bytes)

    // PBKDF2 key derivation (HMAC-SHA-256)
    pbkdf2_hmac::<Sha256>(key.as_bytes(), &salt, 100_000, &mut derived_key);

    // Initialize AES-GCM cipher
    let mut cipher = Aes256Gcm::new_from_slice(&derived_key).map_err(WError::from_err(
        "encrypt_with_cipher - Aes256Gcm::new_from_slice",
    ))?;

    // Generate a random IV
    let mut iv = vec![0u8; initialization_vector_size];
    rand::thread_rng().fill_bytes(&mut iv);
    let nonce = Nonce::from_slice(&iv); // AES-GCM requires a 12-byte nonce

    // Encrypt the data
    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(WError::from_err("encrypt_with_cipher - cipher.encrypt"))?;

    // Return the encrypted data as a JSON-like string (base64 encoding)
    let iv_base64 = general_purpose::STANDARD.encode(&iv);
    let salt_base64 = general_purpose::STANDARD.encode(&salt);
    let ciphertext_base64 = general_purpose::STANDARD.encode(&ciphertext);

    let result = json!({
        "iv": iv_base64,
        "salt": salt_base64,
        "ciphertext": ciphertext_base64,
    });

    Ok(result.to_string())
}

pub fn decrypt_with_cipher(encrypted_data_json: &str, key: &str) -> Result<String, WError> {
    // Parse the encrypted data from JSON
    let encrypted_data: serde_json::Value = serde_json::from_str(encrypted_data_json).map_err(
        WError::from_err("decrypt_with_cipher - JSON parsing failed"),
    )?;

    let iv_base64 = encrypted_data["iv"]
        .as_str()
        .ok_or_else(WError::from_opt("decrypt_with_cipher", "Missing IV"))?;
    let ciphertext_base64 = encrypted_data["ciphertext"]
        .as_str()
        .ok_or_else(WError::from_opt(
            "decrypt_with_cipher",
            "Missing ciphertext",
        ))?;

    // Decode the IV and ciphertext from base64
    let iv = general_purpose::STANDARD
        .decode(iv_base64)
        .map_err(WError::from_err(
            "decrypt_with_cipher - Base64 decode of IV failed",
        ))?;
    let ciphertext = general_purpose::STANDARD
        .decode(ciphertext_base64)
        .map_err(WError::from_err(
            "decrypt_with_cipher - Base64 decode of ciphertext failed",
        ))?;

    // Handle salt - support both new format (with salt) and legacy format (without salt)
    let salt = if let Some(salt_base64) = encrypted_data["salt"].as_str() {
        // New format: use the provided salt
        if !salt_base64.is_empty() {
            general_purpose::STANDARD
                .decode(salt_base64)
                .map_err(WError::from_err(
                    "decrypt_with_cipher - Base64 decode of salt failed",
                ))?
        } else {
            // Empty salt string: use zero-filled salt of IV length for backward compatibility
            vec![0u8; iv.len()]
        }
    } else {
        // Legacy format: use zero-filled salt of IV length for backward compatibility
        vec![0u8; iv.len()]
    };

    // Derive a cryptographic key from the input key using PBKDF2 and SHA-256
    // Matches frontend: 100,000 iterations, SHA-256, 256-bit key
    let mut derived_key = vec![0u8; 32]; // AES-256 requires a 256-bit key (32 bytes)

    // PBKDF2 key derivation (HMAC-SHA-256)
    pbkdf2_hmac::<Sha256>(key.as_bytes(), &salt, 100_000, &mut derived_key);

    // Initialize AES-GCM cipher for decryption
    let mut cipher = Aes256Gcm::new_from_slice(&derived_key).map_err(WError::from_err(
        "decrypt_with_cipher - Aes256Gcm::new_from_slice",
    ))?;

    // Create a nonce from the IV
    let nonce = Nonce::from_slice(&iv);

    // Decrypt the data
    let decrypted_data = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(WError::from_err(
            "decrypt_with_cipher - Decryption failed (incorrect password or corrupted data)",
        ))?;

    // Convert the decrypted data back to a string
    let decrypted_str = String::from_utf8(decrypted_data).map_err(WError::from_err(
        "decrypt_with_cipher - Failed to convert decrypted data to UTF-8",
    ))?;

    Ok(decrypted_str)
}
