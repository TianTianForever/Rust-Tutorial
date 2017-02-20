fn main() {
    let pair = (1, 2);
    println!("Tell me about 'pair' is: {:?}", pair);
    
    // 'match' can be used to destructure a tuple.
    match pair {
        // Destructure the second.
        (1, y) => println!("frist is 1, y is {:?}",y),
        _ => println!("It doesn't match"),
    }
    // 11111111111111111111111111111111
}
