extern crate errors;
use errors::*;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    ages: u32,
}
impl <'a> Person<'a> {
    fn take_name(&'a self) -> &'a str {
        self.name
    }
    fn take_age(&'a self) -> u32 {
        self.ages
    }
    // Compare ages.
    fn older(&'a self, a: &'a Person) -> bool {
        self.ages > a.ages
    }
}

fn main() {
    let person1 = Person {name: "Tom", ages: 23};
    let person2 = Person {name: "Sophic", ages: 25};
    output(parse_number(person1.name));
    let string = "1992";
    output(parse_number(string));
    println!("{:#?}", person1);
    println!("{:#?}", person2);
    println!("Tom is older than Sophic: {:?}",person1.older(&person2));
    let (x, y, z) = (Point::X, Point::Y, Point::Z);
    print_x_and_y(x, Number::One);
    print_x_and_y(y, Number::Two);
    print_x_and_y(z, Number::Empty);        
}
