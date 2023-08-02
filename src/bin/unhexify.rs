use std::env;
use std::fs;
use std::io::prelude::*;

use hex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("usage: unhexify infile outfile");
        std::process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];
    match fs::read_to_string(input_file) {
        Ok(mut input) => if input.len() == 0 {
            return;
        }
        else {
            // hex::decode accepts both upper and lower case, also mixed
            // However, we need to strip all the spaces first.
            remove_whitespace(&mut input);

            match hex::decode(input) {
                Ok(buf) => {
                    let mut f = fs::File::create(&output_file).expect("to create file");
                    f.write_all(&buf).expect("to write to file");
                },
                Err(err) => eprintln!("{}", err)
            }
        },
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
