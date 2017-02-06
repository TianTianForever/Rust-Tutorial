fn main() {
/*
    let number = 10;
    if number < 0 {
        println!("{} is negative", number);
    } else if number > 0 {
         println!("{} is positive", number);
    } else {
        println!("{} is zero",number);
    }
    
    let big_number = if number < 10 && number > -10 {
        println!(" This is a small number");
        10 / 10
    } else {
        println!("This is a big number");
        10 * 10
    };
    println!("number: {}, big_number: {:?}", number, big_number);
*/
    let mut count = 0u32;
    println!("Let's count until infinity");
    
    // Infinite loop
    loop {
        count += 1;
        if count == 5 {
            println!("five");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 10 {
            println!("ten");
            // Exit this loop
            break;
        }
    }
} 

  
