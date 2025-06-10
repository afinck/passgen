use arboard::Clipboard; // Imports the Wayland-specific clipboard library
use clap::Parser;
use rand::{thread_rng, Rng}; // Imports the thread-specific random number generator library

#[derive(Parser)]
#[command(
    version,
    about = env!("CARGO_PKG_DESCRIPTION").lines().next().unwrap(), // First line of description
    long_about = env!("CARGO_PKG_DESCRIPTION") // Full description
)]
struct Cli {
    #[arg(
        short = 'n',
        long = "numbers",
        help = "number of characters",
        default_value_t = 12
    )]
    characters: u8,
    #[arg(
        short = 'x',
        long = "complex",
        help = "add special characters",
        default_value_t = false
    )]
    complex: bool,
    #[arg(
        short = 'c',
        long = "copy",
        help = "copy to clipboard",
        default_value_t = false
    )]
    copy: bool,
}

fn generate_password(characters: u8, complex: bool) -> String {
    // This function generates a random password string with the specified number of characters and optional complexity
    let mut password = String::new();
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let mut chars = Vec::new();

    chars
        .extend_from_slice(b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-_+!=?");

    if complex {
        chars.extend_from_slice("!§$%&/()=?*+#-_.:,;<>|@".as_bytes());
    }
    if chars.is_empty() {
        chars.extend_from_slice(
            b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890-_+!=?",
        );
    }

    for _ in 0..characters {
        let index = rng.gen_range(0..chars.len());
        password.push(chars[index] as char);
    }
    password
}

fn copy_to_clipboard(password: String) {
    // This function copies the content to the clipboard and prints a message
    let mut clipboard = Clipboard::new().unwrap();
    clipboard
        .set_text(password)
        .expect("Error copying to clipboard");
    let content = clipboard.get_text().expect("Error reading from clipboard");
    println!("copied to clipboard: {}", content);
}

fn main() {
    // This function is the entry point of the program
    let cli = Cli::parse();
    let password = generate_password(cli.characters, cli.complex);
    if cli.copy {
        copy_to_clipboard(password.clone());
    } else {
        println!("{}", password.clone());
    }
}
