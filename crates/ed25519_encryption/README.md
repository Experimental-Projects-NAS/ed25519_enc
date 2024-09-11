# ed25519_encryption
A Rust library that facilitates secure encryption and decryption by converting **Ed25519** keys to **X25519** keys. Utilizes **elliptic-curve Diffie-Hellman (ECDH)** to derive a shared secret for **aes-gcm-256** symmetrical encryption.

## Installation

```shell
$ cargo add ed25519_encryption
```

## Usage

From **ed25519** private key to **x25519**:

```rust
use ed25519_encryption::{to_x25519_priv};
...
let ed25519_priv: [u8; 32] = [
  157, 097, 177, 157, 239, 253, 090, 096,
  186, 132, 074, 244, 146, 236, 044, 196,
  068, 073, 197, 105, 123, 050, 105, 025,
  112, 059, 172, 003, 028, 174, 127, 096,
];
let x25519_priv: [u8; 32] = to_x25519_priv(ed25519_priv);
```

From **ed25519** public key to **x25519**:

```rust
use ed25519_encryption::{to_x25519_pub};
...
let ed25519_pub: [u8; 32] = [
  215, 090, 152, 001, 130, 177, 010, 183,
  213, 075, 254, 211, 201, 100, 007, 058,
  014, 225, 114, 243, 218, 166, 035, 037,
  175, 002, 026, 104, 247, 007, 081, 026,
];
let x25519_pub: [u8; 32] = to_x25519_pub(ed25519_pub);
```

Encrypt and decrypt data/message using **ed25519** keys and **aes-gcm-256**:

```rust
use ed25519_encryption::{encrypt, decrypt}

...

let sender_ed25519_priv: [u8; 32] = [
    157, 097, 177, 157, 239, 253, 090, 096,
    186, 132, 074, 244, 146, 236, 044, 196,
    068, 073, 197, 105, 123, 050, 105, 025,
    112, 059, 172, 003, 028, 174, 127, 096,
];

let sender_ed25519_pub: [u8; 32] = [
    215, 090, 152, 001, 130, 177, 010, 183,
    213, 075, 254, 211, 201, 100, 007, 058,
    014, 225, 114, 243, 218, 166, 035, 037,
    175, 002, 026, 104, 247, 007, 081, 026,
];

let receiver_ed25519_priv: [u8; 32] = [
    074, 032, 095, 005, 083, 039, 001, 050,
    066, 088, 090, 056, 021, 011, 031, 011,
    087, 068, 033, 014, 025, 028, 010, 043,
    024, 066, 058, 076, 030, 074, 017, 076,
];

let receiver_ed25519_pub: [u8; 32] = [
    204, 095, 038, 061, 080, 055, 143, 219,
    253, 138, 015, 190, 145, 041, 028, 081,
    001, 004, 227, 080, 197, 078, 000, 043,
    160, 130, 158, 047, 197, 058, 228, 232
];

let data: &[u8] = b"Hello world!";

let enc_msg: Vec<u8> = encrypt(
    sender_ed25519_priv,
    receiver_ed25519_pub,
    data
);

let dec_data: Vec<u8> = decrypt(
    receiver_ed25519_priv,
    sender_ed25519_pub,
    &enc_msg
);

println!("{:?}", dec_data); // should prints "Hello world!"
```

## License
[MIT](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE) (at your option).