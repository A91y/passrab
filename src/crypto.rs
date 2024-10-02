use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub fn encrypt_password(plain_password: &str, key: &[u8; 32]) -> Vec<u8> {
    let iv = rand::thread_rng().gen::<[u8; 16]>();
    let cipher = Aes256Cbc::new_from_slices(key, &iv).unwrap();
    
    let mut buffer = plain_password.as_bytes().to_vec();
    buffer.resize(buffer.len() + 16 - (buffer.len() % 16), 0); // Pad buffer to a multiple of 16
    
    cipher.encrypt(&mut buffer, plain_password.len()).unwrap();
    [iv.to_vec(), buffer].concat()
}

pub fn decrypt_password(encrypted_password: &[u8], key: &[u8; 32]) -> String {
    let (iv, ciphertext) = encrypted_password.split_at(16);
    if let Ok(cipher) = Aes256Cbc::new_from_slices(key, iv) {
        let mut decrypted_data = ciphertext.to_vec();
        if cipher.decrypt(&mut decrypted_data).is_ok() {
            if let Ok(decrypted_string) = String::from_utf8(decrypted_data) {
                return decrypted_string.trim().to_string();
            }
        }
    }
    "".to_string()
}
