use ansi_term::Colour;

pub fn warn(a: &str) {
    println!(
        "{} {}",
        Colour::Yellow.bold().paint("⚠"),
        Colour::Yellow.paint(a)
    );
}

pub fn info(a: &str) {
    println!(
        "{} {}",
        Colour::Blue.bold().paint("ℹ"),
        Colour::Blue.paint(a)
    );
}

pub fn list(a: &str) {
    println!(
        "  {} {}",
        Colour::White.bold().paint("•"),
        Colour::White.paint(a)
    );
}
