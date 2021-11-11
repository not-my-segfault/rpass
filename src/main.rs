use rand::distributions::Alphanumeric;
use rand::Rng;
use std::env;
use text_io::read;

fn main() {
    let dir = format!("{}/.local/passman", env::var("HOME").unwrap());
    let prefix_path = format!("{}/{}", dir, "prefix");
    let secret_path = format!("{}/{}", dir, "secret");
    let cipher_path = format!("{}/{}", dir, "cipher");

    if !std::path::Path::new(&dir).exists() {
        std::fs::create_dir_all(&dir).unwrap();
    };

    if !std::path::Path::new(&prefix_path).exists() {
        let mut rng = rand::thread_rng();
        println!("Noting first run. Generating new prefix.");
        let prefix: String = (0..7)
            .map(|_| {
                let x = rng.sample(Alphanumeric);
                x as char
            })
            .collect();
        std::fs::write(&prefix_path, prefix).unwrap();
    };

    if !std::path::Path::new(&secret_path).exists() {
        println!("Noting first run. Please enter a memorable secret.");
        let secret: String = read!("{}\n");
        std::fs::write(&secret_path, secret).unwrap();
    };

    if !std::path::Path::new(&cipher_path).exists() {
        let mut rng = rand::thread_rng();
        println!("Noting first run. Generating new cipher.");
        let cipher: String = (0..26)
            .map(|_| {
                let x = rng.sample(Alphanumeric);
                x as char
            })
            .collect();
        std::fs::write(&cipher_path, cipher).unwrap();
    };

    let prefix = std::fs::read_to_string(&prefix_path).unwrap();
    let secret = std::fs::read_to_string(&secret_path).unwrap();
    let cipher = std::fs::read_to_string(&cipher_path).unwrap();

    println!("App Name: ");
    let app: String = read!();

    let app_ciphered = cipher
        .chars()
        .zip(app.chars())
        .map(|(c, a)| {
            let c = c as u8;
            let a = a as u8;
            let c = c ^ a;
            c as char
        })
        .collect::<String>();

    let pass = format!("{}{}{}", prefix, secret, app_ciphered);

    println!(
        "Your password for {} is: {} [ {} + {} + {} ]",
        app, pass, prefix, secret, app_ciphered
    );
}
