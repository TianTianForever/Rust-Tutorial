//! standard input

use std::io;

/*
fn read_input() -> io::Result<()> {
    let mut input = String::new();
    try!(io::stdin().read_line(&mut input));
    println!("You Types: {}",input.trim());
    Ok(())
}
*/

fn main() {
    //read_input();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read",n);
            println!("You types: {}", input);
        }
        Err(error) => println!("error: {}", error),
    }

/*
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Woring!");
    println!("You Typed: {}", input.trim());
*/
    
}
