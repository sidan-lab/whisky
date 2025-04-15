#[cfg(test)]
mod tests {
    use whisky_wallet::*;

    #[test]
    fn test_decrypt_with_cipher() {
        let data = "solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution";
        let key = "01234567890123456789";

        let encrypted_data = "{\"iv\":\"/bs1AzciZ1bDqT5W\",\"ciphertext\":\"mh5pgH8ErqqH2KLLEBqqr8Pwm+mUuh9HhaAHslSD8ho6zk7mXccc9NUQAW8rb9UajCq8LYyANuiorjYD5N0hd2Lbe2n1x8AGRZrogyRKW6uhoFD1/FW6ofjgGP/kQRQSW2ZdJaDMbCxwYSdzxmaRunk6JRfybhfRU6kIxPMu41jhhRC3LbwZ+NnfBJFrg859hbuQgMQm8mqOUgOxcK8kKH54shOpGuLT4YBXhx33dZ//wT5VXrQ8kwIKttNk5h9MNKCacpRZSqU3pGlZ5oxucNEGos0IKTTXfbmwYx14uiERcXd32OP2\"}";
        let decrypted_data = decrypt_with_cipher(&encrypted_data, key).unwrap();

        assert_eq!(data, decrypted_data);
    }

    #[test]
    fn test_encrypt_and_decrypt_with_cipher() {
        let data = "solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution solution";
        let key = "01234567890123456789";

        let encrypted_data = encrypt_with_cipher(data, key, Some(12)).unwrap();
        let decrypted_data = decrypt_with_cipher(&encrypted_data, key).unwrap();

        assert_eq!(data, decrypted_data);
    }
}
