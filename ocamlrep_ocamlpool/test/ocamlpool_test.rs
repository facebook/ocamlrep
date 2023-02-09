// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![feature(exit_status_error)]

use ocamlrep_ocamlpool::ocaml_ffi;

extern "C" {
    fn ocamlpool_enter();
    fn ocamlpool_reserve_block(tag: u8, size: usize) -> usize;
    fn ocamlpool_leave();
}

// This test attempts to catch off by one issues in ocamlpool.c

// Magic constant needs to fulfill two requirements:
// Needs to be above the OCAMLPOOL_DEFAULT_SIZE constant in ocamlpool.h
//   This requirement is easy to fulfill
// Needs to be the exact size of memory block allocated by ocamlpool_reserve_block
//   which is given by the Chunk_size call in chunk_alloc in ocamlpool.c
//   This requirement requires some magic
const MAGIC_MEMORY_SIZE: usize = 1053183;

ocaml_ffi! {
    fn test() {
        unsafe {
            ocamlpool_enter();
            // This line will crash on off by one error
            ocamlpool_reserve_block(0, MAGIC_MEMORY_SIZE);
            ocamlpool_leave();
        }
    }
}

#[cfg(test)]
mod tests {
    include! {"../../cargo_test_utils/cargo_test_utils.rs"}

    #[test]
    fn ocamlpool_test() {
        let compile_cmd = ocamlopt_cmd(&[
            "-verbose",
            "-c",
            "ocamlpool_test.ml",
            "-o",
            "ocamlpool_test_ml.cmx",
        ]);
        assert_eq!(run(compile_cmd).map_err(fmt_exit_status_err), Ok(()));
        let link_cmd = ocamlopt_cmd(&[
            "-verbose",
            "-o",
            "ocamlpool_test",
            "ocamlpool_test_ml.cmx",
            "-ccopt",
            &("-L".to_owned() + workspace_dir(&["target", build_flavor()]).to_str().unwrap()),
            "-cclib",
            "-locamlpool_test",
            "-cclib",
            "-locamlrep_ocamlpool",
        ]);
        assert_eq!(run(link_cmd).map_err(fmt_exit_status_err), Ok(()));
        let ocamlpool_test_cmd = sh_cmd(&["-c", "./ocamlpool_test"]);
        assert_eq!(run(ocamlpool_test_cmd).map_err(fmt_exit_status_err), Ok(()));
    }
}
