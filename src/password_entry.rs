use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PasswordEntry {
    pub website: String,
    pub username: String,
    pub encrypted_password: Vec<u8>,
}

impl PasswordEntry {
    pub fn new(website: String, username: String, encrypted_password: Vec<u8>) -> Self {
        Self {
            website,
            username,
            encrypted_password,
        }
    }
}
