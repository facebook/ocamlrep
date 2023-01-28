#!/usr/bin/env bash

set -euxo pipefail

root="$(dirname "$(cargo locate-project --workspace --message-format plain)")"
targets="$root/target/debug"

# Assumes `cargo build` has already run.
ocamlopt.opt -verbose -c ocamlpool_test.ml -o ocamlpool_test_ml.cmx
ocamlopt.opt -verbose -o ocamlpool_test ocamlpool_test_ml.cmx -ccopt -L"$targets" -cclib -locamlpool_test -cclib -locamlpool

./ocamlpool_test
rm ./*.o ./*.cmi ./*.cmx ocamlpool_test
