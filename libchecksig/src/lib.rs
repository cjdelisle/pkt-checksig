// SPDX-License-Identifier: Apache-2.0 OR MIT
use anyhow::{bail, Context, Result};
use bitcoin::hash_types::PubkeyHash;
use bitcoin::hashes::Hash;
use bitcoin::secp256k1;
use bitcoin::util::misc::{signed_msg_hash, MessageSignature};
use bitcoin::util::{address, base58, key::PrivateKey};
use bitcoin::{Address, Network};

use std::str::FromStr;

// Parse a PKT or other bitcoin-like address, returns an address of type Bitcoin
pub fn parse_addr(addr: &str) -> Result<Address> {
    let b58 =
        base58::from_check(addr).with_context(|| format!("Decoding b58 PKT addr {}", addr))?;
    if b58.len() != 21 {
        bail!("Invalid PKT address binary length {}", b58.len());
    }
    if b58[0] != 0x75 {
        bail!("Not a PKT address, begins with byte 0x{:#x}", b58[0]);
    }
    Ok(Address {
        network: Network::Bitcoin,
        payload: address::Payload::PubkeyHash(PubkeyHash::from_slice(&b58[1..])?),
    })
}

// Check a signature, returns Ok() if signature is good, Err() otherwise
pub fn verify_msg(addr: &str, sig: &str, msg: &str) -> Result<()> {
    //println!("'{}' '{}' '{}'", addr, sig, msg);
    let addr = parse_addr(addr)?;
    let ms = MessageSignature::from_str(sig).context("Decoding message signature")?;
    let msg_hash = signed_msg_hash(msg);
    let secp = secp256k1::Secp256k1::new();
    if !ms
        .is_signed_by_address(&secp, &addr, msg_hash)
        .context("Verifying sig")?
    {
        bail!("Signature check failed");
    }
    Ok(())
}

// Parse a PKT or other Bitcoin-like private key, returns a private key of type Bitcoin
pub fn parse_pvt(privkey: &str) -> Result<PrivateKey> {
    let b58 = base58::from_check(privkey)?;
    let compressed = match b58.len() {
        33 => false,
        34 => true,
        _ => bail!("Invalid data length {}", b58.len())
    };
    Ok(PrivateKey {
        compressed,
        network: Network::Bitcoin,
        key: secp256k1::SecretKey::from_slice(&b58[1..33])?,
    })
}

// Sign a message using a string representation of a private key
pub fn sign_msg(privkey: &str, msg: &str) -> Result<String> {
    let key = parse_pvt(privkey).context("Parsing private key")?;
    let msg_hash = signed_msg_hash(msg);
    let m = secp256k1::Message::from_slice(&msg_hash)?;
    let secp = secp256k1::Secp256k1::new();
    let secp_sig = secp.sign_recoverable(&m, &key.key);
    Ok(MessageSignature {
        signature: secp_sig,
        compressed: true,
    }.to_base64())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sign() {
        let sig = super::sign_msg("aFMZowhWGibSVLz88KHKjZ4hwafHeVdCS5US9WhFSY9yUxAQNRbC", "hello world").unwrap();
        println!("{}", sig);
        assert!(!super::verify_msg(
            "pGKemQBhkQY4yce9tPnAiq4c27m1k38s2i",
            &sig,
            "hello world",
        ).is_ok());
    }
    #[test]
    fn test_check() {
        assert!(super::verify_msg(
            "pDWYi9XtZHiUgoVGMqykEHwvhytYv5Ejam",
            "IFlf6v1rDHzjpJqoNSgW8ilJLeAz6ARJ+euecWrZqy43eM6J6JLnEZ4zL586DVEZfkYNJLTT6xovWardkFaq4tw=",
            "f5ef339a9b43c74900f854d9d28528e05438eb3a7ada3c66cacccb1a60c0cb27",
        ).is_ok());

        assert!(!super::verify_msg(
            "pDWYi9XtZHiUgoVGMqykEHwvhytYv5Ejam",
            "IFlf6v1rDHzjpJqoNSgW8ilJLeAz6ARJ+euecWrZqy43eM6J6JLnEZ4zL586DVEZfkYNJLTT6xovWardkFaq4tw=",
            "f5ef339a9b43c74900f854d9d28528e05438eb3a7ada3c66cacccb1a60c0cb2_",
        ).is_ok());

        assert!(!super::verify_msg(
            "pDWYi9XtZHiUgoVGMqykEHwvhytYv5Ejam", //                                      v-- here 
            "IFlf6v1rDHzjpJqoNSgW8ilJLeAz6ARJ+euecWrZqy43eM6J6JLnEZ4zL586DVEZfkYNJLTT6xovWbrdkFaq4tw=",
            "f5ef339a9b43c74900f854d9d28528e05438eb3a7ada3c66cacccb1a60c0cb27",
        ).is_ok());

        assert!(!super::verify_msg(
            "pDWYi9XtZHiUgoVGMqykEHwvhytYv5Ejan", // bad addr
            "IFlf6v1rDHzjpJqoNSgW8ilJLeAz6ARJ+euecWrZqy43eM6J6JLnEZ4zL586DVEZfkYNJLTT6xovWardkFaq4tw=",
            "f5ef339a9b43c74900f854d9d28528e05438eb3a7ada3c66cacccb1a60c0cb27",
        ).is_ok());

        assert!(!super::verify_msg(
            "p65SN5gLGwFeW7QnZUwhn2h344Dwm9km4N", // valid wrong addr
            "IFlf6v1rDHzjpJqoNSgW8ilJLeAz6ARJ+euecWrZqy43eM6J6JLnEZ4zL586DVEZfkYNJLTT6xovWardkFaq4tw=",
            "f5ef339a9b43c74900f854d9d28528e05438eb3a7ada3c66cacccb1a60c0cb27",
        ).is_ok());
    }
}