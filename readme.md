# PKT Checksig & Signmsg
Signs and verifies signatures made with pktwallet or compatible.

## Why not use pgp?
PKT and other wallets are based on seed words which are written or remembered.
Signing with a key derived from the seed which also contains your money makes
for fewer secrets to remember.

## How to use `signmsg`:

`./target/release/signmsg <privkey> <message>`

For example:

```
❯ ./target/release/signmsg aFMZowhWGibSVLz88KHKjZ4hwafHeVdCS5US9WhFSY9yUxAQNRbC 'hello world'
H4pzcxCHYX7ilB8xoLUUuG9p0raydOOZq7IX/K91Br8UYUfIOEeUPem5xorZ7L78JGJ1NBpNXHOw+Wu8qbwjEGQ=
```

You can also pass the arguments in through stdin and pass `-` on the command line:

```
❯ echo 'aFMZowhWGibSVLz88KHKjZ4hwafHeVdCS5US9WhFSY9yUxAQNRbC hello world' | ./target/release/signmsg -
H4pzcxCHYX7ilB8xoLUUuG9p0raydOOZq7IX/K91Br8UYUfIOEeUPem5xorZ7L78JGJ1NBpNXHOw+Wu8qbwjEGQ=
```

## How to use `checksig`:

`./target/release/checksig <address> <signature> <message>`

For example:

```
❯ ./target/release/checksig pGKemQBhkQY4yce9tPnAiq4c27m1k38s2i H4pzcxCHYX7ilB8xoLUUuG9p0raydOOZq7IX/K91Br8UYUfIOEeUPem5xorZ7L78JGJ1NBpNXHOw+Wu8qbwjEGQ= 'hello world'
OK
```

You can also pass the arguments in through stdin and pass `-` on the command line:

```
❯ echo -n 'pGKemQBhkQY4yce9tPnAiq4c27m1k38s2i H4pzcxCHYX7ilB8xoLUUuG9p0raydOOZq7IX/K91Br8UYUfIOEeUPem5xorZ7L78JGJ1NBpNXHOw+Wu8qbwjEGQ= hello world' | ./target/release/checksig -
OK
```

### Result
If signature is ok, it prints "OK" to stdout and returns zero, otherwise it returns an error code.

```
❯ echo -n 'pGKemQBhkQY4yce9tPnAiq4c27m1k38s2i H4pzcxCHYX7ilB8xoLUUuG9p0raydOOZq7IX/K91Br8UYUfIOEeUPem5xorZ7L78JGJ1NBpNXHOw+Wu8qbwjEGQ= hello world' | ./target/release/checksig - ||
 echo 'did not work'
OK
❯ echo -n 'pGKemQBhkQY4yce9tPnAiq4c27m1k38s2i H4pzcxCHYX7ilB8xoLUUuG9p0raydOOZq7IX/K91Br8UYUfIOEeUPem5xorZ7L78JGJ1NBpNXHOw+Wu8qbwjEGQ= hello worldd' | ./target/release/checksig - || echo 'did not work'
Error: Signature check failed
did not work
```

## Programmatic usage

This project contains 2 subprojects, libchecksig and checksig, include libchecksig for the following:

```rust
// Parse a PKT or other bitcoin-like address, returns an address of type Bitcoin
pub fn parse_addr(addr: &str) -> Result<Address>

// Check a signature, returns Ok() if signature is good, Err() otherwise
pub fn verify_msg(addr: &str, sig: &str, msg: &str) -> Result<()>

// Parse a PKT or other Bitcoin-like private key, returns a private key of type Bitcoin
pub fn parse_pvt(privkey: &str) -> Result<PrivateKey> 

// Sign a message using a string representation of a private key
pub fn sign_msg(privkey: &str, msg: &str) -> Result<String>
```

## License

MIT OR Apache2 at your option.
