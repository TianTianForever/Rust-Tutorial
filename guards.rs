fn main() {
    let pair = (2, -2);
    
    // A match guard can be added to filter the arm.  
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, y) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation"),
    }
  
    let point = (0, 1);
    match point {
        (0, y) => println!("First is '0' and 'y' is '{:?}'",y),
        (x, 0) => println!("'x' is '{:?}' and last is '0'", x),
        _      => println!("It doesn't matter what they are"),
    }

    enum Color {
        Red,
        Green,
        Blue,
        RGB(u32, u32, u32),
    }

    let color = Color::RGB(111, 121, 11);
    match color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
        Color::RGB(r, g, b) => println!("Red is {}, Green is {}, blue is {}",r, g, b),
    }

    struct Origin {
        x: u32,
        y: u32,
        point: (u32, u32),
    }
    let origin = Origin{x: 0, y: 0, point: (1, 1)};
    let Origin {x: a, y: b, point: (c, d)} = origin;
    println!("x is {}, y: is {}, c is {}, d is {}", a, b, c, d);
    
    let Origin {x: i, y: j, point: k} = origin;
    println!("x is {}, y is {} and point is {:?}", i, j, k);
}
