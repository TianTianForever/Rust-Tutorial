#![crate_type="lib"]
#![crate_name="options_with_results"]
use std::num::ParseIntError;
use std::result;

type Result<T> = std::result::Result<T, String>;
pub fn multiplication_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
       // Convert the 'Option' to a 'Result' if there is a value.
       .ok_or("Please use a vector with at least one element.".to_owned())

       // Parse and return 'Result<T, ParseIntError>'.
       .and_then(|x| x.parse::<i32>()

                      // To map any errors 'parse' yields to 'string'.
                      .map_err(|e| e.to_string())
      
                      .map(|i| 2 * i))
}

pub fn multiplication_last(vec: Vec<&str>) -> Result<i32> {
    vec.last().ok_or("please use a vector with at least one element.".to_owned())
              .and_then(|l| l.parse::<i32>()
              .map_err(|e| e.to_string())
              .map(|x| 2 * x))
}

pub fn print_result_frist(result: Result<i32>) {
    match result {
        Ok(o) => println!("multiplication_first: {:?}", o),
        Err(e) => println!("Error: {:?}", e),
    }
}
pub fn print_result_last(result: Result<i32>) {
    match result {
        Ok(n)   => println!("multiplication_last: {:?}", n),
        Err(e)  => println!("Error: {:?}", e),
    }
}
pub fn is_work() {
    println!("work");
}
