# ed25519_enc

[<img alt="github" src="https://img.shields.io/badge/ed25519_enc-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Experimental-Projects-NAS/ed25519_enc)
[<img alt="crates.io" src="https://img.shields.io/crates/v/ed25519_enc.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/ed25519_enc)
[<img alt="NPM Version" src="https://img.shields.io/npm/v/ed25519_enc?style=for-the-badge&logo=npm&color=red" height="20">](https://www.npmjs.com/package/ed25519_enc)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-ed25519_enc-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/ed25519_enc)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/Experimental-Projects-NAS/ed25519_enc/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/Experimental-Projects-NAS/ed25519_enc/actions?query=branch%3Amain)

**ed25519_enc:** A Rust and Node.js library that facilitates secure encryption and decryption by converting Ed25519 keys to X25519 keys. It utilizes elliptic-curve Diffie-Hellman (ECDH) to derive a shared secret, which is then used for symmetric encryption with AES-256-GCM.

This project was bootstrapped by [create-neon](https://www.npmjs.com/package/create-neon).

## Table of Contents
* [NPM usage](#npm-usage)
  + [Installation](#installation)
  + [From ed25519 private key to x25519](#from-ed25519-private-key-to-x25519)
  + [From ed25519 public key to x25519](#from-ed25519-public-key-to-x25519)
  + [Encrypt and decrypt data using ed25519 keys](#encrypt-and-decrypt-data-using-ed25519-keys)
* [Cargo usage](#cargo-usage)
  + [Installation](#installation-1)
  + [From ed25519 private key to x25519](#from-ed25519-private-key-to-x25519-1)
  + [From ed25519 public key to x25519](#from-ed25519-public-key-to-x25519-1)
  + [Encrypt and decrypt data using ed25519 keys](#encrypt-and-decrypt-data-using-ed25519-keys-1)
* [Building ed25519_enc](#building-ed25519_enc)
* [Exploring ed25519_enc](#exploring-ed25519_enc)
* [Available Scripts](#available-scripts)
    - [`npm run build`](#npm-run-build)
    - [`npm run debug`](#npm-run-debug)
    - [`npm run cross`](#npm-run-cross)
    - [`npm run release`](#npm-run-release)
    - [`npm run dryrun`](#npm-run-dryrun)
    - [`npm test`](#npm-test)
* [Project Layout](#project-layout)
* [Learn More](#learn-more)
* [LICENSE](#license)

<small><i><a href='http://ecotrust-canada.github.io/markdown-toc/'>Table of contents generated with markdown-toc</a></i></small>

## NPM usage

### Installation
```sh
npm i ed25519_enc
```

### From ed25519 private key to x25519
```js
import { to_x25519_priv } from "ed25519_enc";
...
const ed25519_priv = Buffer.from(
  "D12549B4AE46086002E4110B1F94D0BA942C2967AA83D3003590FD7A5087C6A8",
  "hex"
);
const x25519_priv = to_x25519_priv(ed25519_priv);
```

### From ed25519 public key to x25519
```js
import { to_x25519_pub } from "ed25519_enc";
...
const ed25519_pub = Buffer.from(
  "B68F106DAA004E7AE715C5159E1C27CB864EED20D3ACB332BD6F87E89226E925",
  "hex"
);
const x25519_pub = to_x25519_pub(ed25519_pub);
```

### Encrypt and decrypt data using ed25519 keys
```js
import { encrypt } from "ed25519_enc";
...
const sender_priv = Buffer.from(
  "D12549B4AE46086002E4110B1F94D0BA942C2967AA83D3003590FD7A5087C6A8",
  "hex"
);
const sender_pub = Buffer.from(
  "B68F106DAA004E7AE715C5159E1C27CB864EED20D3ACB332BD6F87E89226E925",
  "hex"
);
const receiver_priv = Buffer.from(
  "3C659C3FD72E56861C81C1684C5C342958645E2C459A11A2282B46161E47BF2E",
  "hex"
);
const receiver_pub = Buffer.from(
  "C02ABDA285014F207D5D31219E0F5647484F6DFF09E595DF38BDBDB07CE6E1B2",
  "hex"
);
const data = Buffer.from("Hello world!");
const encrypted_data = encrypt(sender_priv, receiver_pub, data);
const decrypted_data = decrypt(receiver_priv, sender_pub, encrypted_data);

console.log("Decrypted data: ", new TextDecoder().decode(dec_data));
// Decrypted data:  Hello World!
```

## Cargo usage
### Installation
```sh
cargo add ed25519_enc
```

### From ed25519 private key to x25519

```rust
use ed25519_enc::{to_x25519_priv};
...
let ed25519_priv: [u8; 32] = [
  157, 097, 177, 157, 239, 253, 090, 096,
  186, 132, 074, 244, 146, 236, 044, 196,
  068, 073, 197, 105, 123, 050, 105, 025,
  112, 059, 172, 003, 028, 174, 127, 096,
];
let x25519_priv: [u8; 32] = to_x25519_priv(ed25519_priv);
```

### From ed25519 public key to x25519

```rust
use ed25519_enc::{to_x25519_pub};
...
let ed25519_pub: [u8; 32] = [
  215, 090, 152, 001, 130, 177, 010, 183,
  213, 075, 254, 211, 201, 100, 007, 058,
  014, 225, 114, 243, 218, 166, 035, 037,
  175, 002, 026, 104, 247, 007, 081, 026,
];
let x25519_pub: [u8; 32] = to_x25519_pub(ed25519_pub);
```

### Encrypt and decrypt data using ed25519 keys

```rust
use ed25519_enc::{encrypt, decrypt}

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


## Building ed25519_enc

Building ed25519_enc requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support).

To run the build, run:

```sh
$ npm run build
```

This command uses the [@neon-rs/cli](https://www.npmjs.com/package/@neon-rs/cli) utility to assemble the binary Node addon from the output of `cargo`.

## Exploring ed25519_enc

After building ed25519_enc, you can explore its exports at the Node console:

```sh
$ npm i
$ npm run build
$ node
> const lib = require('.');
> const sender_priv = Buffer.from("D12549B4AE46086002E4110B1F94D0BA942C2967AA83D3003590FD7A5087C6A8", "hex");
> const sender_pub = Buffer.from("B68F106DAA004E7AE715C5159E1C27CB864EED20D3ACB332BD6F87E89226E925", "hex");
> const receiver_priv = Buffer.from("3C659C3FD72E56861C81C1684C5C342958645E2C459A11A2282B46161E47BF2E", "hex");
> const receiver_pub = Buffer.from("C02ABDA285014F207D5D31219E0F5647484F6DFF09E595DF38BDBDB07CE6E1B2", "hex");
> const data = Buffer.from("Hello World!");
> const enc_data = lib.encrypt(sender_priv, receiver_pub, data);
> const dec_data = lib.decrypt(receiver_priv, sender_pub, enc_data);
> console.log("Decrypted data: ", new TextDecoder().decode(dec_data));

Decrypted data:  Hello World!
```

## Available Scripts

In the project directory, you can run:

#### `npm run build`

Builds the Node addon (`index.node`) from source, generating a release build with `cargo --release`.

Additional [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) arguments may be passed to `npm run build` and similar commands. For example, to enable a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
npm run build -- --feature=beetle
```

#### `npm run debug`

Similar to `npm run build` but generates a debug build with `cargo`.

#### `npm run cross`

Similar to `npm run build` but uses [cross-rs](https://github.com/cross-rs/cross) to cross-compile for another platform. Use the [`CARGO_BUILD_TARGET`](https://doc.rust-lang.org/cargo/reference/config.html#buildtarget) environment variable to select the build target.

#### `npm run release`

Initiate a full build and publication of a new patch release of this library via GitHub Actions.

#### `npm run dryrun`

Initiate a dry run of a patch release of this library via GitHub Actions. This performs a full build but does not publish the final result.

#### `npm test`

Runs the unit tests by calling `cargo test`. You can learn more about [adding tests to your Rust code](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) from the [Rust book](https://doc.rust-lang.org/book/).

## Project Layout

The directory structure of this project is:

```
ed25519_enc/
├── Cargo.toml
├── README.md
├── lib/
├── src/
|   ├── index.mts
|   └── index.cts
├── crates/
|   └── ed25519_enc/
|       └── src/
|           └── lib.rs
├── platforms/
├── package.json
└── target/
```

| Entry          | Purpose                                                                                                                                  |
|----------------|------------------------------------------------------------------------------------------------------------------------------------------|
| `Cargo.toml`   | The Cargo [manifest file](https://doc.rust-lang.org/cargo/reference/manifest.html), which informs the `cargo` command.                   |
| `README.md`    | This file.                                                                                                                               |
| `lib/`         | The directory containing the generated output from [tsc](https://typescriptlang.org).                                                    |
| `src/`         | The directory containing the TypeScript source files.                                                                                    |
| `index.mts`    | Entry point for when this library is loaded via [ESM `import`](https://nodejs.org/api/esm.html#modules-ecmascript-modules) syntax.       |
| `index.cts`    | Entry point for when this library is loaded via [CJS `require`](https://nodejs.org/api/modules.html#requireid).                          |
| `crates/`      | The directory tree containing the Rust source code for the project.                                                                      |
| `lib.rs`       | Entry point for the Rust source code.                                                                                                          |
| `platforms/`   | The directory containing distributions of the binary addon backend for each platform supported by this library.                          |
| `package.json` | The npm [manifest file](https://docs.npmjs.com/cli/v7/configuring-npm/package-json), which informs the `npm` command.                    |
| `target/`      | Binary artifacts generated by the Rust build.                                                                                            |

## Learn More

Learn more about:

- [Neon](https://neon-bindings.com).
- [Rust](https://www.rust-lang.org).
- [Node](https://nodejs.org).

## LICENSE

[MIT](LICENSE-MIT) or [Apache License 2.0](LICENSE-APACHE) (at your option).