pub mod idx_manual;

pub mod index_array;

mod declarative_macro;

#[cfg(feature = "derive")]
mod derive_macro;

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
