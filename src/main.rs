mod strs;
mod util;

use ansi_term::Colour::White;
use random_string::generate;
use std::{env, io, io::Write, os::unix::fs::PermissionsExt, process::exit};
use util::{help, tut};

fn main() {
    let args: Vec<String> = env::args().collect();
    let dir = format!("{}/.local/passman", env::var("HOME").unwrap());
    if args.len() <= 1 {
        let arg: String = "".to_string();
        passgen(arg);
        exit(0);
    }
    if args[1] == "--tut" {
        tut();
        exit(0);
    } else if args[1] == "--clean" {
        let _ = std::fs::remove_dir_all(dir);
    } else if args[1] == "--help" {
        help();
        exit(0);
    } else {
        let arg: String = args[1].clone();
        passgen(arg);
        exit(0);
    }
}

fn passgen(arg: String) {
    const CHARSET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}\\|;:'\",<.>/?`~";

    let dir = format!("{}/.local/passman", env::var("HOME").unwrap());
    let prefix_path = format!("{}/{}", dir, "prefix");
    let cipher_path = format!("{}/{}", dir, "cipher");

    // making sure our directory exists
    if !std::path::Path::new(&dir).exists() {
        std::fs::create_dir_all(&dir).unwrap();
        tut();
    };

    // prefix generation (if not present)
    if !std::path::Path::new(&prefix_path).exists() {
        // random alphanumeric string of length 7
        let prefix: String = generate(7, &CHARSET);
        std::fs::write(&prefix_path, prefix).unwrap();
        // making sure only the owner can only read the file
        std::fs::set_permissions(&prefix_path, std::fs::Permissions::from_mode(0o400)).unwrap();
    };

    // cipher generation (if not present)
    if !std::path::Path::new(&cipher_path).exists() {
        // random alphanumeric string of length 26
        let cipher: String = generate(512, &CHARSET);
        std::fs::write(&cipher_path, cipher).unwrap();
        // making sure only the owner can only read the file
        std::fs::set_permissions(&cipher_path, std::fs::Permissions::from_mode(0o400)).unwrap();
    };

    // read the values from the files
    let prefix = std::fs::read_to_string(&prefix_path).unwrap();
    let secret = White.bold().paint("your secret");
    let cipher = std::fs::read_to_string(&cipher_path).unwrap();

    // set the app to arg if exists, otherwise prompt the user for it
    let app: String = match arg.as_ref() {
        "" => {
            print!("App Name: ");
            io::stdout().flush().ok().unwrap();
            let mut app: String = String::new();
            let _ = std::io::stdin().read_line(&mut app);
            app
        }
        _ => arg,
    };

    // mapp the app name against the cipher
    let app_ciphered: String = app
        .trim()
        .chars()
        .map(|x| {
            let mut y = x;
            if x.is_alphabetic() {
                if x.is_uppercase() {
                    y = cipher.chars().nth(x as usize - 65).unwrap();
                } else {
                    y = cipher.chars().nth(x as usize - 97).unwrap();
                }
            }
            y
        })
        .collect();

    // print to the user
    println!(
        "\nYour password for {} is:\n\n{} + {} + {}\n",
        app.trim(),
        prefix,
        secret,
        app_ciphered
    );
}
