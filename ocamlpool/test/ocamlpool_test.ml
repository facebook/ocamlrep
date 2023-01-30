(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *)

external test : unit -> unit = "test"

(* Although the test is entirely written in rust,
 * we need to build the rust code with the ocaml runtime dependencies
 * in order to allocate memory for ocaml. Calling rust from ocaml
 * is a good way of ensuring this dependecy is built.
 *)
let () = begin
    print_endline "[ocamlpool_test][info]: start";
    test ();
    print_endline "[ocamlpool_test][info]: finish"
end
