use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Type};

use crate::{
    attrs::{Attrs, BoundsChecksMode},
    derive_context::{DeriveContext, DeriveContextBase},
    shared_derives::{
        derive_add_assign, derive_add_assign_compat, derive_add_compat, derive_clone, derive_copy,
        derive_default, derive_div_assign, derive_div_assign_compat, derive_div_compat, derive_eq,
        derive_mul_assign, derive_mul_assign_compat, derive_mul_compat, derive_rem_assign,
        derive_rem_assign_compat, derive_rem_compat, derive_sub_assign, derive_sub_assign_compat,
        derive_sub_compat,
    },
    utils::token_stream_to_compact_string,
};

struct NewtypeCtxCustom<'a> {
    base_type: &'a Type,
    base_as_idx: TokenStream,
}

type NewtypeCtx<'a> = DeriveContext<NewtypeCtxCustom<'a>>;

fn newtype_derive_idx(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;

    let (impl_generics, ty_generics, where_clause) = ctx.base.generics.split_for_impl();

    let base_as_idx = &ctx.custom.base_as_idx;

    let checked_conversions = match ctx.base.attrs.bounds_checks_mode {
        BoundsChecksMode::Never => {
            quote! {
                #[inline(always)]
                fn from_usize(v: usize) -> Self {
                    #name(#base_as_idx::from_usize_unchecked(v))
                }
                #[inline(always)]
                fn into_usize(self) -> usize {
                    #base_as_idx::into_usize_unchecked(self.0)
                }
            }
        }
        BoundsChecksMode::DebugOnly => {
            quote! {
                #[inline(always)]
                fn from_usize(v: usize) -> Self {
                    #[cfg(debug_assertions)]
                    return #name(#base_as_idx::from_usize(v));

                    #[cfg(not(debug_assertions))]
                    #name(#base_as_idx::from_usize_unchecked(v))
                }
                #[inline(always)]
                fn into_usize(self) -> usize {
                    #[cfg(debug_assertions)]
                    return #base_as_idx::into_usize(self.0);

                    #[cfg(not(debug_assertions))]
                    #base_as_idx::into_usize_unchecked(self.0)
                }
            }
        }
        BoundsChecksMode::Always => {
            quote! {
                #[inline(always)]
                fn from_usize(v: usize) -> Self {
                    #name(#base_as_idx::from_usize(v))
                }
                #[inline(always)]
                fn into_usize(self) -> usize {
                    #base_as_idx::into_usize(self.0)
                }
            }
        }
    };

    quote! {
        #[automatically_derived]
        impl #impl_generics #indexland::Idx for #name #ty_generics #where_clause {
            const ZERO: Self = #name(#base_as_idx::ZERO);
            const ONE: Self = #name(#base_as_idx::ONE);
            const MAX: Self = #name(#base_as_idx::MAX);
            #checked_conversions
            #[inline(always)]
            fn from_usize_unchecked(v: usize) -> Self {
                #name(#base_as_idx::from_usize_unchecked(v))
            }
            #[inline(always)]
            fn into_usize_unchecked(self) -> usize {
                #base_as_idx::into_usize_unchecked(self.0)
            }
            fn wrapping_add(self, other: Self) -> Self {
                #name(#base_as_idx::wrapping_add(self.0, other.0))
            }
            fn wrapping_sub(self, other: Self) -> Self {
                #name(#base_as_idx::wrapping_sub(self.0, other.0))
            }
            fn saturating_add(self, other: Self) -> Self {
                #name(#base_as_idx::saturating_add(self.0, other.0))
            }
            fn saturating_sub(self, other: Self) -> Self {
                #name(#base_as_idx::saturating_sub(self.0, other.0))
            }
        }
    }
}

fn newtype_derive_idx_newtype(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    let (impl_generics, ty_generics, where_clause) = ctx.base.generics.split_for_impl();
    let base_type = &ctx.custom.base_type;
    quote! {
        #[automatically_derived]
        impl #impl_generics #indexland::IdxNewtype for #name #ty_generics #where_clause {
            type Base = #base_type;
            #[inline]
            fn new(v: #base_type) -> Self {
                #name(v)
            }
            #[inline]
            fn into_inner(self) -> #base_type {
                self.0
            }
        }
    }
}

fn newtype_derive_hash(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::hash::Hash for #name {
            #[inline]
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

    }
}

fn newtype_derive_from_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    let base_type = &ctx.custom.base_type;
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<usize> for #name {
            #[inline]
            fn from(v: usize) -> #name {
                #name(<#base_type as #indexland::Idx>::from_usize(v))
            }
        }
    }
}

fn newtype_derive_from_self_for_usize(ctx: &NewtypeCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    let base_type = &ctx.custom.base_type;
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<#name> for usize {
            #[inline]
            fn from(v: #name) -> usize {
                <#base_type as #indexland::Idx>::into_usize(v.0)
            }
        }
    }
}

fn newtype_derive_debug(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::fmt::Debug for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Debug::fmt(&self.0, f)
            }
        }
    }
}

fn newtype_derive_display(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::fmt::Display for #name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::fmt::Display::fmt(&self.0, f)
            }
        }
    }
}

fn newtype_derive_add(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #name(self.0 + rhs.0)
            }
        }
    }
}

fn newtype_derive_sub(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                #name(self.0 - rhs.0)
            }
        }
    }
}

fn newtype_derive_mul(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Mul for #name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                #name(self.0 * rhs.0)
            }
        }
    }
}

fn newtype_derive_div(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Div for #name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                #name(self.0 / rhs.0)
            }
        }
    }
}

fn newtype_derive_rem(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Rem for #name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self::Output {
                #name(self.0 % rhs.0)
            }
        }
    }
}

fn newtype_derive_partial_ord(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                core::cmp::PartialOrd::partial_cmp(&self.0, &other.0)
            }
        }
    }
}

fn newtype_derive_ord(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                core::cmp::Ord::cmp(&self.0, &other.0)
            }
        }
    }
}

fn newtype_derive_partial_eq(ctx: &NewtypeCtx) -> TokenStream {
    let name = &ctx.base.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
    }
}

fn fill_derivation_list(ctx: &mut NewtypeCtx, rich_defaults: bool) {
    let (base_arith, full_arith) = match ctx.base.attrs.arith_mode {
        crate::attrs::ArithMode::Disabled => (false, false),
        crate::attrs::ArithMode::Basic => (true, false),
        crate::attrs::ArithMode::Full => (true, true),
    };
    ctx.add_deriv_custom(true, "Idx", newtype_derive_idx);
    ctx.add_deriv_custom(true, "IdxEnum", newtype_derive_idx_newtype);
    ctx.add_deriv_custom(rich_defaults, "Debug", newtype_derive_debug);
    ctx.add_deriv_custom(rich_defaults, "Display", newtype_derive_display);
    ctx.add_deriv_shared(rich_defaults, "Default", derive_default);
    ctx.add_deriv_shared(rich_defaults, "Clone", derive_clone);
    ctx.add_deriv_shared(rich_defaults, "Copy", derive_copy);
    ctx.add_deriv_custom(rich_defaults, "Add", newtype_derive_add);
    ctx.add_deriv_custom(rich_defaults, "Sub", newtype_derive_sub);
    ctx.add_deriv_custom(full_arith, "Mul", newtype_derive_mul);
    ctx.add_deriv_custom(full_arith, "Div", newtype_derive_div);
    ctx.add_deriv_custom(full_arith, "Rem", newtype_derive_rem);
    ctx.add_deriv_shared(rich_defaults, "AddAssign", derive_add_assign);
    ctx.add_deriv_shared(rich_defaults, "SubAssign", derive_sub_assign);
    ctx.add_deriv_shared(full_arith, "MulAssign", derive_mul_assign);
    ctx.add_deriv_shared(full_arith, "DivAssign", derive_div_assign);
    ctx.add_deriv_shared(full_arith, "RemAssign", derive_rem_assign);
    ctx.add_deriv_custom(rich_defaults, "Hash", newtype_derive_hash);
    ctx.add_deriv_custom(rich_defaults, "PartialOrd", newtype_derive_partial_ord);
    ctx.add_deriv_custom(rich_defaults, "Ord", newtype_derive_ord);
    ctx.add_deriv_custom(rich_defaults, "PartialEq", newtype_derive_partial_eq);
    ctx.add_deriv_shared(rich_defaults, "Eq", derive_eq);
    ctx.add_deriv_custom(rich_defaults, "From<usize>", newtype_derive_from_usize);
    ctx.add_deriv_custom(
        rich_defaults,
        "From<Self> for usize",
        newtype_derive_from_self_for_usize,
    );

    for i in 0..ctx.base.attrs.arith_compat_list.len() {
        let ty_tt = ctx.base.attrs.arith_compat_list[i].to_token_stream();
        let ty_str = token_stream_to_compact_string(&ty_tt);

        let add_compat =
            move |ctx: &mut DeriveContext<NewtypeCtxCustom<'_>>,
                  default: bool,
                  name: &str,
                  f: fn(&DeriveContextBase, TokenStream) -> TokenStream| {
                let ty = ty_tt.clone();
                ctx.add_deriv_shared(default, format!("{name}<{ty_str}>"), move |ctx| f(ctx, ty));
            };

        add_compat(ctx, base_arith, "Add", derive_add_compat);
        add_compat(ctx, base_arith, "Sub", derive_sub_compat);
        add_compat(ctx, base_arith, "AddAssign", derive_add_assign_compat);
        add_compat(ctx, base_arith, "SubAssign", derive_sub_assign_compat);

        add_compat(ctx, full_arith, "Mul", derive_mul_compat);
        add_compat(ctx, full_arith, "Div", derive_div_compat);
        add_compat(ctx, full_arith, "Rem", derive_rem_compat);

        add_compat(ctx, full_arith, "MulAssign", derive_mul_assign_compat);
        add_compat(ctx, full_arith, "DivAssign", derive_div_assign_compat);
        add_compat(ctx, full_arith, "RemAssign", derive_rem_assign_compat);
    }
}

pub fn derive_idx_newtype_inner(
    ast: DeriveInput,
    rich_defaults: bool,
) -> Result<TokenStream, syn::Error> {
    let Data::Struct(struct_data) = &ast.data else {
        return Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports newtype structs",
        ));
    };
    let inner = match &struct_data.fields {
        Fields::Unnamed(fields_unnamed) => {
            if fields_unnamed.unnamed.len() != 1 {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "This macro only supports newtype structs with exactly one member",
                ));
            }
            &fields_unnamed.unnamed[0]
        }
        Fields::Named(_) | Fields::Unit => {
            return Err(syn::Error::new(
                Span::call_site(),
                "This macro only supports newtype structs",
            ));
        }
    };
    let attrs = Attrs::from_input(&ast);
    let base_type = &inner.ty;
    let name = ast.ident;
    let indexland = &attrs.indexland_path;

    let custom = NewtypeCtxCustom {
        base_type,
        base_as_idx: quote! { <#base_type as #indexland::Idx> },
    };

    let mut ctx = NewtypeCtx::new(attrs, name, ast.generics, custom);

    // we don't derive if the type definition is already borked
    ctx.base.attrs.error_list.check()?;

    fill_derivation_list(&mut ctx, rich_defaults);

    let res = ctx.generate();

    ctx.base.attrs.error_list.check()?;

    Ok(res)
}
