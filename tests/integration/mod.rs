use core::hash::{BuildHasher, Hasher};

pub mod idx_manual;

pub mod index_array;

mod declarative_macro;

#[cfg(feature = "derive")]
mod derive_macro;

#[cfg(feature = "smallvec")]
mod index_small_vec;

#[cfg(feature = "arrayvec")]
mod index_array_vec;

#[cfg(feature = "indexmap")]
mod index_hash_map;

#[cfg(feature = "indexmap")]
mod index_hash_set;

#[derive(Default)]
pub struct OneByteHasher(u8);

impl Hasher for OneByteHasher {
    fn finish(&self) -> u64 {
        self.0 as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        if let Some(last) = bytes.last() {
            self.0 = *last;
        }
    }
}

impl BuildHasher for OneByteHasher {
    type Hasher = OneByteHasher;

    fn build_hasher(&self) -> Self::Hasher {
        OneByteHasher::default()
    }
}
