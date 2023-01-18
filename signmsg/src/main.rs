// SPDX-License-Identifier: Apache-2.0 OR MIT
use anyhow::{anyhow, bail, Result};

fn from_stdin() -> Result<String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;

    let mut it = buffer.trim().splitn(2, ' ');
    let mut clean = |name| {
        it.next()
            .map(|s| s.trim())
            .and_then(|s| (!s.is_empty()).then_some(s))
            .ok_or_else(|| anyhow!("Error: missing {}", name))
    };
    compute_sign(clean("privkey")?, clean("message")?)
}

fn compute_sign(privkey: &str, msg: &str) -> Result<String> {
    // println!("'{privkey}' '{msg}'");
    libchecksig::sign_msg(privkey, msg)
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let res = match args[..] {
        ["-"] => from_stdin(),
        [_] => Err(anyhow!("Missing message")),
        [privkey, message] => compute_sign(privkey, message),
        _ => Err(anyhow!("Invalid arguments")),
    };
    match res {
        Ok(out) => println!("{}", out),
        Err(err) => bail!(
            "{}\n\n\
                Usage:\tsignmsg <privatekey> <message>  # Sign from args\n\
                      \tsignmsg -                       # Through stdin",
            err
        ),
    }
    Ok(())
}
