use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields};

use crate::{
    attrs::{Attrs, BoundsChecksMode},
    derive_context::{DeriveContext, DeriveContextBase},
    shared_derives::{
        derive_add, derive_add_assign, derive_add_assign_compat, derive_add_compat, derive_clone,
        derive_copy, derive_default, derive_div, derive_div_assign, derive_div_assign_compat,
        derive_div_compat, derive_eq, derive_from_self_for_usize, derive_from_usize, derive_mul,
        derive_mul_assign, derive_mul_assign_compat, derive_mul_compat, derive_ord,
        derive_partial_ord, derive_rem, derive_rem_assign, derive_rem_assign_compat,
        derive_rem_compat, derive_sub, derive_sub_assign, derive_sub_assign_compat,
        derive_sub_compat,
    },
    utils::token_stream_to_compact_string,
};

struct EnumCtxCustom<'a> {
    idents: Vec<&'a Ident>,
    ident_strings: Vec<String>,
}

type EnumCtx<'a> = DeriveContext<EnumCtxCustom<'a>>;

fn enum_derive_idx(ctx: &EnumCtx) -> TokenStream {
    let self_as_idx = &ctx.base.self_as_idx;
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;

    let (impl_generics, ty_generics, where_clause) = ctx.base.generics.split_for_impl();

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
                #self_as_idx::into_usize_unchecked(self)
            }

            fn saturating_add(self, other: Self) -> Self {
                #self_as_idx::from_usize(
                    #self_as_idx::into_usize(self)
                        .saturating_add(#self_as_idx::into_usize(other))
                        .min(#self_as_idx::into_usize(#self_as_idx::MAX))
                )
            }

            fn saturating_sub(self, other: Self) -> Self {
                #self_as_idx::from_usize(
                    #self_as_idx::into_usize(self).saturating_sub(other.into_usize())
                )
            }

            fn wrapping_add(self, other: Self) -> Self {
                const COUNT: usize = #count;
                let offset_on_wrap =
                    (::core::primitive::usize::MAX % COUNT).saturating_add(1);
                let (sum, of) = #self_as_idx::into_usize(self)
                    .overflowing_add( #self_as_idx::into_usize(other));
                if of {
                    return #self_as_idx::from_usize(sum + offset_on_wrap);
                }
                if sum < COUNT {
                    return #self_as_idx::from_usize(sum);
                }
                return #self_as_idx::from_usize(sum % COUNT);
            }

            fn wrapping_sub(self, other: Self) -> Self {
                const COUNT: usize = #count;
                let offset_on_wrap =
                    (::core::primitive::usize::MAX % COUNT).saturating_add(1);
                let (diff, of) = #self_as_idx::into_usize(self)
                    .overflowing_sub(#self_as_idx::into_usize(other));
                if of {
                    return #self_as_idx::from_usize(diff - offset_on_wrap);
                }
                if diff < COUNT {
                    return #self_as_idx::from_usize(diff);
                }
                #self_as_idx::from_usize(diff % COUNT)
            }
        }
    }
}

fn enum_derive_idx_enum(ctx: &EnumCtx) -> TokenStream {
    let indexland = &ctx.base.attrs.indexland_path;
    let name = &ctx.base.name;
    let (impl_generics, ty_generics, where_clause) = ctx.base.generics.split_for_impl();
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

fn enum_derive_hash(ctx: &EnumCtx) -> TokenStream {
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

fn enum_derive_debug(ctx: &EnumCtx) -> TokenStream {
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

fn enum_derive_display(ctx: &EnumCtx) -> TokenStream {
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

fn enum_derive_partial_eq(ctx: &EnumCtx) -> TokenStream {
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

fn fill_derivation_list(ctx: &mut EnumCtx) {
    let (base_arith, full_arith) = match ctx.base.attrs.arith_mode {
        crate::attrs::ArithMode::Disabled => (false, false),
        crate::attrs::ArithMode::Basic => (true, false),
        crate::attrs::ArithMode::Full => (true, true),
    };
    ctx.add_deriv_custom(true, "Idx", enum_derive_idx);
    ctx.add_deriv_custom(true, "IdxEnum", enum_derive_idx_enum);
    ctx.add_deriv_custom(true, "Debug", enum_derive_debug);
    ctx.add_deriv_custom(false, "Display", enum_derive_display);
    ctx.add_deriv_shared(true, "Default", derive_default);
    ctx.add_deriv_shared(true, "Clone", derive_clone);
    ctx.add_deriv_shared(true, "Copy", derive_copy);
    ctx.add_deriv_shared(true, "Add", derive_add);
    ctx.add_deriv_shared(true, "AddAssign", derive_add_assign);
    ctx.add_deriv_shared(true, "Sub", derive_sub);
    ctx.add_deriv_shared(true, "SubAssign", derive_sub_assign);
    ctx.add_deriv_shared(full_arith, "Mul", derive_mul);
    ctx.add_deriv_shared(full_arith, "MulAssign", derive_mul_assign);
    ctx.add_deriv_shared(full_arith, "Div", derive_div);
    ctx.add_deriv_shared(full_arith, "DivAssign", derive_div_assign);
    ctx.add_deriv_shared(full_arith, "Rem", derive_rem);
    ctx.add_deriv_shared(full_arith, "RemAssign", derive_rem_assign);
    ctx.add_deriv_custom(true, "Hash", enum_derive_hash);
    ctx.add_deriv_shared(true, "PartialOrd", derive_partial_ord);
    ctx.add_deriv_shared(true, "Ord", derive_ord);
    ctx.add_deriv_custom(true, "PartialEq", enum_derive_partial_eq);
    ctx.add_deriv_shared(true, "Eq", derive_eq);
    ctx.add_deriv_shared(true, "From<usize>", derive_from_usize);
    ctx.add_deriv_shared(true, "From<Self> for usize", derive_from_self_for_usize);

    for i in 0..ctx.base.attrs.arith_compat_list.len() {
        let ty_tt = ctx.base.attrs.arith_compat_list[i].to_token_stream();
        let ty_str = token_stream_to_compact_string(&ty_tt);

        let add_compat =
            move |ctx: &mut DeriveContext<EnumCtxCustom<'_>>,
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

pub fn derive_idx_enum_inner(ast: DeriveInput) -> Result<TokenStream, syn::Error> {
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
