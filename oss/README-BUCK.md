# Building ocamlrep with Buck2

## Setup

These are things that need to be done once to get going.

### Install Buck2 and Reindeer

These commands install the `buck2` and `reindeer` binaries into '~/.cargo/bin'.
```bash
    cargo install --git https://github.com/facebook/buck2.git buck2
    cargo install --git https://github.com/facebookincubator/reindeer.git reindeer
```

*Note: Make sure after installing them to configure your `PATH` environment variable so they can be found.*

### Install the OCaml package Manager

If you haven't already, install [opam](https://opam.ocaml.org/).

When opam has been installed execute `~/.ocaml-setup.sh` from the root of the distribution. The effect of `ocaml-setup.sh` is to create symlinks in `shim/third/party/ocaml` that point into the local opam installation.

*Note: The script assumes that [`OPAM_SWITCH_PREFIX`](https://opam.ocaml.org/doc/Manual.html#Switches) has been set by the way.*

## Vendor sources & generate buck rules for ocamlrep's Rust dependencies

[Reindeer](https://github.com/facebookincubator/reindeer) is a a tool that imports Rust crates from crates.io and generates Buck2 build rules for them. Run it from the root of the ocamlrep repository like this.
```bash
    reindeer --third-party-dir shim/third-party/rust vendor && \
    reindeer --third-party-dir shim/third-party/rust buckify
```

That's it, you're all set.

## Profit!

Run this command from the root of the repository to build all the targets you can.
```
    buck2 build root//...
```
To run all the tests you can, replace `build` with `test` in the above command.

More examples and more detail about building with Buck2 are available on the [Buck2 website](https://buck2.build/)!
