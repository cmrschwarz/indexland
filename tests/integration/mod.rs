#[cfg(feature = "std")]
mod std;

#[cfg(feature = "derive")]
mod no_std;

mod no_derive_no_std;
