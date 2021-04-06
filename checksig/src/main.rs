// SPDX-License-Identifier: Apache-2.0 OR MIT
use anyhow::{Result,bail};
use std::io::Read;

fn from_stdin() -> Result<()> {
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer)?;
    let (addr, rest) = if let Some(i) = buffer.find(' ') {
        (&buffer[0..i], &buffer[i + 1..])
    } else {
        bail!("Reading stdin: No space found: {}", buffer);
    };
    let (sig, msg) = if let Some(i) = rest.find(' ') {
        (&rest[0..i], &rest[i + 1..])
    } else {
        bail!("Reading stdin: No space found after addr: {}", buffer);
    };
    libchecksig::verify_msg(addr, sig, msg)
}

fn check() -> Result<()> {
    let mut argv = std::env::args();
    if argv.len() < 4 {
        if let Some(arg) = argv.next_back() {
            if arg == "-" {
                return from_stdin();
            }
        }
        println!("Usage: checksig <address> <signature> <message>  # Check signature from args");
        println!("       checksig -                                # Through stdin");
        bail!("Invalid arguments");
    } else {
        while argv.len() > 3 {
            argv.next().unwrap();
        }
        let args = argv.collect::<Vec<_>>();
        libchecksig::verify_msg(&args[0], &args[1], &args[2])
    }
}

// address, sig, msg
fn main() -> Result<()> {
    check()?;
    println!("OK");
    Ok(())
}