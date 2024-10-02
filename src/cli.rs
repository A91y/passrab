pub fn print_help() {
    println!("Please provide a command (add, get, list, delete, update, help)");
}

use std::io;

use crate::{
    crypto,
    file_io::{load_passwords, save_password},
    password_entry::PasswordEntry,
};

pub fn handle_add_password(key: &[u8; 32]) {
    let mut website = String::new();
    let mut username = String::new();

    println!("Enter website:");
    io::stdin()
        .read_line(&mut website)
        .expect("Failed to read line");
    let website = website.trim().to_string();

    println!("Enter username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");
    let username = username.trim().to_string();

    println!("Enter password:");
    let plain_password = rpassword::read_password().expect("Failed to read password");

    let encrypted_password = crypto::encrypt_password(&plain_password, key);
    let new_entry = PasswordEntry::new(website, username, encrypted_password);
    let _ = save_password(&new_entry, "passwords.json");

    println!("Password saved.");
}

pub fn handle_get_password(key: &[u8; 32]) {
    let mut website = String::new();
    println!("Enter website:");
    io::stdin()
        .read_line(&mut website)
        .expect("Failed to read line");
    let website = website.trim().to_string();
    let all_passwords = load_passwords("passwords.json");

    for entry in all_passwords {
        if entry.website == website {
            let decrypted_password = crypto::decrypt_password(&entry.encrypted_password, key);
            if decrypted_password.is_empty() {
                continue;
            }
            println!("Username: {}", entry.username);
            println!("Password for {}: {}", entry.website, decrypted_password);
        }
    }
}

pub fn handle_list_passwords() {
    let all_passwords = load_passwords("passwords.json");
    for entry in all_passwords {
        println!("Website: {}, Username: {}", entry.website, entry.username);
    }
}
