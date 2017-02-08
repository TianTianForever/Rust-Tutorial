fn main() {
    // A counter variable
    let mut number = 0;
    while number < 101 {
        if number % 10 == 15 {
            println!("ten");
        } else if number % 3 == 0 {
            println!("tree");
        } else if number % 5 == 0 {
            println!("five");
        } else {
            println!("{}", number);
        }
        // Increment counter
        number += 1;
    }
} 
