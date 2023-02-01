/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
#ifndef OCAMLPOOL_H
#define OCAMLPOOL_H

#include <caml/mlvalues.h>

/*
 * FIXME: The current system always maintain the heap in a well formed state,
 *        making the current pool look like a string to the OCaml GC and
 *        fragmenting it during allocation.
 *        This is not necessary, it should be correct to just keep a pointer
 *        and the size of the unallocated area while in the section and make
 *        it look like a string when leaving the section.
 * FIXME: The current chunking system might be incorrect if the incremental
 *        scan stops in the middle of the unallocated chunk.
 *        To prevent that, this chunk is marked as non-scannable (a string),
 *        but I should double check the behavior of Obj.truncate.
 * FIXME: For now, the chunk is just strongly referenced during used and
 *        unreferenced when released.
 *        Improvements:
 *        - make it weak so that OCaml GC can grab it under memory pressure
 *        - add it to freelist on release, so that memory can be reclaimed
 *          before next GC.
 */

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
 * A fast to reserve OCaml memory when inside ocamlpool section.
 */

value ocamlpool_reserve_block(tag_t tag, mlsize_t words);

#endif /*!OCAMLPOOL_H*/
