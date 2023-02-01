/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
#define CAML_NAME_SPACE

#include <stdio.h>

#define CAML_INTERNALS

#include "ocamlpool.h"

#include <caml/gc.h>
#include <caml/memory.h>
#include <caml/misc.h>
#include <caml/shared_heap.h>

/* Global state
 * ===========================================================================
 */

/* 1 iff we are inside an ocamlpool section */
static int ocamlpool_in_section = 0;

/* For sanity checks, caml_young_ptr is copied when entering the section.
 * While inside the section, we check that the value has not changed as this
 * would result in difficult to track bugs. */
static void* ocamlpool_sane_young_ptr;

/* Sanity checks
 * ===========================================================================
 *
 * Contracts checking that the invariants are maintained inside the library
 * and that the API is used correctly.
 */

static void abort_unless(int x, const char* message) {
  if (!x) {
    fprintf(stderr, "OCamlPool invariant violation (%d): %s\n", x, message);
    abort();
  }
}

#ifndef OCAMLPOOL_NO_ASSERT

static void ocamlpool_assert(int x, const char* message) {
  abort_unless(x, message);
}

#else

static void ocamlpool_assert(int x, const char* message) {
  (void)x;
}

#endif

static void assert_in_section(void) {
  ocamlpool_assert(
      ocamlpool_in_section == 1 && ocamlpool_sane_young_ptr == caml_young_ptr,
      "assert_in_section");
}

static void assert_out_of_section(void) {
  ocamlpool_assert(ocamlpool_in_section == 0, "assert_out_of_section");
}

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

void ocamlpool_enter(void) {
  assert_out_of_section();

  ocamlpool_in_section = 1;
  ocamlpool_sane_young_ptr = caml_young_ptr;
}

void ocamlpool_leave(void) {
  assert_in_section();

  ocamlpool_in_section = 0;

  caml_process_pending_actions();
}

/* OCaml value allocations
 * ===========================================================================
 *
 * A fast way to reserve OCaml memory when inside ocamlpool section.
 */
value ocamlpool_reserve_block(tag_t tag, mlsize_t wosize) {
  caml_domain_state * d = Caml_state;
  value *p = caml_shared_try_alloc(d->shared_heap, wosize, tag, 0);
  d->allocated_words += Whsize_wosize(wosize);

  if (p == NULL) {
    /* TODO: handle this case... */
    abort();
  }

  Hd_hp(p) = Make_header(wosize, tag, caml_global_heap_state.MARKED);
  return Val_hp(p);
}
