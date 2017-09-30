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
#[derive(Copy, Clone, Debug)]
pub struct Scheme {
     inner: Scheme2,
}

#[derive(Copy, Clone, Debug)]
pub enum Scheme2 {
    None,
    Standard(Protocol),
}

#[derive(Copy, Clone, Debug)]
pub enum Protocol {
    Http,
    Https,
}

/// Represents the Authority component of a URI.
#[derive(Copy, Clone, Debug)]
pub struct Authority {

}

/// Represents the Path component of a URI.
#[derive(Copy, Clone, Debug)]
pub struct PathAndQuery {

}