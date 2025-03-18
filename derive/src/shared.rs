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
        unsafe impl<T> #indexland::index_slice_index::IndexSliceIndex<#name, T> for usize {
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
        unsafe impl<S: ?Sized, R: ?Sized, C: ?Sized>
            #indexland::generic_index::GenericIndex<FooId, S, R, C>
            for usize
        {
            type Output = S;

            fn get<FS, FR>(
                self,
                container: &C,
                _len: usize,
                single: FS,
                _range: FR,
            ) -> Option<&Self::Output>
            where
                FS: Fn(&C, usize) -> Option<&S>,
                FR: Fn(&C, core::ops::Range<usize>) -> Option<&R>,
            {
                single(container, self.into_usize())
            }

            fn get_mut<FS, FR>(
                self,
                container: &mut C,
                _len: usize,
                single: FS,
                _range: FR,
            ) -> Option<&mut Self::Output>
            where
                FS: Fn(&mut C, usize) -> Option<&mut S>,
                FR: Fn(&mut C, core::ops::Range<usize>) -> Option<&mut R>,
            {
                single(container, self.into_usize())
            }

            unsafe fn get_unchecked<FS, FR>(
                self,
                container: *const C,
                _len: usize,
                single: FS,
                _range: FR,
            ) -> *const Self::Output
            where
                FS: Fn(*const C, usize) -> *const S,
                FR: Fn(*const C, core::ops::Range<usize>) -> *const R,
            {
                single(container, self.into_usize())
            }

            unsafe fn get_unchecked_mut<FS, FR>(
                self,
                container: *mut C,
                _len: usize,
                single: FS,
                _range: FR,
            ) -> *mut Self::Output
            where
                FS: Fn(*mut C, usize) -> *mut S,
                FR: Fn(*mut C, core::ops::Range<usize>) -> *mut R,
            {
                single(container, self.into_usize())
            }

            fn index<FS, FR>(
                self,
                container: &C,
                _len: usize,
                single: FS,
                _range: FR,
            ) -> &Self::Output
            where
                FS: Fn(&C, usize) -> &S,
                FR: Fn(&C, core::ops::Range<usize>) -> &R,
            {
                single(container, self.into_usize())
            }

            fn index_mut<FS, FR>(
                self,
                container: &mut C,
                _len: usize,
                single: FS,
                _range: FR,
            ) -> &mut Self::Output
            where
                FS: Fn(&mut C, usize) -> &mut S,
                FR: Fn(&mut C, core::ops::Range<usize>) -> &mut R,
            {
                single(container, self.into_usize())
            }
        }
    }
}
