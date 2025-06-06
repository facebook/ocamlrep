// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::ffi::CString;
use std::panic::UnwindSafe;

pub use bumpalo::Bump;
use ocamlrep::Allocator;
use ocamlrep::BlockBuilder;
pub use ocamlrep::FromOcamlRep;
pub use ocamlrep::FromOcamlRepIn;
use ocamlrep::MemoizationCache;
use ocamlrep::ToOcamlRep;
pub use ocamlrep::Value;

unsafe extern "C" {
    fn ocamlpool_enter();
    fn ocamlpool_leave();
    fn ocamlpool_reserve_block(tag: u8, size: usize) -> usize;
    fn caml_failwith(msg: *const i8);
    fn caml_initialize(addr: *mut usize, value: usize);
    static ocamlpool_generation: usize;

    pub fn caml_named_value(name: *const std::ffi::c_char) -> *mut usize;
    pub fn caml_callbackN_exn(closure: usize, n: std::ffi::c_int, args: *const usize) -> usize;
}

pub struct Pool {
    cache: MemoizationCache,
}

impl Pool {
    /// Prepare the ocamlpool library to allocate values directly on the OCaml
    /// runtime's garbage-collected heap.
    ///
    /// # Safety
    ///
    /// The OCaml runtime is not thread-safe, and this function will interact
    /// with it. If any other thread interacts with the OCaml runtime or
    /// ocamlpool library during the lifetime of the `Pool`, undefined behavior
    /// will result.
    #[inline(always)]
    pub unsafe fn new() -> Self {
        unsafe {
            ocamlpool_enter();
        }
        Self {
            cache: MemoizationCache::new(),
        }
    }

    #[inline(always)]
    pub fn add<'a, T: ToOcamlRep + ?Sized>(&'a self, value: &'a T) -> Value<'a> {
        value.to_ocamlrep(self)
    }
}

impl Drop for Pool {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe { ocamlpool_leave() };
    }
}

impl Allocator for Pool {
    #[inline(always)]
    fn generation(&self) -> usize {
        unsafe { ocamlpool_generation }
    }

    #[inline(always)]
    fn block_with_size_and_tag(&self, size: usize, tag: u8) -> BlockBuilder<'_> {
        let ptr = unsafe { ocamlpool_reserve_block(tag, size) as *mut Value<'_> };
        BlockBuilder::new(unsafe { std::slice::from_raw_parts_mut(ptr, size) })
    }

    #[inline(always)]
    fn set_field<'a>(&self, block: &mut BlockBuilder<'a>, index: usize, value: Value<'a>) {
        assert!(index < block.size());
        unsafe {
            caml_initialize(
                (self.block_ptr_mut(block) as *mut usize).add(index),
                value.to_bits(),
            )
        };
    }

    unsafe fn block_ptr_mut<'a>(&self, block: &mut BlockBuilder<'a>) -> *mut Value<'a> {
        block.address() as *mut _
    }

    fn memoized<'a>(
        &'a self,
        ptr: usize,
        size: usize,
        f: impl FnOnce(&'a Self) -> Value<'a>,
    ) -> Value<'a> {
        let bits = self.cache.memoized(ptr, size, || f(self).to_bits());
        // SAFETY: The only memoized values in the cache are those computed in
        // the closure on the previous line. Since f returns Value<'a>, any
        // cached bits must represent a valid Value<'a>,
        unsafe { Value::from_bits(bits) }
    }

    fn add_root<'a, T: ToOcamlRep + ?Sized>(&'a self, value: &'a T) -> Value<'a> {
        self.cache.with_cache(|| value.to_ocamlrep(self))
    }
}

/// Convert the given value to an OCaml value on the OCaml runtime's
/// garbage-collected heap.
///
/// # Safety
///
/// The OCaml runtime is not thread-safe, and this function will interact with
/// it. If any other thread interacts with the OCaml runtime or ocamlpool
/// library during the execution of `to_ocaml`, undefined behavior will result.
///
/// # Panics
///
/// Panics upon attempts to re-enter `to_ocaml`.
#[inline(always)]
pub unsafe fn to_ocaml<T: ToOcamlRep + ?Sized>(value: &T) -> usize {
    let pool = unsafe { Pool::new() };
    let result = pool.add_root(value);
    result.to_bits()
}

/// Catches panics in `f` and raises a OCaml exception of type Failure
/// with the panic message (if the panic was raised with a `&str` or `String`).
pub fn catch_unwind(f: impl FnOnce() -> usize + UnwindSafe) -> usize {
    catch_unwind_with_handler(f, |msg: &str| -> Result<usize, String> { Err(msg.into()) })
}

/// Catches panics in `f` and raises a OCaml exception of type Failure
/// with the panic message (if the panic was raised with a `&str` or `String`).
/// `h` handles panic msg, it may re-raise by returning Err.
pub fn catch_unwind_with_handler(
    f: impl FnOnce() -> usize + UnwindSafe,
    h: impl FnOnce(&str) -> Result<usize, String>,
) -> usize {
    let err = match std::panic::catch_unwind(f) {
        Ok(value) => return value,
        Err(err) => err,
    };
    let msg: &str = if let Some(s) = err.downcast_ref::<&str>() {
        s
    } else if let Some(s) = err.downcast_ref::<String>() {
        s.as_str()
    } else {
        // TODO: Build a smarter message in this case (using panic::set_hook?)
        "Panicked with non-string object"
    };
    match h(msg) {
        Ok(value) => return value,
        Err(err) => unsafe {
            let msg = CString::new(err).unwrap();
            caml_failwith(msg.as_ptr().cast());
        },
    }
    unreachable!();
}

/// Assume that some Pool exists in some parent scope. Since ocamlpool is
/// implemented with statics, we don't need a reference to that pool to write to
/// it.
///
/// Does not preserve sharing of values referred to by multiple references or
/// Rcs (but sharing is preserved for `ocamlrep::rc::RcOc`).
///
/// # Safety
///
/// The OCaml runtime is not thread-safe, and this function will interact with
/// it. If any other thread interacts with the OCaml runtime or ocamlpool
/// library during the execution of this function, undefined behavior will
/// result.
#[inline(always)]
pub unsafe fn add_to_ambient_pool<T: ToOcamlRep>(value: &T) -> usize {
    let fake_pool = Pool {
        cache: MemoizationCache::new(),
    };
    let result = value.to_ocamlrep(&fake_pool).to_bits();
    std::mem::forget(fake_pool);
    result
}

/// Check if an OCaml value is an exception.
///
/// For internal purposes.
pub fn is_exception_result(v: usize) -> bool {
    v & 3 == 2
}

#[macro_export]
macro_rules! ocaml_ffi_fn {
    (fn $name:ident($($param:ident: $ty:ty),+  $(,)?) -> $ret:ty $code:block) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name ($($param: usize,)*) -> usize {
            $crate::catch_unwind(|| {
                fn inner($($param: $ty,)*) -> $ret { $code }
                use $crate::FromOcamlRep;
                $(let $param = <$ty>::from_ocaml($param).unwrap();)*
                let result = inner($($param,)*);
                $crate::to_ocaml(&result)
            })
        }
    };

    (fn $name:ident() -> $ret:ty $code:block) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name (_unit: usize) -> usize {
            $crate::catch_unwind(|| {
                fn inner() -> $ret { $code }
                let result = inner();
                $crate::to_ocaml(&result)
            })
        }
    };

    (fn $name:ident($($param:ident: $ty:ty),*  $(,)?) $code:block) => {
        $crate::ocaml_ffi_fn! {
            fn $name($($param: $ty),*) -> () $code
        }
    };
}

/// Convenience macro for declaring OCaml FFI wrappers.
///
/// Each parameter will be converted from OCaml using `ocamlrep` and the result
/// will be converted to OCaml using `ocamlrep` and allocated on the OCaml GC
/// heap using `ocamlpool`.
///
/// Panics in the function body will be caught and converted to an OCaml
/// exception of type Failure.
#[macro_export]
macro_rules! ocaml_ffi {
    ($(fn $name:ident($($param:ident: $ty:ty),*  $(,)?) $(-> $ret:ty)? $code:block)*) => {
        $($crate::ocaml_ffi_fn! {
            fn $name($($param: $ty),*) $(-> $ret)* $code
        })*
    };
}

#[macro_export]
macro_rules! ocaml_ffi_with_arena_fn {
    (fn $name:ident<$lifetime:lifetime>($arena:ident: $arena_ty:ty, $($param:ident: $ty:ty),+ $(,)?) -> $ret:ty $code:block) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name ($($param: usize,)*) -> usize {
            $crate::catch_unwind(|| {
                use $crate::FromOcamlRepIn;
                let arena = &$crate::Bump::new();
                fn inner<$lifetime>($arena: $arena_ty, $($param: usize,)*) -> $ret {
                    $(let $param = unsafe {
                        <$ty>::from_ocamlrep_in($crate::Value::from_bits($param), $arena).unwrap()
                    };)*
                    $code
                }
                let result = inner(arena, $($param,)*);
                $crate::to_ocaml(&result)
            })
        }
    };

    (fn $name:ident<$lifetime:lifetime>($arena:ident: $arena_ty:ty $(,)?) -> $ret:ty $code:block) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name (_unit: usize) -> usize {
            $crate::catch_unwind(|| {
                fn inner<$lifetime>($arena: $arena_ty) -> $ret { $code }
                let arena = &$crate::Bump::new();
                let result = inner(arena);
                $crate::to_ocaml(&result)
            })
        }
    };

    (fn $name:ident<$lifetime:lifetime>($($param:ident: $ty:ty),* $(,)?) $code:block) => {
        $crate::ocaml_ffi_with_arena_fn! {
            fn $name<$lifetime>($($param: $ty),*) -> () $code
        }
    };
}

/// Convenience macro for declaring OCaml FFI wrappers which use an arena to
/// allocate the arguments and return value.
///
/// FFI functions declared with this macro must declare exactly one lifetime
/// parameter. The function's first value parameter must be a reference to a
/// `bumpalo::Bump` arena with that lifetime:
///
/// ```
/// ocaml_ffi_with_arena! {
///     fn swap_str_pair<'a>(arena: &'a Bump, pair: (&'a str, &'a str)) -> (&'a str, &'a str) {
///         (pair.1, pair.0)
///     }
/// }
/// ```
///
/// An OCaml extern declaration for this function would look like this:
///
/// ```
/// external swap_str_pair : string * string -> string * string = "swap_str_pair"
/// ```
///
/// Note that no parameter for the arena appears on the OCaml side--it is
/// constructed on the Rust side and lives only for the duration of one FFI
/// call.
///
/// Each (non-arena) parameter will be converted from OCaml using
/// `ocamlrep::FromOcamlRepIn`, and allocated in the given arena (if its
/// `FromOcamlRepIn` implementation makes use of the arena).
///
/// The return value (which may be allocated in the given arena, if convenient)
/// will be converted to OCaml using `ocamlrep::ToOcamlRep`. The converted OCaml
/// value will be allocated on the OCaml heap using `ocamlpool`.
///
/// Panics in the function body will be caught and converted to an OCaml
/// exception of type `Failure`.
#[macro_export]
macro_rules! ocaml_ffi_with_arena {
    ($(fn $name:ident<$lifetime:lifetime>($($param:ident: $ty:ty),* $(,)?) $(-> $ret:ty)? $code:block)*) => {
        $($crate::ocaml_ffi_with_arena_fn! {
            fn $name<$lifetime>($($param: $ty),*) $(-> $ret)* $code
        })*
    };
}

#[macro_export]
macro_rules! ocaml_ffi_arena_result_fn {
    (fn $name:ident<$lifetime:lifetime>($arena:ident: $arena_ty:ty, $($param:ident: $ty:ty),+ $(,)?) -> $ret:ty $code:block) => {
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn $name ($($param: usize,)*) -> usize {
            $crate::catch_unwind(|| {
                fn inner<$lifetime>($arena: $arena_ty, $($param: $ty,)*) -> $ret {
                    $code
                }
                use $crate::FromOcamlRep;
                $(let $param = <$ty>::from_ocaml($param).unwrap();)*
                let arena = &$crate::Bump::new();
                let result = inner(arena, $($param,)*);
                $crate::to_ocaml(&result)
            })
        }
    };
}

/// Convenience macro for declaring OCaml FFI wrappers which use an arena to
/// just the return value.
///
/// FFI functions declared with this macro must declare exactly one lifetime
/// parameter. The function's first value parameter must be a reference to a
/// `bumpalo::Bump` arena with that lifetime:
///
/// ```
/// ocaml_ffi_arena_result! {
///     fn parse_example<'a>(arena: &'a Bump, text: String) -> ('a str, &'a str) {
///         /* copy parts of text into arena while parsing */
///         (pair.1, pair.0)
///     }
/// }
/// ```
///
/// An OCaml extern declaration for this function would look like this:
///
/// ```
/// external parse_example : string -> string * string = "parse_example"
/// ```
///
/// Note that no parameter for the arena appears on the OCaml side--it is
/// constructed on the Rust side and lives only for the duration of one FFI
/// call.
///
/// Each parameter after the arena parameter will be converted from OCaml using
/// `ocamlrep` and passed in as an owned value.
///
/// The return value (which may be allocated in the given arena, if convenient)
/// will be converted to OCaml using `ocamlrep::ToOcamlRep`. The converted OCaml
/// value will be allocated on the OCaml heap using `ocamlpool`.
///
/// Panics in the function body will be caught and converted to an OCaml
/// exception of type `Failure`.
#[macro_export]
macro_rules! ocaml_ffi_arena_result {
    ($(fn $name:ident<$lifetime:lifetime>($($param:ident: $ty:ty),* $(,)?) $(-> $ret:ty)? $code:block)*) => {
        $($crate::ocaml_ffi_arena_result_fn! {
            fn $name<$lifetime>($($param: $ty),*) $(-> $ret)* $code
        })*
    };
}

#[macro_export]
macro_rules! ocaml_registered_function_fn {
    // This needs to be first, as macro matching is linear.
    //
    // caml_callback_exn works as it directly calls into the implemented OCaml functions.
    //
    // caml_callback{2,3,N}_exn don't work, as they go through caml_apply2,
    // caml_apply3 etc. which for some reason crashes!
    //
    // TODO: FIgure out how to make caml_apply2 and friends not crash, and remove the below rule.
    ($ocaml_name:expr_2021, fn $name:ident($param1:ident: $ty1:ty, $($params:ident: $ty:ty),+  $(,)?) -> $ret:ty) => {
        compile_error!("We don't support functions with more than one parameter.");
    };

    ($ocaml_name:expr_2021, fn $name:ident($($param:ident: $ty:ty),+  $(,)?) -> $ret:ty) => {
        #[unsafe(no_mangle)]
        pub unsafe fn $name ($($param: $ty,)*) -> $ret {
            use std::sync::OnceLock;
            static FN: OnceLock<usize> = OnceLock::new();
            let the_function_to_call = *FN.get_or_init(|| {
                let the_function_to_call_name = std::ffi::CString::new($ocaml_name).expect("string contained null byte");
                let the_function_to_call = $crate::caml_named_value(the_function_to_call_name.as_ptr());
                if the_function_to_call.is_null() {
                    panic!("Could not find function. Use Callback.register");
                }
                *the_function_to_call
            });
            let args_to_function: Vec<usize> = vec![$($crate::to_ocaml(&$param),)*];
            let args_to_function_ptr: *const usize = args_to_function.as_ptr();
            let result = $crate::caml_callbackN_exn(the_function_to_call, args_to_function.len().try_into().unwrap(), args_to_function_ptr);
            if $crate::is_exception_result(result) {
                panic!("OCaml function threw an unknown exception");
            }
            let result = <$ret>::from_ocaml(result).unwrap();
            result
        }
    };

    ($ocaml_name:expr_2021, fn $name:ident() -> $ret:ty) => {
        unsafe fn $name() -> $ret {
            $crate::ocaml_registered_function_fn!(
                $ocaml_name,
                fn inner(_unit: ()) -> $ret
            );
            inner(())
        }
    };

    ($ocaml_name:expr_2021, fn $name:ident($($param:ident: $ty:ty),*  $(,)?)) => {
        $crate::ocaml_registered_function_fn!(
            $ocaml_name,
            fn $name($($param: $ty),*) -> ()
        );
    };
}

/// Convenience macro for declaring Rust FFI wrappers around OCaml-defined functions.
///
/// Each parameter will be converted to OCaml using `ocamlrep` and allocated on
/// the OCaml GC heap using `ocamlpool`. The result will be converted from OCaml
/// using `ocamlrep`.
///
/// Exceptions in OCaml will be caught and converted to a Rust panic. The panic
/// will not contain useful information due to the limitations of deserializing
/// arbitrary OCaml exceptions.
#[macro_export]
macro_rules! ocaml_registered_function {
    ($(fn $name:ident($($param:ident: $ty:ty),*  $(,)?) $(-> $ret:ty)?;)*) => {
        $($crate::ocaml_registered_function_fn!(
            stringify!($name),
            fn $name($($param: $ty),*) $(-> $ret)*
        );)*
    };
}
