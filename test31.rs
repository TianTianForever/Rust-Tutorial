fn main() {
    // 'number' will take the values: 1, 2, 3 ..., 100 in each iteration.
    for number in 1 .. 101 {
        if number % 15 == 0 {
            println!("fifteen");
        } else if number % 5  == 0 {
            println!("five");
        } else if number % 10 == 0 {
            println!("ten");
        } else {
            println!("number is: {}",number);
        }
    }
} 
