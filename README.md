# ocamlrep: Interop Ocaml and Rust code

he goal of this project is make OCaml and Rust code interoperable.
While in its early stage of development, this library is actively used
by the [HHVM](https://github.com/facebook/hhvm) project, mostly in the Hack
type checker, to make ocaml code rely on rust.

## Requirements
This project is stand-alone and should compile with recent versions of the
ocaml compiler and rust echo-system.

ocamlrep requires or works with:
* Ocaml 4.14.0
* Rust 1.65.0 (see Cargo.lock for third-party information)

## Building ocamlrep
TODO

See the [CONTRIBUTING](CONTRIBUTING.md) file for how to help out.


## License
ocamlrep is using the MIT license, as found in the LICENSE file.
