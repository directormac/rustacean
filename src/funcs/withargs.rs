use std::io::{self, Write};

// Add
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn double(a: i32) -> i32 {
    a + a
}

pub fn printme(args: &str) -> std::io::Result<()> {
    let mut stdout = io::stdout();
    let _ = stdout.write_all(args.as_bytes());
    Ok(())
}
