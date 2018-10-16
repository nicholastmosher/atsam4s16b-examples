# `atsam4s16b-examples`

> A set of embedded Rust example programs for the atsam4s16b.

## Getting Started

Building for ARM targets currently requires some preview tools to be installed
which are currently distributed in Rust's beta channel, but some of the examples
require nightly, so the instructions here are to set up nightly. Here's what
you'll need to have installed:

### Installation

* Rust nightly:

```
rustup update nightly
rustup default nightly
```

* ARM toolchain needed for the atsam4s16b:

```
rustup target add thumbv7em-none-eabi
```

* Cargo-binutils for preparing the binary:

```
rustup component add llvm-tools-preview
cargo install cargo-binutils
```

### Compiling targets

Now we can use `cargo build` like normal, which will produce an elf file in the
`target/thumbv7em-none-eabi` directory. To prepare the program for flashing, we
still need to use `objdump` which came packaged with `cargo-binutils`. For example,
to prepare blinky, run the following:

```
cargo objcopy --bin blinky --release -- -O binary blinky.bin
```

This will also run `cargo-build` if your sources have updated, but then will place
the `blinky.bin` file in the current directory. This `blinky.bin` file is the binary
file to be flashed to the atsam4s16b for execution.

# License

This crate is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
