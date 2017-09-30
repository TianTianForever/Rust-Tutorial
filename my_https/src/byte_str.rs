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
             bytes: Bytes::new(),
         }
     }

     /// Crates a new `ByteStr` from a static slice.
     pub fn from_static(value: &'static str) -> ByteStr {
         ByteStr {
             bytes: Bytes::from_static(value.as_bytes())
         }
     }

    /// This is the unchecked alternative to indexing the str.
    /// Return s unchecked subslice of str.
     pub unsafe fn from_utf8_unchecked(bytes: Bytes) -> ByteStr {
         ByteStr {
             bytes: bytes,
         }
     }  
}
impl ops::Deref for ByteStr {
    type Target = str;
    fn deref(&self) -> &str {
        let b: &[u8] = self.bytes.as_ref();
        unsafe {
            str::from_utf8_unchecked(b)
        }
    }
}
impl From<String> for ByteStr {
    fn from(src: String) -> ByteStr {
        ByteStr {
            bytes: Bytes::from(src),
        }
    }
}
impl From<ByteStr> for Bytes {
    fn from(src: ByteStr) -> Self {
        src.bytes
    }
}
impl<'a> From<&'a str> for ByteStr {
    fn from(src: &'a str) -> ByteStr {
        ByteStr { bytes: Bytes::from(src) }
    }
}