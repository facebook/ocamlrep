// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

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
