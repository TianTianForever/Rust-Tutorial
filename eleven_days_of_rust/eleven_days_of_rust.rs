extern crate closure;
use closure::*;

fn main() {
    let greeting = "hello";
    let a = || {
        println!("I said {}.", greeting);
    };
    my_print(a);
    
    let a = |x| x * 5;
    println!("five of square: {}", square(a));
    let double = |x| x * 2;
    assert_eq!(call_with_one(double), 2);
    let one = |x| x + 1;
    assert_eq!(add_one(one), 2);
   
    let s = String::from("Taking a clusure");
    let s2 = move || s;
    print_string(s2);
   
    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }
    assert_eq!(x, 5);
}

