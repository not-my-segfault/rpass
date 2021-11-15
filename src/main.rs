mod strs;
mod util;

use ansi_term::Colour::White;
use random_string::generate;
use std::{env, io, io::Write, process::exit};
use util::{help, tut};

fn main() {
    let dir = format!("{}/{}", dirs::home_dir().unwrap().to_str().unwrap(), ".rpass");
    let args: Vec<String> = env::args().collect();
    // if no arguments are passed, passes it to the passgen() function to prompt for app name
    if args.len() <= 1 {
        let arg: String = "".to_string();
        passgen(arg);
        exit(0);
    }
    // shows the tutorial if the user passes the --tut flag
    if args[1] == "--tut" {
        tut();
        exit(0);
    // removes the .rpass directory if the user passes the --clean flag
    } else if args[1] == "--clean" {
        let _ = std::fs::remove_dir_all(dir);
    // displays the help output if the user passes the --help flag
    } else if args[1] == "--help" {
        help();
        exit(0);
    // catches any other arguments and passes them to the passgen() function
    } else {
        let arg: String = args[1].clone();
        passgen(arg);
        exit(0);
    }
}

fn passgen(arg: String) {
    let nchars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let schars = "!@#$%^&*()-_=+[]{}\\|;:'\",<.>/?`~";

    // adjust the .repeat(x) values to change the ration of normal characters to special characters
    let charset = format!("{}{}", nchars.repeat(2), schars.repeat(1));

    // this variable is set to the home directory of the user on windows
    let dir = format!("{}/{}", dirs::home_dir().unwrap().to_str().unwrap(), ".rpass");
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
        let prefix: String = generate(7, &charset);
        std::fs::write(&prefix_path, prefix).unwrap();
        // making sure only the owner can only read the file
        let mut permissions = std::fs::metadata(&prefix_path).unwrap().permissions();
        permissions.set_readonly(true);
    };

    // cipher generation (if not present)
    if !std::path::Path::new(&cipher_path).exists() {
        // random alphanumeric string of length 26
        let cipher: String = generate(512, &charset);
        std::fs::write(&cipher_path, cipher).unwrap();
        // making sure only the owner can only read the file
        let mut permissions = std::fs::metadata(&cipher_path).unwrap().permissions();
        permissions.set_readonly(true);
    };

    // back up the rpass directory
    let backup_path = format!("{}/{}", dirs::home_dir().unwrap().to_str().unwrap(), ".rpass_backup.tar");
    if !std::path::Path::new(&backup_path).exists() {
        let _ = std::process::Command::new("tar")
            .arg("-cf")
            .arg(&backup_path)
            .arg(&dir)
            .output()
            .expect("Something went wrong creating rpass backup. Check home directory permissions.");
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
