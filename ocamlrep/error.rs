// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::error::Error;
use std::fmt;
use std::num::TryFromIntError;
use std::str::Utf8Error;

/// Returned by
/// [`OcamlRep::from_ocamlrep`](trait.OcamlRep.html#tymethod.from_ocamlrep) when
/// the given [`Value`](struct.Value.html) cannot be converted to a Rust value
/// of the expected type.
#[derive(Debug, PartialEq)]
pub enum FromError {
    BadUtf8(Utf8Error),
    BlockTagOutOfRange { max: u8, actual: u8 },
    ErrorInField(usize, Box<FromError>),
    ExpectedBlock(isize),
    ExpectedBlockTag { expected: u8, actual: u8 },
    ExpectedBool(isize),
    ExpectedChar(isize),
    ExpectedInt(usize),
    Expected63BitInt(isize),
    ExpectedUnit(isize),
    ExpectedZeroTag(u8),
    IntOutOfRange(TryFromIntError),
    NullaryVariantTagOutOfRange { max: usize, actual: isize },
    WrongBlockSize { expected: usize, actual: usize },
    UnexpectedCustomOps { expected: usize, actual: usize },
}

impl std::convert::From<TryFromIntError> for FromError {
    fn from(error: TryFromIntError) -> Self {
        FromError::IntOutOfRange(error)
    }
}

impl std::convert::From<Utf8Error> for FromError {
    fn from(error: Utf8Error) -> Self {
        FromError::BadUtf8(error)
    }
}

impl fmt::Display for FromError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use FromError::*;
        match self {
            BadUtf8(_) => write!(f, "Invalid UTF-8"),
            BlockTagOutOfRange { max, actual } => {
                write!(f, "Expected tag value <= {max}, but got {actual}")
            }
            ErrorInField(idx, _) => write!(f, "Failed to convert field {idx}"),
            ExpectedBlock(x) => write!(f, "Expected block, but got integer value {x}"),
            ExpectedBlockTag { expected, actual } => {
                write!(f, "Expected block with tag {expected}, but got {actual}",)
            }
            ExpectedBool(x) => write!(f, "Expected bool, but got {x}"),
            ExpectedChar(x) => write!(f, "Expected char, but got {x}"),
            ExpectedInt(x) => {
                write!(f, "Expected integer value, but got block pointer {x:p}")
            }
            Expected63BitInt(x) => write!(
                f,
                "Expected integer value between -2^(n-2) and 2^(n-2)-1, where n is the number of bits in isize, but got {x}",
            ),
            ExpectedUnit(x) => write!(f, "Expected (), but got {x}"),
            ExpectedZeroTag(x) => write!(
                f,
                "Expected block with tag 0 (tuple, record, cons cell, etc), but got tag value {x}",
            ),
            IntOutOfRange(_) => write!(f, "Integer value out of range"),
            NullaryVariantTagOutOfRange { max, actual } => write!(
                f,
                "Expected nullary variant tag, where 0 <= tag <= {max}, but got {actual}",
            ),
            WrongBlockSize { expected, actual } => write!(
                f,
                "Expected block of size {expected}, but got size {actual}",
            ),
            UnexpectedCustomOps { expected, actual } => write!(
                f,
                "Expected custom operations struct address 0x{expected:x}, but got address 0x{actual:x}",
            ),
        }
    }
}

impl Error for FromError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use FromError::*;
        match self {
            BadUtf8(err) => Some(err),
            ErrorInField(_, err) => Some(err),
            IntOutOfRange(err) => Some(err),
            BlockTagOutOfRange { .. }
            | ExpectedBlock(..)
            | ExpectedBlockTag { .. }
            | ExpectedBool(..)
            | ExpectedChar(..)
            | ExpectedInt(..)
            | Expected63BitInt(..)
            | ExpectedUnit(..)
            | ExpectedZeroTag(..)
            | NullaryVariantTagOutOfRange { .. }
            | WrongBlockSize { .. }
            | UnexpectedCustomOps { .. } => None,
        }
    }
}
