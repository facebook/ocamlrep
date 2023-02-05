// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![feature(exit_status_error)]

use std::cell::Cell;

use ocamlrep_custom::caml_serialize_default_impls;
use ocamlrep_custom::CamlSerialize;
use ocamlrep_custom::Custom;
use ocamlrep_ocamlpool::ocaml_ffi;

pub struct Counter(Cell<isize>);

impl CamlSerialize for Counter {
    caml_serialize_default_impls!();
}

ocaml_ffi! {
    fn counter_new() -> Custom<Counter> {
        Custom::from(Counter(Cell::new(0)))
    }

    fn counter_inc(counter: Custom<Counter>) -> Custom<Counter> {
        counter.0.set(counter.0.get() + 1);
        counter
    }

    fn counter_read(counter: Custom<Counter>) -> isize {
        counter.0.get()
    }
}

#[cfg(test)]
mod tests {
    include! {"../../../cargo_test_utils/cargo_test_utils.rs"}

    #[test]
    fn counter_test() {
        let compile_cmd = ocamlopt_cmd(&[
            "-verbose",
            "-c",
            "counter_client.ml",
            "-o",
            "counter_client_ml.cmx",
        ]);
        assert_eq!(run(compile_cmd).map_err(fmt_exit_status_err), Ok(()));
        let link_cmd = ocamlopt_cmd(&[
            "-verbose",
            "-o",
            "counter_test",
            "counter_client_ml.cmx",
            "-ccopt",
            &("-L".to_owned() + workspace_dir(&["target", build_flavor()]).to_str().unwrap()),
            "-cclib",
            "-lcounter",
            "-cclib",
            "-locamlpool",
        ]);
        assert_eq!(run(link_cmd).map_err(fmt_exit_status_err), Ok(()));
        let counter_test_cmd = sh_cmd(&["-c", "./counter_test"]);
        assert_eq!(run(counter_test_cmd).map_err(fmt_exit_status_err), Ok(()));
    }
}
