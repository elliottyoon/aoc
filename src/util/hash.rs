use bytes::Buf;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, Hash, Hasher};
use std::ops::BitXor;

/// Type alias for hash sets using `FxHasher`.
pub type FastSet<K> = HashSet<K, BuildFxHasher>;

pub trait FastSetBuilder<K> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn build<const N: usize>(array: [K; N]) -> Self;
}

impl<K: Eq + Hash> FastSetBuilder<K> for FastSet<K> {
    fn new() -> Self {
        Self::with_hasher(BuildFxHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildFxHasher)
    }

    fn build<const N: usize>(array: [K; N]) -> Self {
        let mut set = FastSet::with_capacity(N);
        for key in array {
            set.insert(key);
        }
        set
    }
}

/// Type alias for hash maps using `FxHasher`.
pub type FastMap<K, V> = HashMap<K, V, BuildFxHasher>;

pub trait FastMapBuilder<K, V> {
    fn new() -> Self;
    fn with_capacity(capacity: usize) -> Self;
    fn build<const N: usize>(array: [(K, V); N]) -> Self;
}

impl<K: Eq + Hash, V> FastMapBuilder<K, V> for FastMap<K, V> {
    fn new() -> Self {
        Self::with_hasher(BuildFxHasher)
    }

    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, BuildFxHasher)
    }

    fn build<const N: usize>(array: [(K, V); N]) -> Self {
        let mut map = Self::with_capacity(N);
        for (key, value) in array {
            map.insert(key, value);
        }
        map
    }
}

#[derive(Clone, Copy, Default)]
pub struct BuildFxHasher;

impl BuildHasher for BuildFxHasher {
    type Hasher = FxHasher;

    fn build_hasher(&self) -> Self::Hasher {
        FxHasher { hash: 0 }
    }
}

/// Assumes a 64-bit usize.
///
/// Checkout the [Firefox code](https://searchfox.org/mozilla-central/rev/633345116df55e2d37be9be6555aa739656c5a7d/mfbt/HashFunctions.h#109-153)
/// for a full description.
const K: u64 = 0x517cc1b727220a95;

pub struct FxHasher {
    hash: u64,
}

impl FxHasher {
    #[inline]
    fn add(&mut self, i: u64) {
        self.hash = self.hash.rotate_left(5).bitxor(i).wrapping_mul(K);
    }
}

impl Hasher for FxHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        let mut buf = &bytes[..];
        while buf.len() >= 8 {
            self.add(buf.get_u64());
        }
        while buf.len() >= 4 {
            self.add(buf.get_u32() as u64);
        }
        while buf.len() >= 2 {
            self.add(buf.get_u16() as u64);
        }
        if !buf.is_empty() {
            self.add(buf[0] as u64);
        }
    }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.add(i as u64);
    }

    #[inline]
    fn write_u16(&mut self, i: u16) {
        self.add(i as u64);
    }

    #[inline]
    fn write_u32(&mut self, i: u32) {
        self.add(i as u64);
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }

    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
}
