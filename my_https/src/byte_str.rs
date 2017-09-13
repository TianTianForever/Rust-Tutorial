//! Provides abstractions for working with bytes.
use bytes::Bytes;
use std::{ops, str};

/// `Bytes` is an efficient container for storing and operating
/// on continguous slices of memory.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ByteStr {
    bytes: Bytes,
}

impl ByteStr {
    /// Create `ByteStr` type instance.
     pub fn new() -> ByteStr {
         ByteStr {
             bytes: Bytes::new()
         }
     }

     /// Crates a new `ByteStr` from a static slice.
     pub fn from_static(value: &'static str) -> ByteStr {
         ByteStr {
             bytes: Bytes::from_static(value.as_bytes())
         }
     }
}