mod cli;
mod crypto;
mod file_io;
mod password_entry;
mod utils;

use std::env;

use cli::{handle_add_password, handle_get_password, handle_list_passwords};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        cli::print_help();
        return;
    }

    let command = &args[1];

    let master_password = utils::get_master_password();
    let key = utils::derive_key(&master_password);
    
    match command.as_str() {
        "add" => handle_add_password(&key),
        "get" => handle_get_password(&key),
        "list" => handle_list_passwords(),
        "delete" => println!("Delete functionality is not implemented yet."),
        "help" => cli::print_help(),
        _ => {
            println!("Invalid command");
            cli::print_help();
        }
    }

    
}
