use std::num::ParseIntError;

// Using 'unwrap()'.
fn double_number(s: &str) -> i32 {
    2 * s.parse::<i32>().unwrap()
}

// Using 'ParseIntError'.
fn double_number_v1(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(t) => Ok(2 * t),
        Err(e) => Err(e),
    }
}
// Using 'ParseIntError' and 'map()'
fn double_number_v2(s: &str) -> Result<i32, ParseIntError> {
    s.parse::<i32>().map(| a| 2 * a)
}
fn my_print(p: Result<i32, ParseIntError>) {
    match p {
        Ok(n)  => println!("n is {:?}", n),
        Err(e) => println!("e is {:?}", e),
    }
}
fn main() {
    let s = double_number("10");
    assert_eq!(s, 20);
    println!("{:?}", s);
    println!("{:?}", double_number_v1("1111"));
    println!("{:?}", double_number_v1("sssssss"));
    println!("{:?}", double_number_v2("2222"));
    println!("{:?}", double_number_v2("aaaaaaa"));
    let p = double_number_v2("222222222222");
    my_print(p);
    // Error!!!
    //let s1 = double_number("ssss");
    //println!("{:?}", s1);
}
