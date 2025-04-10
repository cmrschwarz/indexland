use core::hash::{BuildHasher, Hasher};

/// A dummy hasher for hashing indices as themselves.
/// Do not use this if collision attacks are relevant for you.
/// This does not support non integer types like `&str` and will panic on use.
#[derive(Clone, Copy, Debug, Default)]
pub struct IdentityHasher {
    hash: u64,
    #[cfg(debug_assertions)]
    accessed: bool,
}

impl BuildHasher for IdentityHasher {
    type Hasher = Self;
    fn build_hasher(&self) -> Self::Hasher {
        Self::default()
    }
}

impl Hasher for IdentityHasher {
    fn finish(&self) -> u64 {
        // cfg needed because self.accessed doesn't exist in release mode
        #[cfg(debug_assertions)]
        debug_assert!(
            self.accessed,
            "IdentityHasher: finish() called before writing"
        );
        self.hash
    }
    fn write(&mut self, _: &[u8]) {
        panic!("IdentityHasher: writing non integer types is not supported")
    }
    fn write_u64(&mut self, n: u64) {
        #[cfg(debug_assertions)]
        {
            debug_assert!(
                !self.accessed,
                "IdentityHasher: write was called twice"
            );
            self.accessed = true;
        }
        self.hash = n;
    }
    fn write_u8(&mut self, n: u8) {
        self.write_u64(u64::from(n));
    }
    fn write_u16(&mut self, n: u16) {
        self.write_u64(u64::from(n));
    }
    fn write_u32(&mut self, n: u32) {
        self.write_u64(u64::from(n));
    }
    fn write_usize(&mut self, n: usize) {
        self.write_u64(n as u64);
    }
    fn write_i8(&mut self, n: i8) {
        #[allow(clippy::cast_sign_loss)]
        self.write_u64(n as u64);
    }
    fn write_i16(&mut self, n: i16) {
        #[allow(clippy::cast_sign_loss)]
        self.write_u64(n as u64);
    }
    fn write_i32(&mut self, n: i32) {
        #[allow(clippy::cast_sign_loss)]
        self.write_u64(n as u64);
    }
    fn write_i64(&mut self, n: i64) {
        #[allow(clippy::cast_sign_loss)]
        self.write_u64(n as u64);
    }
    fn write_isize(&mut self, n: isize) {
        #[allow(clippy::cast_sign_loss)]
        self.write_u64(n as u64);
    }
}

#[cfg(test)]
mod tests {
    use super::IdentityHasher;
    use core::hash::Hasher;

    #[test]
    fn basic_write() {
        let mut h1 = IdentityHasher::default();
        h1.write_u8(42);
        assert_eq!(42, h1.finish());

        let mut h2 = IdentityHasher::default();
        h2.write_u16(42);
        assert_eq!(42, h2.finish());

        let mut h3 = IdentityHasher::default();
        h3.write_u32(42);
        assert_eq!(42, h3.finish());

        let mut h4 = IdentityHasher::default();
        h4.write_u64(42);
        assert_eq!(42, h4.finish());

        let mut h5 = IdentityHasher::default();
        h5.write_usize(42);
        assert_eq!(42, h5.finish());

        let mut h6 = IdentityHasher::default();
        h6.write_i8(42);
        assert_eq!(42, h6.finish());

        let mut h7 = IdentityHasher::default();
        h7.write_i16(42);
        assert_eq!(42, h7.finish());

        let mut h8 = IdentityHasher::default();
        h8.write_i32(42);
        assert_eq!(42, h8.finish());

        let mut h9 = IdentityHasher::default();
        h9.write_i64(42);
        assert_eq!(42, h9.finish());

        let mut h10 = IdentityHasher::default();
        h10.write_isize(42);
        assert_eq!(42, h10.finish());
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "IdentityHasher: write was called twice")]
    fn double_usage() {
        let mut h = IdentityHasher::default();
        h.write_u8(42);
        h.write_u8(43);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(
        expected = "IdentityHasher: writing non integer types is not supported"
    )]
    fn string_hash_attempt() {
        let mut h = IdentityHasher::default();
        h.write("asdf".as_bytes());
    }
}
