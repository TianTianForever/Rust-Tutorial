// Globals are declared outside all other scopes.
static LANGUAGE: &'static str = "rust";
const NUMBER: i32 = 11;

fn is_big(n: i32) -> bool {
    // Access constant in some function.
    n > NUMBER
}

fn main() {
    let n = 16;
    // Access constant in the main thread.
    println!("This is {}",LANGUAGE);
    println!("the number is {}",NUMBER);
    println!("{} is {}",n, if is_big(n) { "big" } else { "small" });
    
    //NUMBER = 5; // Error! Cannot modify a 'const'.
   
}
