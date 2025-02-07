use clap::Parser;
use rand::prelude::*;
use rand::rng;

#[derive(Parser, Debug)]
#[command(name = "Grigode CLI")]
enum GrigodeCLI {
    Gpws {
        #[arg(short = 'L', long, default_value_t = 12)]
        length: usize,

        #[arg(short, long)]
        uppercase: bool,

        #[arg(short, long)]
        lowercase: bool,

        #[arg(short, long)]
        number: bool,

        #[arg(short, long)]
        symbol: bool,
    },
}

fn generate_password(
    length: usize,
    mut uppercase: bool,
    mut lowercase: bool,
    mut number: bool,
    mut symbol: bool,
) -> String {
    let mut charset = Vec::new();

    if !uppercase && !lowercase && !number && !symbol {
        uppercase = true;
        lowercase = true;
        number = true;
        symbol = true;
    }

    if uppercase {
        charset.extend(b'A'..=b'Z');
    }
    if lowercase {
        charset.extend(b'a'..=b'z');
    }
    if number {
        charset.extend(b'0'..=b'9');
    }
    if symbol {
        charset.extend("!@#$%^&*()-_=+".bytes());
    }

    let mut rng = rng();
    let password: String = (0..length)
        .map(|_| *charset.choose(&mut rng).unwrap() as char)
        .collect();

    password
}

fn main() {
    let cli = GrigodeCLI::parse();

    match cli {
        GrigodeCLI::Gpws {
            length,
            uppercase,
            lowercase,
            number,
            symbol,
        } => {
            let password = generate_password(length, uppercase, lowercase, number, symbol);
            println!("Generated password: {password}");
        }
    }
}
