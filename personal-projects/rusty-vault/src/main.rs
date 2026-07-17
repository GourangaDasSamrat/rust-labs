use rusty_vault::{handle_add, handle_delete, handle_get, handle_list};
use std::env;

fn print_usage() {
    println!("Rusty Vault - A CLI Password Manager");
    println!();
    println!("Usage:");
    println!("  rusty-vault add <site> <username> <password>   Add a new password entry");
    println!("  rusty-vault list                               List all entries");
    println!("  rusty-vault get <site>                         Get password for a site");
    println!("  rusty-vault delete <site>                      Delete an entry");
    println!("  rusty-vault help                               Show this help message");
}

#[allow(clippy::indexing_slicing)]
fn main() {
    let args: Vec<String> = env::args().collect();

    // Skip the program name
    let command = if args.len() > 1 {
        &args[1]
    } else {
        print_usage();
        return;
    };

    let result = match command.as_str() {
        "add" => {
            if args.len() != 5 {
                eprintln!("Error: 'add' command requires 3 arguments: site, username, password");
                eprintln!("Usage: rusty-vault add <site> <username> <password>");
                return;
            }
            handle_add(&args[2], &args[3], &args[4])
        }

        "list" => handle_list(),

        "get" => {
            if args.len() != 3 {
                eprintln!("Error: 'get' command requires 1 argument: site");
                eprintln!("Usage: rusty-vault get <site>");
                return;
            }
            handle_get(&args[2])
        }

        "delete" => {
            if args.len() != 3 {
                eprintln!("Error: 'delete' command requires 1 argument: site");
                eprintln!("Usage: rusty-vault delete <site>");
                return;
            }
            handle_delete(&args[2])
        }

        "help" => {
            print_usage();
            Ok(())
        }

        _ => {
            eprintln!("Error: Unknown command '{command}'");
            eprintln!();
            print_usage();
            return;
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
