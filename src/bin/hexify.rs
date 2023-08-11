use clap::{Parser, arg, command, ArgAction};
use hex;

use hexify::read_file;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // Optional name to operate on
    #[arg(short, long)]
    name: Option<String>,

    // Insert spaces between bytes
    #[arg(short, long, action = ArgAction::SetTrue)]
    spaces: bool,

    /// Use uppercase hex characters
    #[arg(short, long, action = ArgAction::SetTrue)]
    upper: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Some(name) = cli.name.as_deref() {
        if let Some(buffer) = read_file(&name.to_string()) {
            let s: String;
            if cli.upper {
                s = hex::encode_upper(&buffer);
            }
            else {
                s = hex::encode(&buffer);
            }

            let mut result = String::new();

            if cli.spaces {
                let n = 2;  // each byte is two characters
                for (i, c) in s.chars().enumerate() {
                    result.push(c);
                    if (i + 1) % n == 0 {
                        result.push(' ');
                    }
                }
            }
            else {
                result = s;
            }

            println!("{}", result.trim_end());
        }
    }
}
