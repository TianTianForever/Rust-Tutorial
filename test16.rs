use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    //Latitude
    lat: f32,
    //Longitude
    lon: f32,
}

// Implement 'Display' for City.
impl Display for City {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_city = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_city = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f,"{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_city, self.lon.abs(), lon_city)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City {name: "ZheJang", lat: 123.111111111, lon: 32.111111111111},
        City {name: "HangZhou", lat: 23.3224432, lon: -23.242414},
        City {name: "BeiJing", lat: 123.1232222, lon: -123.1213132},
    ].iter() {
        println!("{}",city);
    }

    for color in [
        Color {red: 123, green: 22, blue: 112},
        Color {red: 123, green: 11, blue: 45},
        Color {red: 0, green: 0, blue: 0},
    ].iter() {
        println!("{:?}", *color); 
    }
}
