//!  URI component of request and response lines
use byte_str::ByteStr;
use std::{fmt, u8, u16};
use std::ascii::AsciiExt;
use std::hash::{Hash, Hasher};
use std::error::Error;
use std::str::{self, FromStr};
use std::error;
///  The URI is structured as follows:
/// 
/// abc://username:password@example.com:123/path/data?key=value&key1=value1#fragid1
/// |-|   |-------------------------------||--------| |-------------------| |-----|
///  |                  |                       |               |              |
/// scheme          authority                 path            query         fragment
/// 
#[derive(Clone)]
pub struct Uri {
    scheme: Scheme,
    authority: Authority,
    path_and_query: PathAndQuery,
}

/// Represents the scheme component of a URI.
#[derive(Clone, Debug)]
pub struct Scheme {
     inner: Scheme2,
}

#[derive(Copy, Clone, Debug)]
pub enum Scheme2<T = Box<ByteStr>> {
    None,
    Standard(Protocol),
    Other(T),
}

#[derive(Copy, Clone, Debug)]
pub enum Protocol {
    Http,
    Https,
}

/// Represents the Authority component of a URI.
#[derive(Clone, Debug)]
pub struct Authority {
    data: ByteStr,
}

/// Represents the Path component of a URI.
#[derive(Clone, Debug)]
pub struct PathAndQuery {
    data: ByteStr,
    query: u16,
}

/// This various parts of Uri.
/// This struct is used to provide to and retrive from a URI.
pub struct Parts {
    /// The scheme component of a URI.
    pub scheme: Option<Scheme>,
    /// The authority component of a URI.
    pub authority: Option<Authority>,
    /// The pathandquery component of a URI.
    pub path_and_query: Option<PathAndQuery>,
}

/// Handling error.
/// An error resulting from a failed attempt to construct a URI.
#[derive(Debug)] 
pub struct InvalidUri(ErrorKind);

#[derive(Debug)]
pub struct IvalidUriBytes(InvalidUri);

#[derive(Debug)]
pub struct IvalidUriParts(InvalidUri);

#[derive(Debug, Eq, PartialEq)]
enum ErrorKind {
    InvalidUriChar,
    InvalidScheme,
    InvalidAuthority,
    InvalidFormat,
    AuthorityMissing,
    PathAndQueryMissing,
    TooLong,
    Empty,
    SchemeTooLong,
}
// u16::MAX is reserved for None. 
// u16::MAX is 65535.
const MAX_LEN: usize = (u16::MAX - 1) as usize;

