use proc_macro2::TokenStream;
use quote::quote;

use crate::derive_context::DeriveContextBase;

pub fn derive_idx_compat(
    indexland: &syn::Path,
    name: &syn::Ident,
    compat: &syn::Path,
) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl #indexland::idx::IdxCompatible<#name> for #compat {
            fn idx_cast(self) -> #name {
                #name::from_usize(Idx::into_usize(self))
            }
        }
        #[automatically_derived]
        unsafe impl<T>
            #indexland::index_slice_index::IndexSliceIndex<#name, T>
            for #compat
        {
            type Output = T;

            fn get(self, slice: & #indexland::IndexSlice<#name, T>) -> Option<&Self::Output> {
                slice.as_slice().get(#indexland::Idx::into_usize(self))
            }

            fn get_mut(
                self,
                slice: &mut #indexland::IndexSlice<#name, T>,
            ) -> Option<&mut Self::Output> {
                slice
                    .as_mut_slice()
                    .get_mut(#indexland::Idx::into_usize(self))
            }

            unsafe fn get_unchecked(
                self,
                slice: *const #indexland::IndexSlice<#name, T>,
            ) -> *const Self::Output {
                unsafe { slice.cast::<T>().add(self.into_usize_unchecked()) }
            }

            unsafe fn get_unchecked_mut(
                self,
                slice: *mut #indexland::IndexSlice<#name, T>,
            ) -> *mut Self::Output {
                unsafe { slice.cast::<T>().add(self.into_usize_unchecked()) }
            }

            fn index(self, slice: & #indexland::IndexSlice<#name, T>) -> &Self::Output {
                &slice.as_slice()[#indexland::Idx::into_usize(self)]
            }

            fn index_mut(
                self,
                slice: &mut #indexland::IndexSlice<#name, T>,
            ) -> &mut Self::Output {
                &mut slice.as_mut_slice()[#indexland::Idx::into_usize(self)]
            }
        }
        #[automatically_derived]
        unsafe impl<E: ?Sized, S: ?Sized, C: ?Sized>
            #indexland::raw_index_container::GenericIndex<#name, E, S, C>
            for #compat
        {
            type Output = E;

            fn get(self, container: &C) -> Option<&Self::Output>
            where
                C: #indexland::raw_index_container::RawIndexContainer<Element = E, Slice = S>,
            {
                C::get(container, self.into_usize())
            }

            unsafe fn get_unchecked<FS, FR>(
                self,
                container: *const C,
            ) -> *const Self::Output
            where
                C: #indexland::raw_index_container::RawIndexContainer<Element = E, Slice = S>,
            {
                C::get_unchecked(container, self.into_usize())
            }

            fn index(self, container: &C) -> &Self::Output
            where
                C: #indexland::raw_index_container::RawIndexContainer<Element = E, Slice = S>,
            {
                C::index(container, self.into_usize())
            }

            fn get_mut(self, container: &mut C) -> Option<&mut Self::Output>
            where
                C: #indexland::raw_index_container::RawIndexContainerMut<Element = E, Slice = S>,
            {
                C::get_mut(container, self.into_usize())
            }

            unsafe fn get_unchecked_mut(self, container: *mut C) -> *mut Self::Output
            where
                C: #indexland::raw_index_container::RawIndexContainerMut<Element = E, Slice = S>,
            {
                C::get_unchecked_mut(container, self.into_usize())
            }

            fn index_mut(self, container: &mut C) -> &mut Self::Output
            where
                C: #indexland::raw_index_container::RawIndexContainerMut<Element = E, Slice = S>,
            {
                C::index_mut(container, self.into_usize())
            }
        }
    }
}

pub fn derive_arith_compat(
    indexland: &syn::Path,
    name: &syn::Ident,
    compat: &syn::Path,
) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl #indexland::idx::ArithCompat<#name> for #compat {
            fn to_idx(self) -> #name {
                #name::from_usize(Idx::into_usize(self))
            }
        }
    }
}

pub fn derive_default(ctx: &DeriveContextBase) -> TokenStream {
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

pub fn derive_clone(ctx: &DeriveContextBase) -> TokenStream {
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

pub fn derive_copy(ctx: &DeriveContextBase) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::marker::Copy for #name {}
    }
}

pub fn derive_add_assign(ctx: &DeriveContextBase) -> TokenStream {
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

pub fn derive_sub_assign(ctx: &DeriveContextBase) -> TokenStream {
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

pub fn derive_mul_assign(ctx: &DeriveContextBase) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::MulAssign for #name {
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }
    }
}

pub fn derive_div_assign(ctx: &DeriveContextBase) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::DivAssign for #name {
            fn div_assign(&mut self, rhs: Self) {
                *self = *self / rhs;
            }
        }
    }
}

pub fn derive_rem_assign(ctx: &DeriveContextBase) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::RemAssign for #name {
            fn rem_assign(&mut self, rhs: Self) {
                *self = *self % rhs;
            }
        }
    }
}

pub fn derive_eq(ctx: &DeriveContextBase) -> TokenStream {
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Eq for #name {}
    }
}

// The following impls could be shared because they dont rely on
// enum or newtype specifics but are currently only used by
// enum because for newtype it's more efficient to perform
// the operation on the base type.

pub fn derive_add(ctx: &DeriveContextBase) -> TokenStream {
    let self_as_idx = &ctx.self_as_idx;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Add for #name {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                #self_as_idx::from_usize(
                    #self_as_idx::into_usize(self) + #self_as_idx::into_usize(rhs),
                )
            }
        }
    }
}

pub fn derive_sub(ctx: &DeriveContextBase) -> TokenStream {
    let self_as_idx = &ctx.self_as_idx;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Sub for #name {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                #self_as_idx::from_usize(
                    #self_as_idx::into_usize(self) - #self_as_idx::into_usize(rhs),
                )
            }
        }
    }
}

pub fn derive_mul(ctx: &DeriveContextBase) -> TokenStream {
    let self_as_idx = &ctx.self_as_idx;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Mul for #name {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                #self_as_idx::from_usize(
                    #self_as_idx::into_usize(self) * #self_as_idx::into_usize(rhs),
                )
            }
        }
    }
}

pub fn derive_div(ctx: &DeriveContextBase) -> TokenStream {
    let self_as_idx = &ctx.self_as_idx;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Div for #name {
            type Output = Self;
            fn div(self, rhs: Self) -> Self::Output {
                #self_as_idx::from_usize(
                    #self_as_idx::into_usize(self) / #self_as_idx::into_usize(rhs),
                )
            }
        }
    }
}

pub fn derive_rem(ctx: &DeriveContextBase) -> TokenStream {
    let self_as_idx = &ctx.self_as_idx;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Rem for #name {
            type Output = Self;
            fn rem(self, rhs: Self) -> Self::Output {
                #self_as_idx::from_usize(
                    #self_as_idx::into_usize(self) % #self_as_idx::into_usize(rhs),
                )
            }
        }
    }
}

pub fn derive_partial_ord(ctx: &DeriveContextBase) -> TokenStream {
    let self_as_idx = &ctx.self_as_idx;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::PartialOrd for #name {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                #self_as_idx::into_usize(*self)
                    .partial_cmp(&#self_as_idx::into_usize(*other))
            }
        }
    }
}

pub fn derive_ord(ctx: &DeriveContextBase) -> TokenStream {
    let self_as_idx = &ctx.self_as_idx;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::cmp::Ord for #name {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                #self_as_idx::into_usize(*self)
                    .cmp(&#self_as_idx::into_usize(*other))
            }
        }
    }
}

pub fn derive_from_usize(ctx: &DeriveContextBase) -> TokenStream {
    let self_as_idx = &ctx.self_as_idx;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<usize> for #name {
            #[inline]
            fn from(v: usize) -> #name {
                #self_as_idx::from_usize(v)
            }
        }
    }
}

pub fn derive_from_self_for_usize(ctx: &DeriveContextBase) -> TokenStream {
    // !! Can't use self_as_idx here because self is `usize`. !!
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::convert::From<#name> for usize {
            #[inline]
            fn from(v: #name) -> usize {
                <#name as #indexland::Idx>::into_usize(v)
            }
        }
    }
}

pub fn derive_add_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Add<#ty> for #name {
            type Output = Self;
            fn add(self, rhs: #ty) -> Self::Output {
                self + #indexland::ArithCompat::<#name>::to_idx(rhs)
            }
        }
    }
}

pub fn derive_sub_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Sub<#ty> for #name {
            type Output = Self;
            fn sub(self, rhs: #ty) -> Self::Output {
                self - #indexland::ArithCompat::<#name>::to_idx(rhs)
            }
        }
    }
}

pub fn derive_mul_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Mul<#ty> for #name {
            type Output = Self;
            fn mul(self, rhs: #ty) -> Self::Output {
                self * #indexland::ArithCompat::<#name>::to_idx(rhs)
            }
        }
    }
}

pub fn derive_div_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Div<#ty> for #name {
            type Output = Self;
            fn div(self, rhs: #ty) -> Self::Output {
                self / #indexland::ArithCompat::<#name>::to_idx(rhs)
            }
        }
    }
}

pub fn derive_rem_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::Rem<#ty> for #name {
            type Output = Self;
            fn rem(self, rhs: #ty) -> Self::Output {
                self % #indexland::ArithCompat::<#name>::to_idx(rhs)
            }
        }
    }
}

pub fn derive_add_assign_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::AddAssign<#ty> for #name {
            fn add_assign(&mut self, rhs: #ty) {
                *self = *self + #indexland::ArithCompat::<#name>::to_idx(rhs);
            }
        }
    }
}

pub fn derive_sub_assign_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::SubAssign<#ty> for #name {
            fn sub_assign(&mut self, rhs: #ty) {
                *self = *self - #indexland::ArithCompat::<#name>::to_idx(rhs);
            }
        }
    }
}

pub fn derive_mul_assign_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::MulAssign<#ty> for #name {
            fn mul_assign(&mut self, rhs: #ty) {
                *self = *self * #indexland::ArithCompat::<#name>::to_idx(rhs);
            }
        }
    }
}

pub fn derive_div_assign_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::DivAssign<#ty> for #name {
            fn div_assign(&mut self, rhs: #ty) {
                *self = *self / #indexland::ArithCompat::<#name>::to_idx(rhs);
            }
        }
    }
}

pub fn derive_rem_assign_compat(ctx: &DeriveContextBase, ty: TokenStream) -> TokenStream {
    let indexland = &ctx.attrs.indexland_path;
    let name = &ctx.name;
    quote! {
        #[automatically_derived]
        impl ::core::ops::RemAssign<#ty> for #name {
            fn rem_assign(&mut self, rhs: #ty) {
                *self = *self % #indexland::ArithCompat::<#name>::to_idx(rhs);
            }
        }
    }
}
