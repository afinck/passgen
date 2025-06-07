use arboard::Clipboard; // Importiert die Wayland-spezifische Clipboard-Bibliothek
use clap::Parser; //
use rand::{Rng, thread_rng}; // Importiert die Thread-spezifische Zufallszahlengenerator-Bibliothek

#[derive(Parser)]
#[command(version, about="Generates a random 12 character simple password", long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(
        short = 'n', 
        long = "numbers", 
        help= "number of characters", 
        default_value_t = 12
    )]
    characters: u8,
    #[arg(
        short = 'x', 
        long = "complex", 
        help= "add special characters", 
        default_value_t = false
    )]
    complex: bool,
    #[arg(
        short = 'c', 
        long = "copy", 
        help= "copy to clipboard", 
        default_value_t = false
    )]
    copy: bool
}

fn generate_password(characters: u8, complex: bool) -> String {
    // Diese Funktion generiert einen zufälligen Passwort-String mit der angegebenen Anzahl von Zeichen und optionaler Komplexität
    let mut password = String::new();
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let mut chars = Vec::new();

    chars
        .extend_from_slice(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-_+!=?");

    if complex {
        chars.extend_from_slice("!§$%&/()=?*+#-_.:,;<>|@".as_bytes());
    }
    
    if chars.is_empty() {
        chars.extend_from_slice(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-_+!=?");
    }

    for _ in 0..characters {
        let index = rng.gen_range(0..chars.len());
        password.push(chars[index] as char);
    }
    password
}

fn copy_to_clipboard(password: String) {
    // Diese Funktion kopiert den Inhalt in die Zwischenablage und gibt eine Nachricht aus
    let mut clipboard = Clipboard::new().unwrap();
    clipboard
        .set_text(password)
        .expect("Fehler beim Kopieren in die Zwischenablage");
    let content = clipboard.get_text().expect("Fehler beim Lesen aus der Zwischenablage");
    println!("copied to clipboard: {}", content);
}

fn main() {
    // Diese Funktion ist der Einstiegspunkt des Programms
    let cli = Cli::parse();
    let password = generate_password(cli.characters, cli.complex);
    if cli.copy {
        copy_to_clipboard(password.clone());   
    } else {
        println!("{}", password.clone());
    }
}