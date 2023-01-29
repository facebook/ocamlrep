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
    use std::process::{Command, ExitStatusError};

    fn cmd(prog: &str, args: &[&str]) -> Command {
        let mut prog_cmd = Command::new(prog);
        prog_cmd.args(args);
        prog_cmd
    }

    fn ocamlopt_cmd(args: &[&str]) -> Command {
        cmd("ocamlopt.opt", args)
    }

    fn sh_cmd(args: &[&str]) -> Command {
        cmd("sh", args)
    }

    fn cargo_cmd(args: &[&str]) -> Command {
        cmd("cargo", args)
    }

    fn workspace_dir(ds: &[&str]) -> std::path::PathBuf {
        let mut cargo_cmd = cargo_cmd(&["locate-project", "--workspace", "--message-format=plain"]);
        let output = cargo_cmd.output().unwrap().stdout;
        let root_cargo_toml = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
        let mut p = root_cargo_toml.parent().unwrap().to_path_buf();
        for d in ds {
            p.push(d);
        }
        p
    }

    fn run(mut cmd: Command) -> Result<(), ExitStatusError> {
        cmd.spawn().unwrap().wait().ok().unwrap().exit_ok()
    }

    fn fmt_exit_status_err(err: ExitStatusError) -> String {
        format!("error status: {err}")
    }

    fn build_flavor() -> &'static str {
        if cfg!(debug_assertions) {
            "debug"
        } else {
            "release"
        }
    }

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
            "-locamlpool",
        ]);
        assert_eq!(run(link_cmd).map_err(fmt_exit_status_err), Ok(()));
        let ocamlpool_test_cmd = sh_cmd(&["-c", "./ocamlpool_test"]);
        assert_eq!(run(ocamlpool_test_cmd).map_err(fmt_exit_status_err), Ok(()));
    }
}
