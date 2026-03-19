use std::io::{self, Read, Write};

use anyhow::Result;

fn main() -> Result<()> {
    let mut c_code = String::new();
    io::stdin().lock().read_to_string(&mut c_code)?;

    io::stdout().write_all(c_code.as_bytes())?;
    io::stdout().flush()?;

    Ok(())
}
