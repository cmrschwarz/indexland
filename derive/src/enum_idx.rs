use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

use crate::{
    attrs::{Attrs, BoundsChecksMode},
    derive_context::DeriveContext,
};

struct EnumCtxCustom<'a> {
    idents: Vec<&'a Ident>,
    ident_strings: Vec<String>,
}

type EnumCtx<'a> = DeriveContext<EnumCtxCustom<'a>>;

fn derive_idx(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;

    let (impl_generics, ty_generics, where_clause) =
        ctx.base.generics.split_for_impl();

    let idents = &ctx.custom.idents;
    let count = idents.len();
    let var_zero = &idents[0];
    let var_one = &idents[1];
    let var_max = &idents[count - 1];

    let indices_1 = 0..count;
    let indices_2 = 0..count;

    let panic_str = format!("index {{}} is out of bounds for {name}");
    let from_usize = match ctx.base.attrs.bounds_checks_mode {
        BoundsChecksMode::Never => quote! {
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                Self::from_usize_unchecked(v)
            }
        },
        BoundsChecksMode::DebugOnly => quote! {
            #[cfg(debug_assertions)]
            #[inline]
            fn from_usize(v: usize) -> Self {
                match v {
                    #(#indices_1 => #name::#idents,)*
                    _ => panic!(#panic_str , v)
                }
            }

            #[cfg(not(debug_assertions))]
            #[inline(always)]
            fn from_usize(v: usize) -> Self {
                Self::from_usize_unchecked(v)
            }
        },
        BoundsChecksMode::Always => quote! {
            #[inline]
            fn from_usize(v: usize) -> Self {
                match v {
                    #(#indices_1 => #name::#idents,)*
                    _ => panic!(#panic_str , v)
                }
            }
        },
    };

    quote! {
        #[automatically_derived]
        impl #impl_generics #indexland::Idx for #name #ty_generics #where_clause {
            const ZERO: Self = #name::#var_zero;
            const ONE: Self = #name::#var_one;
            const MAX: Self = #name::#var_max;
            #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                match v {
                    #(#indices_2 => #name::#idents,)*
                    _ => #name::#var_zero
                }
            }
            #from_usize
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize  {
                self as usize
            }
            #[inline(always)]
            fn into_usize(self) -> usize  {
                Self::into_usize_unchecked(self)
            }
            fn wrapping_add(self, other: Self) -> Self {
                Self::from_usize(
                    self.into_usize()
                        .wrapping_add(other.into_usize())
                        .min(<Self as #indexland::Idx>::MAX.into_usize()),
                )
            }
            fn wrapping_sub(self, other: Self) -> Self {
                Self::from_usize(
                    self.into_usize()
                        .wrapping_sub(other.into_usize())
                        .min(<Self as #indexland::Idx>::MAX.into_usize()),
                )
            }
            fn saturating_add(self, other: Self) -> Self {
                Self::from_usize(
                    self.into_usize()
                        .saturating_add(other.into_usize())
                        .min(<Self as #indexland::Idx>::MAX.into_usize()),
                )
            }
            fn saturating_sub(self, other: Self) -> Self {
                Self::from_usize(
                    self.into_usize()
                        .saturating_sub(other.into_usize())
                        .min(<Self as #indexland::Idx>::MAX.into_usize()),
                )
            }
        }
    }
}

fn derive_idx_enum(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    let (impl_generics, ty_generics, where_clause) =
        ctx.base.generics.split_for_impl();
    let idents = &ctx.custom.idents;
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

fn derive_default(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::default::Default for #name {
            fn default() -> Self {
                #indexland::Idx::ZERO
            }
        }
    }
}

fn derive_clone(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::clone::Clone for #name {
            fn clone(&self) -> Self {
               *self
            }
        }
    }
}

fn derive_copy(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::marker::Copy for #name {}
    }
}

fn derive_hash(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl core::hash::Hash for #name {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                core::mem::discriminant(self).hash(state);
            }
        }
    }
}

fn derive_debug(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    let idents = &ctx.custom.idents;
    let ident_strings = &ctx.custom.ident_strings;
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

fn derive_display(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    let idents = &ctx.custom.idents;
    let ident_strings = &ctx.custom.ident_strings;
    quote! {
        #[automatically_derived]
        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self {
                    #(#name::#idents => f.write_str(#ident_strings)),*
                }
            }
        }
    }
}

fn derive_add(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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

fn derive_sub(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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

fn derive_rem(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Rem for #name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) % #indexland::Idx::into_usize(rhs),
                )
            }
        }
    }
}

fn derive_add_assign(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::AddAssign for #name {
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }
    }
}

fn derive_sub_assign(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::SubAssign for #name {
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }
    }
}

fn derive_rem_assign(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::RemAssign for #name {
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }
    }
}

fn derive_partial_ord(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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

fn derive_ord(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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

fn derive_partial_eq(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                core::mem::discriminant(self) == core::mem::discriminant(other)
            }
        }
    }
}

fn derive_eq(ctx: &EnumCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Eq for #name {}
    }
}

fn derive_from_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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

fn derive_from_self_for_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
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

fn derive_add_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Add<usize> for #name {
            type Output = Self;
            fn add(self, rhs: usize) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) + rhs,
                )
            }
        }
    }
}

fn derive_sub_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Sub<usize> for #name {
            type Output = Self;
            fn sub(self, rhs: usize) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) - rhs,
                )
            }
        }
    }
}

fn derive_rem_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Rem<usize> for #name {
            type Output = Self;
            fn rem(self, rhs: usize) -> Self::Output {
                #indexland::Idx::from_usize(
                    #indexland::Idx::into_usize(self) % rhs,
                )
            }
        }
    }
}

fn derive_add_assign_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::AddAssign<usize> for #name {
            fn add_assign(&mut self, rhs: usize) {
                *self = *self + <#name as #indexland::Idx>::from_usize(rhs);
            }
        }
    }
}

fn derive_sub_assign_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::SubAssign<usize> for #name {
            fn sub_assign(&mut self, rhs: usize) {
                *self = *self - <#name as #indexland::Idx>::from_usize(rhs);
            }
        }
    }
}

fn derive_rem_assign_usize(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::RemAssign<usize> for #name {
            fn rem_assign(&mut self, rhs: usize) {
                *self = *self % <#name as #indexland::Idx>::from_usize(rhs);
            }
        }
    }
}

fn fill_derivation_list(ctx: &mut EnumCtx) {
    let usize_arith = ctx.base.attrs.enable_usize_arith;
    ctx.add_deriv_custom(true, "Idx", derive_idx);
    ctx.add_deriv_custom(true, "IdxEnum", derive_idx_enum);
    ctx.add_deriv_custom(true, "Debug", derive_debug);
    ctx.add_deriv_custom(false, "Display", derive_display);
    ctx.add_deriv_custom(true, "Default", derive_default);
    ctx.add_deriv_custom(true, "Clone", derive_clone);
    ctx.add_deriv_custom(true, "Copy", derive_copy);
    ctx.add_deriv_custom(true, "Add", derive_add);
    ctx.add_deriv_custom(true, "AddAssign", derive_add_assign);
    ctx.add_deriv_custom(true, "Sub", derive_sub);
    ctx.add_deriv_custom(true, "SubAssign", derive_sub_assign);
    ctx.add_deriv_custom(true, "Rem", derive_rem);
    ctx.add_deriv_custom(true, "RemAssign", derive_rem_assign);
    ctx.add_deriv_custom(true, "Hash", derive_hash);
    ctx.add_deriv_custom(true, "PartialOrd", derive_partial_ord);
    ctx.add_deriv_custom(true, "Ord", derive_ord);
    ctx.add_deriv_custom(true, "PartialEq", derive_partial_eq);
    ctx.add_deriv_custom(true, "Eq", derive_eq);
    ctx.add_deriv_custom(true, "From<usize>", derive_from_usize);
    ctx.add_deriv_custom(
        true,
        "From<Self> for usize",
        derive_from_self_for_usize,
    );
    ctx.add_deriv_custom(usize_arith, "Add<usize>", derive_add_usize);
    ctx.add_deriv_custom(usize_arith, "Sub<usize>", derive_sub_usize);
    ctx.add_deriv_custom(usize_arith, "Rem<usize>", derive_rem_usize);
    ctx.add_deriv_custom(
        usize_arith,
        "AddAssign<usize>",
        derive_add_assign_usize,
    );
    ctx.add_deriv_custom(
        usize_arith,
        "SubAssign<usize>",
        derive_sub_assign_usize,
    );
    ctx.add_deriv_custom(
        usize_arith,
        "RemAssign<usize>",
        derive_rem_assign_usize,
    );
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

    let attrs = Attrs::from_input(&ast);

    let name = ast.ident;
    let generics = ast.generics;

    let mut idents = Vec::new();
    let mut ident_strings = Vec::new();

    for variant in &enum_data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            attrs.error_list.push(syn::Error::new(
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

    // we don't start generation if the type is already borked
    attrs.error_list.check()?;

    let mut ctx = EnumCtx::new(
        attrs,
        name,
        generics,
        EnumCtxCustom {
            idents,
            ident_strings,
        },
    );

    fill_derivation_list(&mut ctx);

    let output = ctx.generate();

    ctx.base.attrs.error_list.check()?;

    Ok(output)
}
