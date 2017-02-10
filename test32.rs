fn main() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        // Match a single value.
        1 => println!("One"), 
        2 => println!("Two"),
        3 => println!("Tree"),
        4 | 5 | 6 | 7 | 12 => println!("This is a prime"),
        13 ... 100 => println!(" get the number"),
        _ => println!("ain't special"),
    }
    let boolean = true;
    // Match is an expression too.
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
} 
