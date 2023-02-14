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

// Hack! Trick buck into believing that these libraries are used. See [Note:
// Test blocks for Cargo] in `ocamlrep_ocamlpool/test/ocamlpool_test.rs`.
const _: () = {
    #[allow(unused_imports)]
    use anyhow;
    #[allow(unused_imports)]
    use cargo_test_utils;
    #[allow(unused_imports)]
    use tempdir;
};

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use cargo_test_utils::*;
    use tempdir::TempDir;

    #[test]
    fn ocamlrep_marshal_test() -> Result<()> {
        let tmp_dir = TempDir::new("ocamlrep_marshal_test")?;
        std::fs::copy(
            "test_ocamlrep_marshal.ml",
            tmp_dir.path().join("test_ocamlrep_marshal.ml"),
        )?;
        let compile_cmd = cmd(
            "ocamlopt.opt",
            &[
                "-verbose",
                "-c",
                "test_ocamlrep_marshal.ml",
                "-o",
                "test_ocamlrep_marshal_ml.cmx",
            ],
            Some(tmp_dir.path()),
        );
        assert_eq!(run(compile_cmd).map_err(fmt_exit_status_err), Ok(()));
        let link_cmd = cmd(
            "ocamlopt.opt",
            &[
                "-verbose",
                "-o",
                "ocamlrep_marshal_test",
                "test_ocamlrep_marshal_ml.cmx",
                "-ccopt",
                &("-L".to_owned() + workspace_dir(&["target", build_flavor()]).to_str().unwrap()),
                "-cclib",
                "-locamlrep_marshal",
                "-cclib",
                "-locamlrep_marshal_ffi_bindings",
                "-cclib",
                "-locamlrep_ocamlpool",
            ],
            Some(tmp_dir.path()),
        );
        assert_eq!(run(link_cmd).map_err(fmt_exit_status_err), Ok(()));
        let ocamlrep_marshal_test_cmd = cmd(
            tmp_dir
                .path()
                .join("ocamlrep_marshal_test")
                .as_path()
                .to_str()
                .unwrap(),
            &[],
            None,
        );
        assert_eq!(
            run(ocamlrep_marshal_test_cmd).map_err(fmt_exit_status_err),
            Ok(())
        );
        tmp_dir.close()?;
        Ok(())
    }
}
