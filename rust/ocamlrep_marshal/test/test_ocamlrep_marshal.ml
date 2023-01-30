(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 *)

let assert_eq v =
  let ocaml_marshaled = Marshal.to_string v [] in
  let rust_marshaled = Ocamlrep_marshal_ffi.to_string v [] in
  if not (String.equal rust_marshaled ocaml_marshaled) then begin
    Printf.printf
      "OCaml Marshal output does not match Rust ocamlrep_marshal output:\n%!";
    Printf.printf "ocaml:\t%S\n%!" ocaml_marshaled;
    Printf.printf "rust:\t%S\n%!" rust_marshaled
  end

let test_round_trip show (x : 'a) =
  let bytes = Ocamlrep_marshal_ffi.to_string x [] in
  let y : 'a = Ocamlrep_marshal_ffi.from_string bytes 0 in
  let _ = Printf.printf "y = %s\n" (show y) in
  assert (x = y)

let show_pair_int_int (x, y) = Printf.sprintf "(%d, %d)" x y

let show_pair_opt_int_string (x, y) =
  let xx = match x with
  | Some i -> Printf.sprintf "Some %i" i
  | None -> "None"
 in Printf.sprintf "(%s, %S)" xx y

let show_float_list xs = "[" ^ String.concat "; " ( List.map (fun x -> Printf.sprintf "%f" x) xs) ^ "]"

let show_float_array x = show_float_list (Array.to_list x)

(* A type of non-empty trees of strings. *)
type tree = [
  |`Node of string * tree list
] [@@ocamlformat "disable"]

(* [print tree] produces a rendering of [tree]. *)
let rec print_tree
          ?(pad : (string * string)= ("", ""))
          (tree : tree) : string list =
  let pd, pc = pad in
  match tree with
  | `Node (tag, cs) ->
     let n = List.length cs - 1 in
     Printf.sprintf "%s%s" pd tag :: List.concat (List.mapi (
         fun i c ->
         let pad = let enable_utf8 = true in
           if enable_utf8 then
             (pc ^ (if i = n then "\u{02517} " else "\u{02523} "),
              pc ^ (if i = n then " " else "\u{02503} "))
           else
             (pc ^ (if i = n then "`-- " else "|-- "),
              pc ^ (if i = n then "    " else "|   "))
         in print_tree ~pad c
       ) cs) [@@ocamlformat "disable"]

(* [show_tree] produces a string of [t]. *)
let show_tree t =
  Printf.sprintf "\n%s\n" (String.concat "\n" (print_tree t))

(* An example tree. *)
let tree =
  `Node ("life"
        , [
            `Node ("domain", [
                      `Node ("kingdom", [
                                `Node ("phylum", [])]);
                      `Node ("class", []);
                      `Node ("order", [])
              ])
          ;  `Node ("family", [])
          ]) [@@ocamlformat "disable"]

let test_sharing () =
  let s = "str" in
  let inner = (s, s) in
  let outer = (inner, inner) in
  begin
    let marshaled = Ocamlrep_marshal_ffi.to_string outer [] in
    match Ocamlrep_marshal_ffi.from_string marshaled 0 with
    | (((s1, s2) as tup1), tup2) ->
      assert (tup1 == tup2);
      assert (s1 == s2);
      ()
  end;
  let marshaled = Ocamlrep_marshal_ffi.to_string outer [Marshal.No_sharing] in
  match Ocamlrep_marshal_ffi.from_string marshaled 0 with
  | (((s1, s2) as tup1), (s3, s4)) as tup2 ->
    assert (not (tup1 == tup2));
    assert (not (s1 == s2));
    assert (not (s2 == s3));
    assert (not (s3 == s4));
    ()

let () =
  print_endline "[ocamlrep_marshal_test][info]: start";

  assert_eq 'c';
  assert_eq 5;
  assert_eq 3.14;
  assert_eq (-5);
  assert_eq (-3.14);
  assert_eq (3, 3);
  assert_eq "a";
  assert_eq (Some 42, "foo");

  test_round_trip (fun c -> String.make 1 c) 'c';
  test_round_trip string_of_float 3.14;
  test_round_trip string_of_int (-5);
  test_round_trip string_of_float (-3.14);
  test_round_trip show_pair_int_int (3, 3);
  test_round_trip (Printf.sprintf "%S") "a";
  test_round_trip show_pair_opt_int_string (Some 42, "foo");
  test_round_trip show_float_array (Array.make 3 3.14);
  test_round_trip show_float_array (Array.make 3 (-3.14));
  test_round_trip show_tree tree;

  test_sharing ();

  print_endline "[ocamlrep_marshal_test][info]: finish";

  ()
