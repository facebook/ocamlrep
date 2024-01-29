# Building ocamlrep with Buck2 and Reindeer

Instructions on how to setup a Buck2 build of ocamlrep. More Buck2 information & examples can be found on the [Buck2 website](https://buck2.build/).

## Setup

### Install Buck2

#### Buck2 binary
- Get the latest prebuilt `buck2` binary (and symlink it into '/usr/local/bin').

     ```bash
     wget https://github.com/facebook/buck2/releases/download/latest/buck2-"$PLAT".zst
     zstd -d buck2-"$PLAT".zst -o buck2
     chmod +x buck2
     sudo ln -s "$(pwd)"/buck2 /usr/local/bin/buck2
     ```
*Valid values for `$PLAT` are `x86_64-unknown-linux-gnu` on Linux, `x86_64-apple-darwin` on x86 macOS and `aarch64-apple-darwin` on ARM macOS.*

#### Buck2 prelude
- Initialize a Buck2 prelude Git submodule.

     ```bash
     git submodule add https://github.com/facebook/buck2-prelude.git prelude
     git submodule update --init
     ```
- Checkout the prelude at the right commit.

     ```bash
     prelude_hash=$(curl https://github.com/facebook/buck2/releases/download/latest/prelude_hash)
     (cd prelude && git checkout $prelude_hash)
     ```

### Reindeer
[Reindeer](https://github.com/facebookincubator/reindeer) is a tool to generate Buck2 rules for Rust crates.

- Install the `reindeer` binary from source (into '~/.cargo/bin').

     ```bash
     cargo install --git https://github.com/facebookincubator/reindeer.git reindeer
     ```

### OPAM
- Initialize [OPAM](https://opam.ocaml.org/).

     ```bash
     opam init --compiler=5.1.1 --disable-sandboxing -y
     eval $(opam env)
     ```
### Symlink OPAM
- Run the script 'ocaml-setup.sh'.

     ```bash
     ./ocaml-setup.sh
     ```
*Note: The script assumes that [`OPAM_SWITCH_PREFIX`](https://opam.ocaml.org/doc/Manual.html#Switches) is set.*

### Generate BUCK rules for third-party Rust.
- Run `reindeer buckify`.

     ```bash
     reindeer --third-party-dir shim/third-party/rust buckify
     ```

## Build

- Build the complete set of ocamlrep targets.

     ```bash
     buck2 build root//...
     ```
