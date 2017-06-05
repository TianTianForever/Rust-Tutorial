extern crate my_lifetimes as my;
use my::*;

fn main() {
    let s = String::from("I don't want you to hurt yourself.");
    let word = first_word(&s);
    println!("The word will get value {}", word);
    let s1 = String::from(
        "I know how busy you must be and naturally I wouldn't want to take up too much of your time.");
    let words = &s1[0..];
    println!("{}",words);
   
    // Imutable reference.
    let x = 1;
    let y = 2;
    println!("{:?}",return_refer_value(&x, &y));
   
    // Mutable reference.
    let mut z = 3;
    mut_refer_value(&mut z);

    // generic type 'GenValue'.  
    let value = GenValue(&x, &y);
    gen_ref_print(&value);
 
    let height: f64 = 1.0;
    let width: f64 = 2.0;
    let rectangle = Rectangle{
        height: &height,
        width: &width,
    };
    gen_ref_print(&rectangle);

    // 'Double' is concrete type and explicitly takes 'Empty' and 'Null'.
    let concrete = Concrete(Empty, Null);

    let _String = Generic("Implicitly", "specify");
    let _float = Generic(9.0000009f64, 21.1212f64);
    generic_function::<i32>(Generic(1, 2));
}
