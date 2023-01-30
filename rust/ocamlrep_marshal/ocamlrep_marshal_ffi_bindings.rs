// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
#![feature(exit_status_error)]

use ocamlrep::FromOcamlRep;

type OcamlValue = usize;

#[no_mangle]
unsafe extern "C" fn ocamlrep_marshal_output_value_to_string(
    v: OcamlValue,
    flags: OcamlValue,
) -> OcamlValue {
    ocamlrep_ocamlpool::catch_unwind(|| {
        let v = ocamlrep::Value::from_bits(v);
        let flags = ocamlrep_marshal::ExternFlags::from_ocaml(flags).unwrap();
        let mut cursor = std::io::Cursor::new(vec![]);
        ocamlrep_marshal::output_value(&mut cursor, v, flags).unwrap();
        ocamlrep_ocamlpool::to_ocaml(&cursor.into_inner())
    })
}

#[no_mangle]
unsafe extern "C" fn ocamlrep_marshal_input_value_from_string(
    str: OcamlValue,
    ofs: OcamlValue,
) -> OcamlValue {
    ocamlrep_ocamlpool::catch_unwind(|| {
        let offset = usize::from_ocaml(ofs).unwrap();
        let str = ocamlrep::bytes_from_ocamlrep(ocamlrep::Value::from_bits(str)).unwrap();
        let str = &str[offset..];
        let pool = ocamlrep_ocamlpool::Pool::new();
        ocamlrep_marshal::input_value(str, &pool).to_bits()
    })
}

#[cfg(test)]
mod tests {
    include! {"../../cargo_test_utils/cargo_test_utils.rs"}

    #[test]
    fn ocamlrep_marshal_test() {
        let mut compile_cmd = ocamlopt_cmd(&[
            "-verbose",
            "-c",
            "ocamlrep_marshal_ffi.ml",
            "-o",
            "ocamlrep_marshal_ffi.cmx",
        ]);
        compile_cmd.current_dir("../..");
        assert_eq!(run(compile_cmd).map_err(fmt_exit_status_err), Ok(()));
        let mut archive_cmd = ocamlopt_cmd(&[
            "-verbose",
            "-a",
            "-o",
            "ocamlrep_marshal_ffi.cmxa",
            "ocamlrep_marshal_ffi.cmx",
            "-ccopt",
            &("-L".to_owned() + workspace_dir(&["target", build_flavor()]).to_str().unwrap()),
            "-cclib",
            "-locamlrep_marshal_ffi_bindings",
        ]);
        archive_cmd.current_dir("../..");
        assert_eq!(run(archive_cmd).map_err(fmt_exit_status_err), Ok(()));
        let mut compile_cmd = ocamlopt_cmd(&[
            "-verbose",
            "-c",
            "test/test_ocamlrep_marshal.ml",
            "-o",
            "test_ocamlrep_marshal_ml.cmx",
        ]);
        compile_cmd.current_dir("../..");
        assert_eq!(run(compile_cmd).map_err(fmt_exit_status_err), Ok(()));
        let mut link_cmd = ocamlopt_cmd(&[
            "-verbose",
            "-o",
            "ocamlrep_marshal_test",
            "ocamlrep_marshal_ffi.cmxa",
            "test_ocamlrep_marshal_ml.cmx",
            "-ccopt",
            &("-L".to_owned() + workspace_dir(&["target", build_flavor()]).to_str().unwrap()),
            "-cclib",
            "-locamlrep_marshal",
            "-cclib",
            "-locamlrep_marshal_ffi_bindings",
            "-cclib",
            "-locamlpool",
        ]);
        link_cmd.current_dir("../..");
        assert_eq!(run(link_cmd).map_err(fmt_exit_status_err), Ok(()));
        let mut ocamlrep_marshal_test_cmd = sh_cmd(&["-c", "./ocamlrep_marshal_test"]);
        ocamlrep_marshal_test_cmd.current_dir("../..");
        assert_eq!(
            run(ocamlrep_marshal_test_cmd).map_err(fmt_exit_status_err),
            Ok(())
        );
    }
}
