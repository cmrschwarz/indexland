use proc_macro2::TokenStream;
use quote::quote;

pub fn derive_compatible(
    indexland: &syn::Path,
    name: &syn::Ident,
    compat: &syn::Path,
) -> TokenStream {
    quote! {
        #[automatically_derived]
        impl #indexland::idx::IdxCompatible<FooId> for #compat {
            fn idx_cast(self) -> FooId {
                #name::from_usize(Idx::into_usize(self))
            }
        }
        #[automatically_derived]
        unsafe impl<T>
            #indexland::index_slice_index::IndexSliceIndex<#name, T>
            for #compat
        {
            type Output = T;

            fn get(self, slice: & #indexland::IndexSlice<FooId, T>) -> Option<&Self::Output> {
                slice.as_slice().get(#indexland::Idx::into_usize(self))
            }

            fn get_mut(
                self,
                slice: &mut #indexland::IndexSlice<FooId, T>,
            ) -> Option<&mut Self::Output> {
                slice
                    .as_mut_slice()
                    .get_mut(#indexland::Idx::into_usize(self))
            }

            unsafe fn get_unchecked(
                self,
                slice: *const #indexland::IndexSlice<FooId, T>,
            ) -> *const Self::Output {
                unsafe { slice.cast::<T>().add(self.into_usize_unchecked()) }
            }

            unsafe fn get_unchecked_mut(
                self,
                slice: *mut #indexland::IndexSlice<FooId, T>,
            ) -> *mut Self::Output {
                unsafe { slice.cast::<T>().add(self.into_usize_unchecked()) }
            }

            fn index(self, slice: & #indexland::IndexSlice<FooId, T>) -> &Self::Output {
                &slice.as_slice()[#indexland::Idx::into_usize(self)]
            }

            fn index_mut(
                self,
                slice: &mut #indexland::IndexSlice<FooId, T>,
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
