use std::env;

#[derive(Debug)]
enum FileSize {
    Bytes(f64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

#[derive(Debug, PartialEq)]
struct Sizes {
    kb: f64,
    mb: f64,
    gb: f64,
}

impl FileSize {
    fn as_bytes(&self) -> f64 {
        match self {
            FileSize::Bytes(value) => *value,
            FileSize::Kilobytes(value) => value * 1_000.0,
            FileSize::Megabytes(value) => value * 1_000_000.0,
            FileSize::Gigabytes(value) => value * 1_000_000_000.0,
        }
    }
}

fn format_size(input: &str) -> Result<Sizes, String> {
    let mut parts = input.split_whitespace();
    let value = parts
        .next()
        .ok_or_else(|| "size must include a number and a unit".to_string())?
        .parse::<f64>()
        .map_err(|_| "size must start with a valid number".to_string())?;
    let unit = parts
        .next()
        .ok_or_else(|| "size must include a unit: b, kb, mb, or gb".to_string())?;

    if !value.is_finite() || value < 0.0 {
        return Err("size must be a non-negative finite number".to_string());
    }

    if parts.next().is_some() {
        return Err("size must contain only a number and a unit".to_string());
    }

    let file_size = match unit.to_ascii_lowercase().as_str() {
        "b" | "bytes" => FileSize::Bytes(value),
        "kb" => FileSize::Kilobytes(value),
        "mb" => FileSize::Megabytes(value),
        "gb" => FileSize::Gigabytes(value),
        _ => return Err("unit must be b, kb, mb, or gb".to_string()),
    };

    let bytes = file_size.as_bytes();
    Ok(Sizes {
        kb: bytes / 1_000.0,
        mb: bytes / 1_000_000.0,
        gb: bytes / 1_000_000_000.0,
    })
}

fn main() {
    let input = env::args().skip(1).collect::<Vec<_>>().join(" ");
    if input.is_empty() {
        eprintln!("Usage: first_rust_project <number> <unit>");
        return;
    }

    let size = format_size(&input).unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    });

    println!("{:?}", size);
}
