use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_path() -> io::Result<String> {
    std::env::args().nth(1).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "Please provide a file path as a command-line argument",
        )
    })
}

fn main() -> io::Result<()> {
    let file = File::open(read_path()?.trim());

    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }

    Ok(())
}
