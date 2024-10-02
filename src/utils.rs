use rpassword::read_password;
use sha2::{Digest, Sha256};
use std::io::{self, Write};

pub fn get_master_password() -> String {
    print!("Enter Master Password: ");
    io::stdout().flush().unwrap();

    let master_password = read_password().unwrap();
    master_password.trim().to_string()
}

pub fn derive_key(master_password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(master_password.as_bytes());
    let result = hasher.finalize();
    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    key
}
