#!/usr/bin/env bash

set -euxo pipefail

root="$(dirname "$(cargo locate-project --workspace --message-format plain)")"
targets="$root/target/debug"

(cd "$root" && cargo build)
ocamlopt.opt -verbose -c ocamlpool_test.ml -o ocamlpool_test_ml.cmx
ocamlopt.opt -verbose -o ocamlpool_test ocamlpool_test_ml.cmx -ccopt "-L$targets -locamlpool -locamlpool_test"

./ocamlpool_test
