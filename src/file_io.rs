use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use crate::password_entry::PasswordEntry;

pub fn save_password(entry: &PasswordEntry, file_path: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)?;

    let entry_json = serde_json::to_string(entry).unwrap();
    writeln!(file, "{}", entry_json)?;

    Ok(())
}

pub fn load_passwords(file_path: &str) -> Vec<PasswordEntry> {
    let mut file = File::open(file_path).unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
        .lines()
        .map(|line| serde_json::from_str::<PasswordEntry>(line).unwrap())
        .collect()
}
