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

    #[test]
    fn test_decrypt_with_cipher_correct_password() {
        let encrypted_data = r#"{"iv":"XRAGv22SYgpZiGhy","salt":"5YowN2Txol1ejcvt9gJB1A==","ciphertext":"SUJcKVu5/yLVXvcVRI0xLTT+HN0j0JQc2YGL4uwmdErIAa4ZwTkfaKP3VNlclBeXoRfRqCRw9ioYZLSrZOsUlSKRDIGkrfHamZw3Nt+bTwWgzAecWmLOeU8Ks1ou6iQa1K9Yqt2+zJi6rDJfkEFEZJBOjC0iFnmeIMemYVD5UexqIkTlGZcKzwW57WU4HPKHpri/PhupcPRVpbZaNurCTB9tfnDLsr83zgHqSFILOdnSwvUaMA=="}"#;
        let password = "testing123456";

        let result = decrypt_with_cipher(encrypted_data, password);

        assert!(result.is_ok(), "expected no error, got: {:?}", result.err());
        let decrypted = result.unwrap();
        assert!(!decrypted.is_empty(), "expected non-empty result, got empty string");
    }

    #[test]
    fn test_decrypt_with_cipher_incorrect_password() {
        let encrypted_data = r#"{"iv":"XRAGv22SYgpZiGhy","salt":"5YowN2Txol1ejcvt9gJB1A==","ciphertext":"SUJcKVu5/yLVXvcVRI0xLTT+HN0j0JQc2YGL4uwmdErIAa4ZwTkfaKP3VNlclBeXoRfRqCRw9ioYZLSrZOsUlSKRDIGkrfHamZw3Nt+bTwWgzAecWmLOeU8Ks1ou6iQa1K9Yqt2+zJi6rDJfkEFEZJBOjC0iFnmeIMemYVD5UexqIkTlGZcKzwW57WU4HPKHpri/PhupcPRVpbZaNurCTB9tfnDLsr83zgHqSFILOdnSwvUaMA=="}"#;
        let password = "wrongPassword";

        let result = decrypt_with_cipher(encrypted_data, password);

        assert!(result.is_err(), "expected error with wrong password, got success");
    }
}
