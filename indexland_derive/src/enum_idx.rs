use std::collections::HashMap;

use proc_macro2::Ident;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Fields, Generics};

use crate::context::{Attrs, Context, ErrorList};

struct EnumCtx<'a> {
    error_list: ErrorList,
    attrs: Attrs,
    name: Ident,
    generics: &'a Generics,
    idents: Vec<&'a Ident>,
    ident_strings: Vec<String>,
}

type EnumTraitDerivation = fn(&EnumCtx) -> TokenStream;

fn enum_derive_idx(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;

    let (impl_generics, ty_generics, where_clause) =
        ctx.generics.split_for_impl();

    let idents = &ctx.idents;
    let count = idents.len();
    let var_zero = &idents[0];
    let var_one = &idents[1];
    let var_max = &idents[count - 1];

    let indices = 0..count;
    let indices_2 = 0..count;

    quote! {
        #[automatically_derived]
        impl #impl_generics #indexland::Idx for #name #ty_generics #where_clause {
            const ZERO: Self = #name::#var_zero;
            const ONE: Self = #name::#var_one;
            const MAX: Self = #name::#var_max;
            fn from_usize(v: usize) -> Self {
                match v {
                    #(#indices => #name::#idents,)*
                    _ => panic!("enum index out of bounds"),
                }
            }
            fn into_usize(self) -> usize  {
                match self {
                    #(#name::#idents => #indices_2),*
                }
            }
        }
    }
}

fn enum_derive_idx_enum(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    let (impl_generics, ty_generics, where_clause) =
        ctx.generics.split_for_impl();
    let idents = &ctx.idents;
    let count = idents.len();
    quote! {
        #[automatically_derived]
        impl #impl_generics #indexland::IdxEnum for #name #ty_generics #where_clause {
            const COUNT: usize = #count;
            type EnumIndexArray<T> = #indexland::index_array::IndexArray<Self, T, #count>;
            const VARIANTS: &'static [Self] = &[ #(#name::#idents),* ];
        }
    }
}

fn enum_derive_default(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::default::Default for #name {
            fn default() -> Self {
                #indexland::Idx::ZERO
            }
        }
    }
}

fn enum_derive_clone(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::clone::Clone for #name {
            fn clone(&self) -> Self {
               *self
            }
        }
    }
}

fn enum_derive_copy(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::marker::Copy for #name {}
    }
}

fn enum_derive_hash(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl core::hash::Hash for #name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                core::mem::discriminant(self).hash(state);
            }
        }
    }
}

fn enum_derive_debug(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.name;
    let idents = &ctx.idents;
    let ident_strings = &ctx.ident_strings;
    quote! {
        #[automatically_derived]
        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #(#name::#idents => f.write_str(#ident_strings)),*
                }
            }
        }
    }
}

fn enum_derive_add(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) + #indexland::Idx::into_usize(rhs),
                )
            }
        }
    }
}

fn enum_derive_sub(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) - #indexland::Idx::into_usize(rhs),
                )
            }
        }
    }
}

fn enum_derive_add_assign(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
    }
}

fn enum_derive_sub_assign(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
    }
}

fn enum_derive_partial_ord(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                #indexland::Idx::into_usize(*self)
                    .partial_cmp(&#indexland::Idx::into_usize(*other))
            }
        }
    }
}

fn enum_derive_ord(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                #indexland::Idx::into_usize(*self)
                    .cmp(&#indexland::Idx::into_usize(*other))
            }
        }
    }
}

fn enum_derive_partial_eq(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                core::mem::discriminant(self) == core::mem::discriminant(other)
            }
        }
    }
}

fn enum_derive_eq(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Eq for #name {}
    }
}

fn enum_derive_from_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<usize> for #name {
            #[inline]
            fn from(v: usize) -> #name {
                #indexland::Idx::from_usize(v)
            }
        }
    }
}

fn enum_derive_from_enum(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<#name> for usize {
            #[inline]
            fn from(v: #name) -> usize {
                #indexland::Idx::into_usize(v)
            }
        }
    }
}

fn derivation_list(ctx: &EnumCtx) -> HashMap<String, EnumTraitDerivation> {
    let mut derivations = HashMap::<String, EnumTraitDerivation>::new();
    derivations.insert("Idx".to_string(), enum_derive_idx);
    derivations.insert("IdxEnum".to_string(), enum_derive_idx_enum);
    derivations.insert("Debug".to_string(), enum_derive_debug);
    derivations.insert("Default".to_string(), enum_derive_default);
    derivations.insert("Clone".to_string(), enum_derive_clone);
    derivations.insert("Copy".to_string(), enum_derive_copy);
    derivations.insert("Add".to_string(), enum_derive_add);
    derivations.insert("AddAssign".to_string(), enum_derive_add_assign);
    derivations.insert("Sub".to_string(), enum_derive_sub);
    derivations.insert("SubAssign".to_string(), enum_derive_sub_assign);
    derivations.insert("Hash".to_string(), enum_derive_hash);
    derivations.insert("PartialOrd".to_string(), enum_derive_partial_ord);
    derivations.insert("Ord".to_string(), enum_derive_ord);
    derivations.insert("PartialEq".to_string(), enum_derive_partial_eq);
    derivations.insert("Eq".to_string(), enum_derive_eq);
    derivations.insert("From<usize>".to_string(), enum_derive_from_usize);
    derivations.insert(
        format!("From<{}> for usize", ctx.name).to_string(),
        enum_derive_from_enum,
    );
    derivations
}

pub fn derive_idx_enum_inner(
    ast: DeriveInput,
) -> Result<TokenStream, syn::Error> {
    let Data::Enum(enum_data) = &ast.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports enums.",
        ));
    };

    let ctx = Context::from_input(&ast);

    let name = ast.ident;
    let generics = &ast.generics;

    let mut idents = Vec::new();
    let mut ident_strings = Vec::new();

    for variant in &enum_data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            ctx.error_list.push(syn::Error::new(
                Span::call_site(),
                "This macro does not support enum variants with payload.",
            ));
        };
        idents.push(&variant.ident);
        ident_strings.push(variant.ident.to_string());
    }

    let count = idents.len();
    if count < 2 {
        return Err(syn::Error::new(
            Span::call_site(),
            "enum deriving IdxEnum must have at least two variants",
        ));
    }

    ctx.error_list.check()?;

    let enum_ctx = EnumCtx {
        error_list: ctx.error_list,
        attrs: ctx.attrs,
        name,
        generics,
        idents,
        ident_strings,
    };

    let mut derivation_list = derivation_list(&enum_ctx);

    for entry in &enum_ctx.attrs.blacklist {
        let descr = entry.to_token_stream().to_string();
        if derivation_list.remove(&*descr).is_none() {
            enum_ctx.error_list.error(
                entry.span(),
                format!(
                    "`{descr}` does not name a trait that will be derived"
                ),
            );
        }
    }

    let mut derivations = Vec::new();

    if enum_ctx.attrs.whitelist_active {
        for entry in &enum_ctx.attrs.whitelist {
            let descr = entry.to_token_stream().to_string();
            match derivation_list.get(&*descr) {
                Some(deriv) => {
                    derivations.push(deriv(&enum_ctx));
                }
                None => enum_ctx.error_list.error(
                    entry.span(),
                    format!(
                        "`{descr}` does not name a trait that will be derived"
                    ),
                ),
            }
        }
    } else {
        for deriv in derivation_list.values() {
            derivations.push(deriv(&enum_ctx));
        }
    }

    enum_ctx.error_list.check()?;

    let output = quote! {
        #(#derivations)*
    };

    Ok(output)
}
