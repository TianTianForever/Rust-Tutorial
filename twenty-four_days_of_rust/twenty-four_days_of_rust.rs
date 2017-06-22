extern crate options_with_results;
use options_with_results::*;

fn multiplication(vec: Vec<&str>) -> i32 {
    // If vector is empty
    let first = vec.first().unwrap();
    
    // If the element don't parse to a number
    2 * first.parse::<i32>().unwrap()
}
fn add(vec: Vec<&str>) -> i32 {
    let last = vec.last().unwrap();
    1 + last.parse::<i32>().unwrap()
}

fn main() {
    let vector = vec!["1993", "1991", "1992"];
    let strings = vec!["23", "25", "24"];
    //let empty_vector = vec![];
    println!("vector: {:?}", multiplication(vector));
    //println!("empty: {:?}", multiplication(empty_vector)); // panic
    println!("string: {:?}", multiplication(strings));
    //assert_eq!(25, add(strings));
    is_work();
    let vec = vec!["1992", "1993"];
    print_result_frist(multiplication_first(vec));
    let vec1 = vec!["1992", "1993"];
    print_result_last(multiplication_last(vec1));
}
