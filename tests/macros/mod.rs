//! We use integration tests for our macros to test calls coming from another crate.
//! That way only public definitions are accessible in the same way that it would be for a user.

pub mod index_array;

mod idx_newtype;

#[cfg(feature = "alloc")]
pub mod index_vec;

#[cfg(feature = "smallvec")]
mod index_small_vec;

#[cfg(feature = "arrayvec")]
mod index_array_vec;

#[cfg(feature = "indexmap")]
mod index_hash_map;

#[cfg(feature = "indexmap")]
mod index_hash_set;
