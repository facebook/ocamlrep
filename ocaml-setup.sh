# shellcheck disable=SC2148

# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

# The commands in this script are to be executed in the current shell and so
# invoke it via the builtin 'source' command e.g. `source ocaml-setup.sh`. The
# script assumes an opam installation. It activates the '4.14.0' switch and
# writes symlinks to opam into 'shim/third-party/ocaml/'.

if ! command -v opam &> /dev/null
then
    echo "opam is not installed, which is a dependency for building targets in ocaml."
    exit
fi

# Bring the OCaml toolchain into scope.
eval "$(opam env --switch=4.14.0 --set-switch)"

# Link 'shim/third-party/ocaml/standard_library'.
if [ ! -L shim/third-party/ocaml/standard_library ]; then
  (cd shim/third-party/ocaml && ln -s "$(ocamlopt.opt -config | grep standard_library: | awk '{ print $2 }' )" standard_library)
else
    echo "Link 'shim/third-party/ocaml/standard_library' exists. To overwrite it, first remove it and run $0 again"
fi

# Link 'third-party/ocaml/opam'.
if [ ! -L shim/third-party/ocaml/opam ]; then
  (cd shim/third-party/ocaml && ln -s "$OPAM_SWITCH_PREFIX" opam)
else
    echo "Link 'shim/third-party/ocaml/opam' exists. To overwrite it, first remove it and run $0 again"
fi
