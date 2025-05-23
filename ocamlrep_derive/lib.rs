// Copyright (c) Meta Platforms, Inc. and affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

#![recursion_limit = "128"]

use proc_macro2::TokenStream;
use quote::quote;
use syn::Attribute;
use syn::Meta;
use syn::NestedMeta;
use synstructure::BindingInfo;
use synstructure::VariantInfo;
use synstructure::decl_derive;

// The rust_to_ocaml_attr crate provides the rust_to_ocaml attribute macro,
// which is intended to be consumed by the rust_to_ocaml codegen tool. It
// doesn't currently control the behavior of any derived ocamlrep trait impls.
//
// Unfortunately, rust_to_ocaml_attr does not strip the attribute macro from
// positions where attribute macros are not allowed (like field definitions).
// The easiest way to do that is to ask proc_macro_derive to do it, but that
// requires the use of a derive macro.
//
// Since all of the types we are interested in passing to rust_to_ocaml
// implement an ocamlrep trait, ask proc_macro_derive (via decl_derive) to strip
// rust_to_ocaml attributes when deriving ocamlrep traits.
//
// Even with this stripping, the rust_to_ocaml_attr crate is still required to
// strip the attribute from type aliases, which cannot use derive macros.
decl_derive!([ToOcamlRep, attributes(rust_to_ocaml, ocamlrep)] => derive_to_ocamlrep);
decl_derive!([FromOcamlRep, attributes(rust_to_ocaml, ocamlrep)] => derive_from_ocamlrep);
decl_derive!([FromOcamlRepIn, attributes(rust_to_ocaml, ocamlrep)] => derive_from_ocamlrep_in);

fn workaround_non_local_def(impl_block: TokenStream) -> TokenStream {
    // We need to upgrade synstructure to remove this warning, but doing so will also require upgrading
    // to syn2 and rewriting to handle the API changes.
    quote! {
        #[allow(non_local_definitions)]
        #impl_block
    }
}

fn derive_to_ocamlrep(mut s: synstructure::Structure<'_>) -> TokenStream {
    // remove #[ocamlrep(skip)]
    for variant in s.variants_mut() {
        variant.filter(|bi| !has_ocamlrep_skip_attr(&bi.ast().attrs));
    }

    // By default, if you are deriving an impl of trait Foo for generic type
    // X<T>, synstructure will add Foo as a bound not only for the type
    // parameter T, but also for every type which appears as a field in X. This
    // is not necessary for our use case--we can just require that the type
    // parameters implement our trait.
    s.add_bounds(synstructure::AddBounds::Generics);

    let to_body = to_ocamlrep_body(&s);
    workaround_non_local_def(s.gen_impl(quote! {
        gen impl ::ocamlrep::ToOcamlRep for @Self {
            fn to_ocamlrep<'__ocamlrep_derive_allocator, Alloc: ::ocamlrep::Allocator>(
                &'__ocamlrep_derive_allocator self,
                arena: &'__ocamlrep_derive_allocator Alloc,
            ) -> ::ocamlrep::Value<'__ocamlrep_derive_allocator> {
                use ::ocamlrep::Allocator;
                match *self { #to_body }
            }
        }
    }))
}

fn derive_from_ocamlrep(mut s: synstructure::Structure<'_>) -> TokenStream {
    s.add_bounds(synstructure::AddBounds::Generics);

    let from_body = from_ocamlrep_body(&mut s);
    workaround_non_local_def(s.gen_impl(quote! {
        gen impl ::ocamlrep::FromOcamlRep for @Self {
            fn from_ocamlrep(value: ::ocamlrep::Value<'_>) -> ::std::result::Result<Self, ::ocamlrep::FromError> {
                use ::ocamlrep::FromOcamlRep;
                #from_body
            }
        }
    }))
}

fn derive_from_ocamlrep_in(mut s: synstructure::Structure<'_>) -> TokenStream {
    s.add_bounds(synstructure::AddBounds::Generics);

    if s.ast().generics.lifetimes().next().is_none() {
        s.add_bounds(synstructure::AddBounds::None);
        let tparams = s.ast().generics.type_params();
        let tparams_implement_from_ocamlrep: TokenStream = tparams
            .map(|t| quote!(#t : ::ocamlrep::FromOcamlRep,))
            .collect();
        let from_body = from_ocamlrep_body(&mut s);
        return workaround_non_local_def(s.gen_impl(quote! {
            gen impl<'__ocamlrep_derive_allocator> ::ocamlrep::FromOcamlRepIn<'__ocamlrep_derive_allocator> for @Self
            where #tparams_implement_from_ocamlrep
            {
                fn from_ocamlrep_in(
                    value: ::ocamlrep::Value<'_>,
                    alloc: &'__ocamlrep_derive_allocator ::ocamlrep::Bump,
                ) -> ::std::result::Result<Self, ::ocamlrep::FromError> {
                    use ::ocamlrep::FromOcamlRep;
                    #from_body
                }
            }
        }));
    }

    // Constrain the lifetime of `'__ocamlrep_derive_allocator` to be equal to
    // any declared lifetimes. This is so that we can reference the lifetime
    // parameter to `FromOcamlRepIn` without requiring implementors to use a
    // certain name for their lifetime parameter.
    let lifetimes = s.ast().generics.lifetimes();
    let lifetimes: TokenStream = lifetimes
        .map(|l| {
            quote! {
                '__ocamlrep_derive_allocator : #l,
                #l : '__ocamlrep_derive_allocator,
            }
        })
        .collect();
    let tparams = s.ast().generics.type_params();
    let tparams_implement_trivialdrop: TokenStream = tparams
        .map(|t| quote!(#t : ::arena_trait::TrivialDrop,))
        .collect();

    let from_in_body = from_ocamlrep_in_body(&mut s);
    workaround_non_local_def(s.gen_impl(quote! {
        gen impl<'__ocamlrep_derive_allocator> ::ocamlrep::FromOcamlRepIn<'__ocamlrep_derive_allocator> for @Self
        where
            #tparams_implement_trivialdrop #lifetimes
        {
            fn from_ocamlrep_in(
                value: ::ocamlrep::Value<'_>,
                alloc: &'__ocamlrep_derive_allocator ::ocamlrep::Bump,
            ) -> ::std::result::Result<Self, ::ocamlrep::FromError> {
                use ::ocamlrep::FromOcamlRepIn;
                #from_in_body
            }
        }
    }))
}

fn to_ocamlrep_body(s: &synstructure::Structure<'_>) -> TokenStream {
    match &s.ast().data {
        syn::Data::Struct(struct_data) => struct_to_ocamlrep(s, struct_data),
        syn::Data::Enum(_) => enum_to_ocamlrep(s, collect_enum_variants(s)),
        syn::Data::Union(_) => panic!("untagged unions not supported"),
    }
}

fn from_ocamlrep_body(s: &mut synstructure::Structure<'_>) -> TokenStream {
    match &s.ast().data {
        syn::Data::Struct(struct_data) => struct_from_ocamlrep(s, struct_data, false),
        syn::Data::Enum(_) => enum_from_ocamlrep(collect_enum_variants(s), false),
        syn::Data::Union(_) => panic!("untagged unions not supported"),
    }
}

fn from_ocamlrep_in_body(s: &mut synstructure::Structure<'_>) -> TokenStream {
    match &s.ast().data {
        syn::Data::Struct(struct_data) => struct_from_ocamlrep(s, struct_data, true),
        syn::Data::Enum(_) => enum_from_ocamlrep(collect_enum_variants(s), true),
        syn::Data::Union(_) => panic!("untagged unions not supported"),
    }
}

fn struct_to_ocamlrep(
    s: &synstructure::Structure<'_>,
    struct_data: &syn::DataStruct,
) -> TokenStream {
    match struct_data.fields {
        syn::Fields::Unit => {
            // Represent unit structs with unit.
            s.each_variant(|_| quote! { arena.add(&()) })
        }
        syn::Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
            // For the newtype pattern (a tuple struct with a single field),
            // don't allocate a block--just use the inner value directly.
            s.each(|bi| quote! { arena.add(#bi) })
        }
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
            // Otherwise, we have a record-like struct or a tuple struct. Both
            // are represented with a block.
            s.each_variant(|v| allocate_block(v, 0))
        }
    }
}

/// Fetch all the parameters from ocamlrep attributes:
///   #[ocamlrep(foo, bar), ocamlrep(baz)]
/// yields:
///   [foo, bar, baz]
fn parse_ocamlrep_attr(attrs: &[Attribute]) -> Option<Vec<NestedMeta>> {
    let mut res = None;
    for attr in attrs {
        let meta = attr.parse_meta().unwrap();
        match meta {
            Meta::Path(_) => {
                // #[foo]
            }
            Meta::List(list) => {
                // #[foo(bar)]
                if list.path.is_ident("ocamlrep") {
                    res.get_or_insert_with(Vec::new).extend(list.nested);
                }
            }
            Meta::NameValue(_) => {
                // #[foo = bar]
            }
        }
    }

    res
}

/// Returns true if the attributes contain an `#[ocamlrep(skip)]`
fn has_ocamlrep_skip_attr(attrs: &[Attribute]) -> bool {
    if let Some(ocamlrep) = parse_ocamlrep_attr(attrs) {
        for rep in ocamlrep {
            match rep {
                NestedMeta::Meta(Meta::Path(path)) if path.is_ident("skip") => {
                    return true;
                }
                _ => {}
            }
        }
    }

    false
}

fn struct_from_ocamlrep(
    s: &mut synstructure::Structure<'_>,
    struct_data: &syn::DataStruct,
    from_in: bool,
) -> TokenStream {
    let variant = &mut s.variants_mut()[0];
    match struct_data.fields {
        syn::Fields::Unit => {
            let constructor = variant.construct(|_, _| quote!(unreachable!()));
            quote! { <()>::from_ocamlrep(value)?; Ok(#constructor) }
        }
        syn::Fields::Unnamed(ref fields) if fields.unnamed.len() == 1 => {
            let constructor = variant.construct(|field, _| {
                let ty = &field.ty;
                if from_in {
                    quote! { <#ty>::from_ocamlrep_in(value, alloc)? }
                } else {
                    quote! { <#ty>::from_ocamlrep(value)? }
                }
            });
            quote! { Ok(#constructor) }
        }
        syn::Fields::Named(_) | syn::Fields::Unnamed(_) => {
            let mut binding = 0;
            let constructor = variant.construct(|field, _| {
                if has_ocamlrep_skip_attr(&field.attrs) {
                    quote!(::std::default::Default::default())
                } else {
                    let idx = binding;
                    binding += 1;
                    field_constructor(idx, from_in)
                }
            });
            quote! {
                let block = ::ocamlrep::from::expect_tuple(value, #binding)?;
                Ok(#constructor)
            }
        }
    }
}

struct EnumVariants<'a> {
    nullary_variants: Vec<(&'a synstructure::VariantInfo<'a>, isize)>,
    block_variants: Vec<(&'a synstructure::VariantInfo<'a>, isize)>,
}

fn collect_enum_variants<'a>(s: &'a synstructure::Structure<'_>) -> EnumVariants<'a> {
    // For tagging purposes, variant constructors of zero arguments are numbered
    // separately from variant constructors of one or more arguments, so we need
    // to count them separately to learn their tags.
    let mut nullary_variants = vec![];
    let mut block_variants = vec![];
    for variant in s.variants().iter() {
        if variant.bindings().is_empty() {
            nullary_variants.push((variant, nullary_variants.len() as isize));
        } else {
            block_variants.push((variant, block_variants.len() as isize));
        };
    }
    // Block tags larger than this value indicate specific OCaml types (and tags
    // larger than 255 wouldn't fit in a u8 anyway).
    // See https://github.com/ocaml/ocaml/blob/3.08/utils/config.mlp#L55
    assert!(
        block_variants.len() <= 246,
        "Too many non-constant enum variants -- maximum is 246"
    );
    EnumVariants {
        nullary_variants,
        block_variants,
    }
}

fn enum_to_ocamlrep(s: &synstructure::Structure<'_>, variants: EnumVariants<'_>) -> TokenStream {
    let EnumVariants {
        nullary_variants,
        mut block_variants,
    } = variants;
    let mut all_variants = nullary_variants;
    all_variants.append(&mut block_variants);
    s.each_variant(|v| {
        let size = v.bindings().len();
        let tag = {
            all_variants
                .iter()
                .find(|(var, _)| *var == v)
                .map(|(_, tag)| *tag)
                .unwrap()
        };
        if size == 0 {
            quote!(::ocamlrep::Value::int(#tag))
        } else {
            let tag = tag as u8;
            match get_boxed_tuple_len(v) {
                None => allocate_block(v, tag),
                Some(len) => boxed_tuple_variant_to_block(&v.bindings()[0], tag, len),
            }
        }
    })
}

fn enum_from_ocamlrep(variants: EnumVariants<'_>, from_in: bool) -> TokenStream {
    let EnumVariants {
        nullary_variants,
        block_variants,
    } = variants;
    let max_nullary_tag = nullary_variants.len().saturating_sub(1);
    let max_block_tag = block_variants.len().saturating_sub(1) as u8;

    let mut nullary_arms = TokenStream::new();
    for (variant, tag) in nullary_variants.iter() {
        let constructor = variant.construct(|_, _| quote!(unreachable!()));
        nullary_arms.extend(quote! { #tag => Ok(#constructor), });
    }
    nullary_arms.extend(quote! {
        tag => Err(::ocamlrep::FromError::NullaryVariantTagOutOfRange {
            max: #max_nullary_tag,
            actual: tag,
        })
    });

    let mut block_arms = TokenStream::new();
    for (variant, tag) in block_variants.iter() {
        let tag = *tag as u8;
        let (size, constructor) = match get_boxed_tuple_len(variant) {
            None => (
                variant.bindings().len(),
                variant.construct(|_, i| field_constructor(i, from_in)),
            ),
            Some(len) => (len, boxed_tuple_variant_constructor(variant, len, from_in)),
        };
        block_arms.extend(quote! { #tag => {
            ::ocamlrep::from::expect_block_size(block, #size)?;
            Ok(#constructor)
        } });
    }
    block_arms.extend(quote! {
        tag => Err(::ocamlrep::FromError::BlockTagOutOfRange {
            max: #max_block_tag,
            actual: tag,
        })
    });

    match (nullary_variants.is_empty(), block_variants.is_empty()) {
        // An enum with no variants is not instantiable.
        (true, true) => panic!("cannot derive OcamlRep for non-instantiable enum"),
        // Nullary variants only.
        (false, true) => quote! {
            match ::ocamlrep::from::expect_int(value)? { #nullary_arms }
        },
        // Block variants only.
        (true, false) => quote! {
            let block = ::ocamlrep::from::expect_block(value)?;
            match block.tag() { #block_arms }
        },
        // Both nullary and block variants.
        (false, false) => quote! {
            if value.is_int() {
                match value.as_int().unwrap() { #nullary_arms }
            } else {
                let block = value.as_block().unwrap();
                match block.tag() { #block_arms }
            }
        },
    }
}

fn allocate_block(variant: &VariantInfo<'_>, tag: u8) -> TokenStream {
    let size = variant.bindings().len();
    let mut fields = TokenStream::new();
    for (i, bi) in variant.bindings().iter().enumerate() {
        fields.extend(quote! {
            arena.set_field(&mut block, #i, arena.add(#bi));
        });
    }
    quote! {
        let mut block = arena.block_with_size_and_tag(#size, #tag);
        #fields
        block.build()
    }
}

fn boxed_tuple_variant_to_block(bi: &BindingInfo<'_>, tag: u8, len: usize) -> TokenStream {
    let mut fields = TokenStream::new();
    for i in 0..len {
        let idx = syn::Index::from(i);
        fields.extend(quote! {
            arena.set_field(&mut block, #i, arena.add(&#bi.#idx));
        });
    }
    quote! {
        let mut block = arena.block_with_size_and_tag(#len, #tag);
        #fields
        block.build()
    }
}

fn field_constructor(index: usize, from_in: bool) -> TokenStream {
    if from_in {
        quote! { ::ocamlrep::from::field_in(block, #index, alloc)? }
    } else {
        quote! { ::ocamlrep::from::field(block, #index)? }
    }
}

fn boxed_tuple_variant_constructor(
    variant: &VariantInfo<'_>,
    len: usize,
    from_in: bool,
) -> TokenStream {
    let mut ident = TokenStream::new();
    if let Some(prefix) = variant.prefix {
        ident.extend(quote!(#prefix ::));
    }
    let id = variant.ast().ident;
    ident.extend(quote!(#id));

    let mut fields = TokenStream::new();
    for idx in 0..len {
        fields.extend(if from_in {
            quote! { ::ocamlrep::from::field_in(block, #idx, alloc)?, }
        } else {
            quote! { ::ocamlrep::from::field(block, #idx)?, }
        })
    }
    if from_in {
        quote! { #ident(alloc.alloc((#fields))) }
    } else {
        quote! { #ident(::std::boxed::Box::new((#fields))) }
    }
}

fn get_boxed_tuple_len(variant: &VariantInfo<'_>) -> Option<usize> {
    use syn::Fields;
    use syn::GenericArgument;
    use syn::PathArguments;
    use syn::Type;
    use syn::TypePath;
    use syn::TypeReference;

    match &variant.ast().fields {
        Fields::Unnamed(_) => {}
        _ => return None,
    }
    let bi = match variant.bindings() {
        [bi] => bi,
        _ => return None,
    };
    let tuple = match &bi.ast().ty {
        Type::Path(TypePath { path, .. }) => {
            let path_seg = match path.segments.first() {
                Some(s) if s.ident == "Box" => s,
                _ => return None,
            };
            let args = match &path_seg.arguments {
                PathArguments::AngleBracketed(args) => args,
                _ => return None,
            };
            match args.args.first() {
                Some(GenericArgument::Type(Type::Tuple(tuple))) => tuple,
                _ => return None,
            }
        }
        Type::Reference(TypeReference { elem, .. }) => match &**elem {
            Type::Tuple(tuple) => tuple,
            _ => return None,
        },
        _ => return None,
    };
    Some(tuple.elems.len())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use macro_test_util::assert_pat_eq;
    use synstructure::Structure;

    use super::*;

    #[test]
    fn basic_to() -> Result<()> {
        let input = quote! {
                struct A {
                    a: i64,
                    b: i64,
                    #[ocamlrep(skip)]
                    c: f64,
                    d: String,
                }
        };
        assert_pat_eq::<anyhow::Error>(
            Ok(derive_to_ocamlrep(Structure::new(&syn::parse2(input)?))),
            quote! {
                #[allow(non_local_definitions)]
                #[allow(non_upper_case_globals)]
                const _DERIVE_ocamlrep_ToOcamlRep_FOR_A: () = {
                    impl ::ocamlrep::ToOcamlRep for A {
                        fn to_ocamlrep<'__ocamlrep_derive_allocator, Alloc: ::ocamlrep::Allocator>(
                            &'__ocamlrep_derive_allocator self,
                            arena: &'__ocamlrep_derive_allocator Alloc,
                        ) -> ::ocamlrep::Value<'__ocamlrep_derive_allocator> {
                            use ::ocamlrep::Allocator;
                            match *self {
                                A {
                                    a: ref __binding_0,
                                    b: ref __binding_1,
                                    d: ref __binding_3,
                                    ..
                                } => {
                                    let mut block = arena.block_with_size_and_tag(3usize, 0u8);
                                    arena.set_field(&mut block, 0usize, arena.add(__binding_0));
                                    arena.set_field(&mut block, 1usize, arena.add(__binding_1));
                                    arena.set_field(&mut block, 2usize, arena.add(__binding_3));
                                    block.build()
                                }
                            }
                        }
                    }
                };
            },
        );
        Ok(())
    }

    #[test]
    fn basic_from() -> Result<()> {
        let input = quote! {
              struct A {
                  a: i64,
                  b: i64,
                  #[ocamlrep(skip)]
                  c: f64,
                  d: String,
              }
        };
        assert_pat_eq::<anyhow::Error>(
            Ok(derive_from_ocamlrep(Structure::new(&syn::parse2(input)?))),
            quote! {
                #[allow(non_local_definitions)]
                #[allow(non_upper_case_globals)]
                const _DERIVE_ocamlrep_FromOcamlRep_FOR_A: () = {
                    impl ::ocamlrep::FromOcamlRep for A {
                        fn from_ocamlrep(
                            value: ::ocamlrep::Value<'_>
                        ) -> ::std::result::Result<Self, ::ocamlrep::FromError> {
                            use ::ocamlrep::FromOcamlRep;
                            let block = ::ocamlrep::from::expect_tuple(value, 3usize)?;
                            Ok(A {
                                a: ::ocamlrep::from::field(block, 0usize)?,
                                b: ::ocamlrep::from::field(block, 1usize)?,
                                c: ::std::default::Default::default(),
                                d: ::ocamlrep::from::field(block, 2usize)?,
                            })
                        }
                    }
                };
            },
        );
        Ok(())
    }
}
