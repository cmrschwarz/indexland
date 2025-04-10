#[cfg(all(feature = "std", feature = "derive"))]
mod std_derive;

#[cfg(feature = "derive")]
mod derive_no_std;

mod no_derive_no_std;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(all(feature = "arrayvec", feature = "derive"))]
mod arrayvec;

#[cfg(all(feature = "indexmap", feature = "derive"))]
mod index_hash_map;
