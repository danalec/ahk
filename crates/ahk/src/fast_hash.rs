//! A hash that is fast, for the tables of the runtime.
//!
//! The tables of the built-ins are looked up in every call of a built-in,
//! and the hash of the standard library is slow for the short names that
//! those tables carry.
//!
//! The hash is the one of FxHash: the bytes of the name are mixed with a
//! multiplication and a rotation, and it is good enough for the tables of
//! the runtime -- the names of the built-ins are not chosen by whoever
//! writes the script.

use core::hash::{BuildHasherDefault, Hasher};

/// The seed of the hash: a number with a bit in every byte of it.
const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

/// The hash of the names of the runtime.
#[derive(Debug, Clone, Copy, Default)]
pub struct FastHasher {
    hash: u64,
}

impl Hasher for FastHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.hash = (self.hash.rotate_left(5) ^ (*byte as u64)).wrapping_mul(SEED);
        }
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.hash = (self.hash.rotate_left(5) ^ i).wrapping_mul(SEED);
    }

    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write_u64(i as u64);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }
}

/// The builder of the hash of the names of the runtime.
pub type FastHasherBuilder = BuildHasherDefault<FastHasher>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn the_hash_of_a_name_is_the_same_every_time() {
        let mut first = FastHasher::default();
        first.write(b"StrLen");
        let mut second = FastHasher::default();
        second.write(b"StrLen");
        assert_eq!(first.finish(), second.finish());
    }

    #[test]
    fn the_names_of_the_built_ins_are_not_in_the_same_place() {
        let mut map: HashMap<&str, u32, FastHasherBuilder> =
            HashMap::with_hasher(FastHasherBuilder::default());
        map.insert("StrLen", 1);
        map.insert("InStr", 2);
        map.insert("SubStr", 3);
        assert_eq!(map.get("StrLen"), Some(&1));
        assert_eq!(map.get("InStr"), Some(&2));
        assert_eq!(map.get("SubStr"), Some(&3));
        assert_eq!(map.get("StrGet"), None);
    }
}
