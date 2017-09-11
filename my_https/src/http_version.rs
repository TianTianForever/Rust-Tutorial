//! HTTP version.

use std::fmt;
/// Represents a version of the HTTP specify.
#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub struct Version(Http);

#[derive(PartialEq, PartialOrd, Copy, Clone, Eq, Ord, Hash)]
pub enum Http {
    Http09,    // HTTP/0.9
    Http10,    // HTTP/1.0
    Http11,    // HTTP/1.1
    Http2,     // HTTP/2.0
}

impl Version {
     /// `HTTP/0.9`
     pub const HTTP_09: Version = Version(Http::Http09);

     /// ｀HTTP/1.0｀
     pub const HTTP_10: Version = Version(Http::Http10);

     /// `HTTP/1.1`
     pub const HTTP_11: Version = Version(Http::Http11);

     /// `HTTP/2.0`
     pub const HTTP_2: Version = Version(Http::Http2);
}

// This 'HTTP/1.1' is used by default.
impl Default for Version {
    #[inline]
    fn default() -> Version {
        Version::HTTP_11
    }
}

impl fmt::Debug for Version {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Http::*;
        f.write_str(match self.0 {
            Http09 => "HTTP/0.9",
            Http10 => "HTTP/1.0",
            Http11 => "HTTP/1.1",
            Http2  => "HTTP/2.0",
        })
    }
}
