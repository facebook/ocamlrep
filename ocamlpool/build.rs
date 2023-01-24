// Assume an opam environment (`eval "$(opam env --switch=default
// --set-switch)"`) then to find the prevailing standard library caml
// headers, `OCAMLLIB=$(ocamlopt.opt -config | grep standard_library:
// | awk '{ print $2 }')`.

fn main() {
    cc::Build::new()
        .include(env!("OCAMLLIB"))
        .file("ocamlpool.c")
        .compile("ocamlpool");
}
