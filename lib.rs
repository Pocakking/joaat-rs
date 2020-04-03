#![warn(unsafe_code)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use std::{
	default::Default,
	hash::{BuildHasherDefault, Hasher},
};

#[cfg(not(feature = "std"))]
use core::{
	default::Default,
	hash::{BuildHasherDefault, Hasher},
};

#[cfg(feature = "std")]
use std::collections::{HashMap, HashSet};

pub struct JoaatHasher(u32);

impl Default for JoaatHasher {
	#[inline]
	fn default() -> Self {
		Self(0)
	}
}

impl JoaatHasher {
	/// Create a Joaat hasher with an initial hash.
	#[inline]
	#[must_use]
	pub const fn with_initial_hash(hash: u32) -> Self {
		Self(hash)
	}
}

impl Hasher for JoaatHasher {
	#[inline]
	fn finish(&self) -> u64 {
		let mut hash = self.0;
		hash = hash.wrapping_add(hash.wrapping_shl(3));
		hash ^= hash.wrapping_shr(11);
		hash = hash.wrapping_add(hash.wrapping_shl(15));
		hash as _
	}

	#[inline]
	fn write(&mut self, bytes: &[u8]) {
		for byte in bytes.iter() {
			self.0 = self.0.wrapping_add(u32::from(*byte));
			self.0 = self.0.wrapping_add(self.0.wrapping_shl(10));
			self.0 ^= self.0.wrapping_shr(6);
		}
	}
}

/// A builder for default Joaat hashers.
pub type JoaatBuildHasher = BuildHasherDefault<JoaatHasher>;

/// Hashes bytes from an iterator.
#[inline]
#[must_use]
pub fn hash_iter<I: Iterator<Item = u8>>(input: I) -> u32 {
	let mut hasher = JoaatHasher::default();

	for byte in input {
		hasher.write_u8(byte);
	}

	hasher.finish() as _
}

/// Hashes text converting alphabetical characters to ASCII lowercase.
#[inline]
#[must_use]
pub fn hash_ascii_lowercase(bytes: &[u8]) -> u32 {
	hash_iter(bytes.iter().map(|c| c.to_ascii_lowercase()))
}

/// Hashes a slice of bytes.
#[inline]
#[must_use]
pub fn hash_bytes(bytes: &[u8]) -> u32 {
	let mut hasher = JoaatHasher::default();
	hasher.write(bytes);
	hasher.finish() as _
}

/// A `HashMap` using a default Joaat hasher.
#[cfg(feature = "std")]
pub type JoaatHashMap<K, V> = HashMap<K, V, JoaatBuildHasher>;

/// A `HashSet` using a default Joaat hasher.
#[cfg(feature = "std")]
pub type JoaatHashSet<T> = HashSet<T, JoaatBuildHasher>;

#[cfg(test)]
#[allow(clippy::unreadable_literal)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(hash_bytes(b""), 0);
		assert_eq!(hash_bytes(b"a"), 0xCA2E9442);
		assert_eq!(hash_bytes(b"b"), 0x00DB819B);
		assert_eq!(hash_bytes(b"c"), 0xEEBA5D59);
		assert_eq!(
			hash_bytes(b"The quick brown fox jumps over the lazy dog"),
			0x519E91F5
		);
		assert_eq!(
			hash_bytes(b"The quick brown fox jumps over the lazy dog."),
			0xAE8EF3CB
		);
	}
}
