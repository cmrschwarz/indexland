use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DataEnum, DeriveInput, Fields};

use crate::context::Context;

pub fn derive_idx_for_enum(
    ctx: &mut Context,
    ast: &DeriveInput,
    enum_data: &DataEnum,
) -> Result<TokenStream, syn::Error> {
    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let mut idents = Vec::new();

    for variant in &enum_data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            ctx.errors.push_error(syn::Error::new(
                Span::call_site(),
                "This macro does not support enum variants with payload.",
            ));
        };

        idents.push(&variant.ident);
    }

    let count = idents.len();
    if count < 2 {
        return Err(syn::Error::new(
            Span::call_site(),
            "enum deriving IdxEnum must have at least two variants",
        ));
    }

    let var_zero = &idents[0];
    let var_one = &idents[1];
    let var_max = &idents[count - 1];

    let indices = 0..count;
    let indices_2 = 0..count;

    let indexland = &ctx.attrs.indexland_path;

    let output = quote! {
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
    };
    Ok(output)
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

    let mut ctx = Context::from_input(&ast);

    let name = &ast.ident;
    let gen = &ast.generics;
    let (impl_generics, ty_generics, where_clause) = gen.split_for_impl();

    let mut idents = Vec::new();
    let mut ident_strings = Vec::new();

    for variant in &enum_data.variants {
        idents.push(&variant.ident);
        ident_strings.push(variant.ident.to_string());
    }

    let count = idents.len();

    let idx_derivation = derive_idx_for_enum(&mut ctx, &ast, enum_data)?;

    let indexland = &ctx.attrs.indexland_path;

    let output = quote! {
        impl ::core::default::Default for #name {
            fn default() -> Self {
                #indexland::Idx::ZERO
            }
        }
        #[allow(clippy::expl_impl_clone_on_copy)]
        impl ::core::clone::Clone for #name {
            fn clone(&self) -> Self {
                #indexland::Idx::from_usize(#indexland::Idx::into_usize(*self))
            }
        }
        impl ::core::marker::Copy for #name {}
        impl core::hash::Hash for #name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                core::mem::discriminant(self).hash(state);
            }
        }
        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #(#name::#idents => f.write_str(#ident_strings)),*
                }
            }
        }
        impl ::core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) + #indexland::Idx::into_usize(rhs),
                )
            }
        }
        impl ::core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) - #indexland::Idx::into_usize(rhs),
                )
            }
        }
        impl ::core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
        impl ::core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
        impl ::core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                #indexland::Idx::into_usize(*self)
                    .partial_cmp(&#indexland::Idx::into_usize(*other))
            }
        }
        impl ::core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                #indexland::Idx::into_usize(*self)
                    .cmp(&#indexland::Idx::into_usize(*other))
            }
        }
        impl ::core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                core::mem::discriminant(self) == core::mem::discriminant(other)
            }
        }
        impl ::core::cmp::Eq for #name {}
        impl #impl_generics #indexland::IdxEnum for #name #ty_generics #where_clause {
            const COUNT: usize = #count;
            type EnumIndexArray<T> = #indexland::index_array::IndexArray<Self, T, #count>;
            const VARIANTS: &'static [Self] = &[ #(#name::#idents),* ];
        }
        #idx_derivation
    };
    Ok(output)
    // Err(syn::Error::new(Span::call_site(), output.to_string()))
}
