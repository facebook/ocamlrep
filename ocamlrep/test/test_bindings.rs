// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
#![feature(exit_status_error)]

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use ocamlrep::FromOcamlRep;
use ocamlrep::ToOcamlRep;

fn val<T: FromOcamlRep + ToOcamlRep>(value: T) -> usize {
    let arena = Box::leak(Box::new(ocamlrep::Arena::new()));
    let value = arena.add(&value);
    // Round-trip back to T to exercise from_ocamlrep.
    let value = T::from_ocamlrep(value).unwrap();
    let value = arena.add(&value);
    value.to_bits()
}

/// # Safety
/// `value` must be a valid pointer to an OCaml value.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn convert_to_ocamlrep(value: usize) -> usize {
    unsafe {
        let arena = Box::leak(Box::new(ocamlrep::Arena::new()));
        let value = ocamlrep::Value::from_bits(value);
        let value = value.clone_with_allocator(arena);
        value.to_bits()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn realloc_in_ocaml_heap(value: usize) -> usize {
    let value = unsafe { ocamlrep::Value::from_bits(value) };
    let pool = unsafe { ocamlrep_ocamlpool::Pool::new() };
    value.clone_with_allocator(&pool).to_bits()
}

// Primitive Tests

#[unsafe(no_mangle)]
pub extern "C" fn get_a(_unit: usize) -> usize {
    val('a')
}

#[unsafe(no_mangle)]
pub extern "C" fn get_five(_unit: usize) -> usize {
    val(5)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_true(_unit: usize) -> usize {
    val(true)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_false(_unit: usize) -> usize {
    val(false)
}

// Option Tests

#[unsafe(no_mangle)]
pub extern "C" fn get_none(_unit: usize) -> usize {
    val(None::<isize>)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_some_five(_unit: usize) -> usize {
    val(Some(5))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_some_none(_unit: usize) -> usize {
    val(Some(None::<isize>))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_some_some_five(_unit: usize) -> usize {
    val(Some(Some(5)))
}

// Ref tests

#[unsafe(no_mangle)]
pub extern "C" fn get_int_ref(_unit: usize) -> usize {
    val(RefCell::new(5))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_int_option_ref(_unit: usize) -> usize {
    val(RefCell::new(Some(5)))
}

// Unsized type tests

#[unsafe(no_mangle)]
pub extern "C" fn get_str(_unit: usize) -> usize {
    let arena = Box::leak(Box::new(ocamlrep::Arena::new()));
    arena.add("static str").to_bits()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_byte_slice(_unit: usize) -> usize {
    let arena = Box::leak(Box::new(ocamlrep::Arena::new()));
    arena.add(&b"byte\x00\xFFslice"[..]).to_bits()
}

#[unsafe(no_mangle)]
pub extern "C" fn get_int_opt_slice(_unit: usize) -> usize {
    let arena = Box::leak(Box::new(ocamlrep::Arena::new()));
    let vec = [None, Some(2), Some(3)];
    let slice = &vec[..];
    arena.add(slice).to_bits()
}

// List Tests

#[unsafe(no_mangle)]
pub extern "C" fn get_empty_list(_unit: usize) -> usize {
    val(Vec::<isize>::new())
}

#[unsafe(no_mangle)]
pub extern "C" fn get_five_list(_unit: usize) -> usize {
    val(vec![5])
}

#[unsafe(no_mangle)]
pub extern "C" fn get_one_two_three_list(_unit: usize) -> usize {
    val(vec![1, 2, 3])
}

#[unsafe(no_mangle)]
pub extern "C" fn get_float_list(_unit: usize) -> usize {
    val(vec![1.0, 2.0, 3.0])
}

// Struct tests

#[derive(FromOcamlRep, ToOcamlRep)]
struct Foo {
    a: isize,
    b: bool,
}

#[derive(FromOcamlRep, ToOcamlRep)]
struct Bar {
    c: Foo,
    d: Option<Vec<Option<isize>>>,
}

#[unsafe(no_mangle)]
pub extern "C" fn get_foo(_unit: usize) -> usize {
    val(Foo { a: 25, b: true })
}

#[unsafe(no_mangle)]
pub extern "C" fn get_bar(_unit: usize) -> usize {
    val(Bar {
        c: Foo { a: 42, b: false },
        d: Some(vec![Some(88), None, Some(66)]),
    })
}

// String Tests

#[unsafe(no_mangle)]
pub extern "C" fn get_empty_string(_unit: usize) -> usize {
    val(String::from(""))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_a_string(_unit: usize) -> usize {
    val(String::from("a"))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_ab_string(_unit: usize) -> usize {
    val(String::from("ab"))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_abcde_string(_unit: usize) -> usize {
    val(String::from("abcde"))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_abcdefg_string(_unit: usize) -> usize {
    val(String::from("abcdefg"))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_abcdefgh_string(_unit: usize) -> usize {
    val(String::from("abcdefgh"))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_zero_float(_unit: usize) -> usize {
    val(0.0_f64)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_one_two_float(_unit: usize) -> usize {
    val(1.2_f64)
}

// Variant tests

#[derive(FromOcamlRep, ToOcamlRep)]
enum Fruit {
    Apple,
    Orange(isize),
    Pear { num: isize },
    Kiwi,
}

#[unsafe(no_mangle)]
pub extern "C" fn get_apple(_unit: usize) -> usize {
    val(Fruit::Apple)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_orange(_unit: usize) -> usize {
    val(Fruit::Orange(39))
}

#[unsafe(no_mangle)]
pub extern "C" fn get_pear(_unit: usize) -> usize {
    val(Fruit::Pear { num: 76 })
}

#[unsafe(no_mangle)]
pub extern "C" fn get_kiwi(_unit: usize) -> usize {
    val(Fruit::Kiwi)
}

// Map tests

#[unsafe(no_mangle)]
pub extern "C" fn get_empty_smap(_unit: usize) -> usize {
    let map: BTreeMap<String, isize> = BTreeMap::new();
    val(map)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_int_smap_singleton(_unit: usize) -> usize {
    let mut map = BTreeMap::new();
    map.insert(String::from("a"), 1);
    val(map)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_int_smap(_unit: usize) -> usize {
    let mut map = BTreeMap::new();
    map.insert(String::from("a"), 1);
    map.insert(String::from("b"), 2);
    map.insert(String::from("c"), 3);
    val(map)
}

// Set tests

#[unsafe(no_mangle)]
pub extern "C" fn get_empty_sset(_unit: usize) -> usize {
    let set: BTreeSet<String> = BTreeSet::new();
    val(set)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_sset_singleton(_unit: usize) -> usize {
    let mut set = BTreeSet::new();
    set.insert(String::from("a"));
    val(set)
}

#[unsafe(no_mangle)]
pub extern "C" fn get_sset(_unit: usize) -> usize {
    let mut set = BTreeSet::new();
    set.insert(String::from("a"));
    set.insert(String::from("b"));
    set.insert(String::from("c"));
    val(set)
}

#[unsafe(no_mangle)]
pub extern "C" fn roundtrip_int64(value: usize) -> usize {
    let i = unsafe { ocamlrep_caml_builtins::Int64::from_ocaml(value).unwrap() };
    val(i)
}

// Hack! Trick buck into believing that these libraries are used. See [Note:
// Test blocks for Cargo] in `ocamlrep_ocamlpool/test/ocamlpool_test.rs`.
const _: () = {
    #[allow(unused_imports)]
    use anyhow;
    #[allow(unused_imports)]
    use cargo_test_utils;
    #[allow(unused_imports)]
    use tempfile;
};

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use cargo_test_utils::*;
    use tempfile::TempDir;

    #[test]
    fn ocamlrep_test() -> Result<()> {
        let parent = std::path::Path::new("..");
        let tmp_dir = TempDir::with_prefix("ocamlrep_test.")?;
        std::fs::copy(
            parent.join("test_ocamlrep.ml"),
            tmp_dir.path().join("test_ocamlrep.ml"),
        )?;
        let compile_cmd = cmd(
            "ocamlopt.opt",
            &[
                "-verbose",
                "-c",
                "test_ocamlrep.ml",
                "-o",
                "test_ocamlrep_ml.cmx",
            ],
            Some(tmp_dir.path()),
        );
        assert_eq!(run(compile_cmd).map_err(fmt_exit_status_err), Ok(()));
        let link_cmd = cmd(
            "ocamlopt.opt",
            &[
                "-verbose",
                "-o",
                "ocamlrep_test",
                "test_ocamlrep_ml.cmx",
                "-ccopt",
                &("-L".to_owned() + workspace_dir(&["target", build_flavor()]).to_str().unwrap()),
                "-cclib",
                "-ltest_bindings",
                "-cclib",
                "-locamlrep_ocamlpool",
            ],
            Some(tmp_dir.path()),
        );
        assert_eq!(run(link_cmd).map_err(fmt_exit_status_err), Ok(()));
        let ocamlrep_test_cmd = cmd(
            tmp_dir
                .path()
                .join("ocamlrep_test")
                .as_path()
                .to_str()
                .unwrap(),
            &[],
            None,
        );
        assert_eq!(run(ocamlrep_test_cmd).map_err(fmt_exit_status_err), Ok(()));
        tmp_dir.close()?;
        Ok(())
    }
}
