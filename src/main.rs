use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use text_io::read;

fn main() {
    let dir = format!("{}/.local/passman", env::var("HOME").unwrap());
    let prefix_path = format!("{}/{}", dir, "prefix");
    let secret_path = format!("{}/{}", dir, "secret");
    let cipher_path = format!("{}/{}", dir, "cipher");

    // making sure our directory exists
    if !std::path::Path::new(&dir).exists() {
        std::fs::create_dir_all(&dir).unwrap();
    };

    // prefix generation (if not present)
    if !std::path::Path::new(&prefix_path).exists() {
        let mut rng = rand::thread_rng();
        println!("Noting first run. Generating new prefix.");
        // random alphanumeric string of length 7
        let prefix: String = (0..7)
            .map(|_| {
                let x = rng.sample(Alphanumeric);
                x as char
            })
            .collect();
        std::fs::write(&prefix_path, prefix).unwrap();
        // making sure only the owner can only read the file
        std::fs::set_permissions(&prefix_path, std::fs::Permissions::from_mode(0o400)).unwrap();
    };

    // collecting secret (if not present)
    if !std::path::Path::new(&secret_path).exists() {
        println!("Noting first run. Please enter a memorable secret.");
        let secret: String = read!("{}\n");
        std::fs::write(&secret_path, secret).unwrap();
        // making sure only the owner can only read the file
        std::fs::set_permissions(&secret_path, std::fs::Permissions::from_mode(0o400)).unwrap();
    };

    // cipher generation (if not present)
    if !std::path::Path::new(&cipher_path).exists() {
        // initialising rng
        let mut rng = rand::thread_rng();
        println!("Noting first run. Generating new cipher.");
        // random alphanumeric string of length 26
        let cipher: String = (0..26)
            .map(|_| {
                let x = rng.sample(Alphanumeric);
                x as char
            })
            .collect();
        std::fs::write(&cipher_path, cipher).unwrap();
        // making sure only the owner can only read the file
        std::fs::set_permissions(&cipher_path, std::fs::Permissions::from_mode(0o400)).unwrap();
    };

    // read the values from the files
    let prefix = std::fs::read_to_string(&prefix_path).unwrap();
    let secret = std::fs::read_to_string(&secret_path).unwrap();
    let cipher = std::fs::read_to_string(&cipher_path).unwrap();

    // collect the app name from the user
    println!("App Name: ");
    let app: String = read!();

    // mapp the app name against the cipher
    let app_ciphered: String = app
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
    
    // format the final password
    let pass = format!("{}{}{}", prefix, secret, app_ciphered);

    // print to the user
    println!(
        "Your password for {} is: {} [ {} + {} + {} ]",
        app, pass, prefix, secret, app_ciphered
    );
}
