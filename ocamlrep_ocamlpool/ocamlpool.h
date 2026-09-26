/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
#ifndef OCAMLPOOL_H
#define OCAMLPOOL_H

#include <caml/mlvalues.h>

/* OCamlpool sections
 * ===========================================================================
 *
 * Inside the section, the OCaml heap will be in an invalid state.
 * OCaml runtime functions should not be called.
 *
 * Since the GC will never run while in an OCaml pool section,
 * it is safe to keep references to OCaml values as long as these does not
 * outlive the section.
 */

void ocamlpool_enter(void);
void ocamlpool_leave(void);

/* OCaml value allocations
 * ===========================================================================
 *
 * Reserve OCaml memory when inside ocamlpool section.
 */

value ocamlpool_reserve_block(tag_t tag, mlsize_t words);

// Generation counter identifying pool sections, for `RcOc`'s memoization cache
// (see `ocamlrep_ocamlpool::Pool::generation`). Bumped by `ocamlpool_leave`.
extern uintnat ocamlpool_generation;

#endif /* !defined(OCAMLPOOL_H) */
