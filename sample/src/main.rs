use std::env;

enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}
#[derive(Debug)]
struct Size {
    byte: u64,
    kilobyte: u64,
    megabyte: u64,
    gigabyte: u64,
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size as f64 / 1000.0),
        1_000_000..=999_999_999 => FileSize::Megabytes(size as f64 / 1_000_000.0),
        _ => FileSize::Gigabytes(size as f64 / 1_000_000_000.0),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb),
    }
}

fn main() {
    let abc: Vec<String> = env::args().collect();
    let new = &abc[1];
    let abc: u64 = new.parse().expect("Failed to parse");
    let result = format_size(abc);
    println!("{}", result);

    let result1 = structured(abc);
    println!("{:?}", result1)
}

fn structured(ab: u64) -> Size {
    let (byte, kilobyte, megabyte, gigabyte) = (ab, ab / 1000, ab / 1_000_000, ab / 1_000_000_000);
    Size {
        byte,
        kilobyte,
        megabyte,
        gigabyte,
    }
}
