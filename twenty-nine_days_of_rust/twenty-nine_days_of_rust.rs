use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
enum IpAddrKind {
   V4,
   V6,
} 

struct IpAddr_v1 {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr_v2 {
    V4(String),
    V6(String),
}
/*
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IpAddr::V4(127, 0, 0, 1);
let lookback = IpAddr::V6(String::from("::1"));
*/
fn route(ip_type: IpAddrKind) {
    
}
/*
struct Ipv4Addr{ 
    fn new(a: u8, b: u8, c: u8, d: u8) -> Ipv4Addr {
        // doing something
    }
}
struct Ipv6Addr{ 
    fn new(a: u16, b: u16, c: u16, d: u16, e: u16, f: u16, f: u16, 
           g: u16, h: u16) -> Ipv4Addr {
        // doing something
    }
}
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
*/
/// ```
/// use std::net::Ipv6Addr;
///
/// let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff);
/// ```
fn main() {
    println!("Progresee is activity of today and the assurance tomorrow.");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let home = IpAddr_v1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr_v1 {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let home1 = IpAddr_v2::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr_v2::V6(String::from("::1"));
    assert_eq!(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)).is_loopback(), true);
    assert_eq!(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0x1)).is_loopback(), true);
}
