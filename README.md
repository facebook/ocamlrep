# ocamlrep: Interop OCaml and Rust code [![Cargo build and test](https://github.com/facebook/ocamlrep/actions/workflows/cargo-build-and-test.yml/badge.svg)](https://github.com/facebook/ocamlrep/actions/workflows/cargo-build-and-test.yml)

The goal of this project is to make OCaml and Rust code interoperable. While in its early stage of development, this library is actively used by the [HHVM](https://github.com/facebook/hhvm) project, mostly in the Hack type checker, to enable OCaml code to rely on Rust.

## Requirements
This project is stand-alone and is currently tested with:
  - OCaml 5.1.1;
  - The Rust toolchain in [./rust-toolchain](rust-toolchain)

## Building ocamlrep

Use [Cargo](https://doc.rust-lang.org/cargo/guide/cargo-home.html).

- Cargo: Install OPAM then `cargo build`.

## Contributing
See the [CONTRIBUTING](CONTRIBUTING.md) file for how to help out.

## License
ocamlrep has an MIT license. See the LICENSE file included in this distribution.
