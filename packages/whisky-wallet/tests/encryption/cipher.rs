// use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
// use aes::Aes128;
// use base64::{decode, encode};
// use rand::rngs::OsRng;
// use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
// use serde::{Deserialize, Serialize};

// const AES_BLOCK_SIZE: usize = 16;

// #[derive(Serialize, Deserialize)]
// struct EncryptedData {
//     iv: String,
//     ciphertext: String,
// }

// // Symmetric encryption using AES
// fn encrypt_with_cipher(data: &str, key: &str) -> EncryptedData {
//     let key = GenericArray::from_slice(key.as_bytes());
//     let cipher = Aes128::new(key);

//     let mut block = GenericArray::clone_from_slice(&data.as_bytes()[..AES_BLOCK_SIZE]);
//     cipher.encrypt_block(&mut block);

//     EncryptedData {
//         iv: encode(&[0u8; AES_BLOCK_SIZE]), // Dummy IV for simplicity
//         ciphertext: encode(block),
//     }
// }

// fn decrypt_with_cipher(encrypted_data: &EncryptedData, key: &str) -> String {
//     let key = GenericArray::from_slice(key.as_bytes());
//     let cipher = Aes128::new(key);

//     let mut block = GenericArray::clone_from_slice(&decode(&encrypted_data.ciphertext).unwrap());
//     cipher.decrypt_block(&mut block);

//     String::from_utf8(block.to_vec()).unwrap()
// }

// // Asymmetric encryption using RSA
// fn generate_key_pair() -> (RsaPublicKey, RsaPrivateKey) {
//     let mut rng = OsRng;
//     let private_key = RsaPrivateKey::new(&mut rng, 2048).expect("Failed to generate private key");
//     let public_key = RsaPublicKey::from(&private_key);
//     (public_key, private_key)
// }

// fn encrypt_with_public_key(public_key: &RsaPublicKey, data: &str) -> String {
//     let mut rng = OsRng;
//     let encrypted = public_key
//         .encrypt(
//             &mut rng,
//             PaddingScheme::new_pkcs1v15_encrypt(),
//             data.as_bytes(),
//         )
//         .expect("Failed to encrypt");
//     encode(encrypted)
// }

// fn decrypt_with_private_key(private_key: &RsaPrivateKey, encrypted_data: &str) -> String {
//     let decrypted = private_key
//         .decrypt(
//             PaddingScheme::new_pkcs1v15_encrypt(),
//             &decode(encrypted_data).unwrap(),
//         )
//         .expect("Failed to decrypt");
//     String::from_utf8(decrypted).unwrap()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_encrypt_and_decrypt_with_cipher() {
//         let data = "solution solution solution solution";
//         let key = "0123456789012345"; // 16-byte key for AES-128

//         let encrypted_data = encrypt_with_cipher(data, key);
//         let decrypted_data = decrypt_with_cipher(&encrypted_data, key);

//         assert_eq!(data, decrypted_data);
//     }

//     #[test]
//     fn test_generate_encrypt_decrypt_with_keypair() {
//         let data = "solution solution solution solution";

//         let (public_key, private_key) = generate_key_pair();

//         let encrypted_data = encrypt_with_public_key(&public_key, data);
//         let decrypted_data = decrypt_with_private_key(&private_key, &encrypted_data);

//         assert_eq!(data, decrypted_data);
//     }
// }
