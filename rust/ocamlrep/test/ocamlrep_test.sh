#!/usr/bin/env bash

set -euxo pipefail

root="$(dirname "$(cargo locate-project --workspace --message-format plain)")"
targets="$root/target/debug"

# Assumes `cargo build` has already run.
ocamlopt.opt -verbose -c test_ocamlrep.ml -o test_ocamlrep_ml.cmx
ocamlopt.opt -verbose -o ocamlrep_test test_ocamlrep_ml.cmx -ccopt -L"$targets" -cclib -ltest_bindings -cclib -locamlpool
./ocamlrep_test
rm *.o *.cmi *.cmx ocamlrep_test
