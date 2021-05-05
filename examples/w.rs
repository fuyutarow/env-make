use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // use std::fmt::Write;
    let mut buffer = std::fs::File::create("foo.txt")?;
    buffer.write_all(b"some bytes")?;
    Ok(())
}
