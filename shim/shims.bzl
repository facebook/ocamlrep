# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

# @lint-ignore FBCODEBZLADDLOADS

_SELECT_TYPE = type(select({"DEFAULT": []}))

def is_select(thing):
    return type(thing) == _SELECT_TYPE

def rust_library(
        rustc_flags = [],
        deps = [],
        named_deps = None,
        os_deps = None,
        test_deps = None,
        test_env = None,
        autocargo = None,
        mapped_srcs = {},
        visibility = ["PUBLIC"],
        **kwargs):
    _unused = (test_deps, test_env, named_deps, autocargo)  # @unused
    deps = _maybe_select_map(deps, _fix_deps)
    mapped_srcs = _maybe_select_map(mapped_srcs, _fix_mapped_srcs)
    if os_deps:
        deps += _select_os_deps(_fix_dict_deps(os_deps))

    # @lint-ignore BUCKLINT: avoid "native is forbidden in fbcode"
    native.rust_library(
        rustc_flags = rustc_flags + [_CFG_BUCK_OSS_BUILD],
        deps = deps,
        visibility = visibility,
        **kwargs
    )

def rust_binary(
        rustc_flags = [],
        deps = [],
        unittests = None,
        allocator = None,
        default_strip_mode = None,
        visibility = ["PUBLIC"],
        **kwargs):
    _unused = (unittests, allocator, default_strip_mode)  # @unused
    deps = _maybe_select_map(deps, _fix_deps)

    # @lint-ignore BUCKLINT: avoid "native is forbidden in fbcode"
    native.rust_binary(
        rustc_flags = rustc_flags + [_CFG_BUCK_OSS_BUILD],
        deps = deps,
        visibility = visibility,
        **kwargs
    )

# Configuration that is used when building open source using Buck2 as
# the build system. E.g. not applied either internally, or when using
# Cargo to build the open source code. At the moment of writing,
# mostly used to disable jemalloc.
_CFG_BUCK_OSS_BUILD = "--cfg=buck_oss_build"

def _maybe_select_map(v, mapper):
    if is_select(v):
        return select_map(v, mapper)
    return mapper(v)

def _select_os_deps(xss: [(
    "string",
    ["string"],
)]) -> "selector":
    d = {
        "prelude//os:" + os: xs
        for os, xs in xss
    }
    d["DEFAULT"] = []
    return select(d)

def _fix_dict_deps(xss: [(
    "string",
    ["string"],
)]) -> [(
    "string",
    ["string"],
)]:
    return [
        (k, _fix_deps(xs))
        for k, xs in xss
    ]

def _fix_mapped_srcs(xs: {"string": "string"}):
    # For reasons, this is source -> file path, which is the opposite
    # of what it should be.
    return {_fix_dep(k): v for (k, v) in xs.items()}

def _fix_deps(xs: ["string"]) -> ["string"]:
    return filter(None, map(_fix_dep, xs))

def _fix_dep(x: "string") -> [
    None,
    "string",
]:
    if x.startswith("fbcode//common/ocaml/interop/"):
        return "root//" + x.removeprefix("fbcode//common/ocaml/interop/")
    else:
        return x
