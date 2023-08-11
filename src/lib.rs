use std::fs;
use std::io::Read;

pub fn read_file(name: &String) -> Option<Vec<u8>> {
    match fs::File::open(&name) {
        Ok(mut f) => {
            let mut buffer = Vec::new();
            match f.read_to_end(&mut buffer) {
                Ok(_) => Some(buffer),
                Err(_) => None
            }
        },
        Err(_) => {
            eprintln!("Unable to open file '{}'", &name);
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
