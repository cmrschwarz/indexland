#[cfg(all(feature = "std", feature = "derive"))]
mod std_derive;

#[cfg(feature = "derive")]
mod no_std;

mod no_derive_no_std;

#[cfg(feature = "smallvec")]
mod smallvec;

#[cfg(feature = "arrayvec")]
mod arrayvec;
