// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use ocamlrep::*;
use ocamlrep_custom::CustomOperations;
use serde::Deserialize;
use serde::Serialize;

/// `Int64.t` conversion guide.
///
/// Communicates that when the wrapped i64 is converted to an OCaml
/// representation, the type `Int64.t` (representing a boxed 64-bit integer)
/// should be used rather than the type `int` (which, on 64-bit architectures,
/// is an unboxed 63-bit integer).
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[derive(Serialize, Deserialize)]
pub struct Int64(pub i64);

unsafe extern "C" {
    static mut caml_int64_ops: CustomOperations;
}

impl From<i64> for Int64 {
    fn from(x: i64) -> Self {
        Self(x)
    }
}

impl From<Int64> for i64 {
    fn from(x: Int64) -> i64 {
        x.0
    }
}

impl ToOcamlRep for Int64 {
    fn to_ocamlrep<'a, A: Allocator>(&'a self, alloc: &'a A) -> Value<'a> {
        let mut block = alloc.block_with_size_and_tag(2, CUSTOM_TAG);
        alloc.set_field(&mut block, 0, unsafe {
            Value::from_bits(std::ptr::addr_of!(caml_int64_ops) as usize)
        });
        alloc.set_field(&mut block, 1, unsafe { Value::from_bits(self.0 as usize) });
        block.build()
    }
}

impl FromOcamlRep for Int64 {
    fn from_ocamlrep(value: Value<'_>) -> Result<Self, FromError> {
        let block = from::expect_block_with_size_and_tag(value, 2, CUSTOM_TAG)?;
        Ok(Self(block[1].to_bits() as i64))
    }
}

impl<'a> FromOcamlRepIn<'a> for Int64 {
    fn from_ocamlrep_in(value: Value<'_>, _alloc: &'a ocamlrep::Bump) -> Result<Self, FromError> {
        Self::from_ocamlrep(value)
    }
}
