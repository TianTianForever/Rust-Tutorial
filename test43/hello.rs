//hello.rs
#![crate_type = "lib"]
#![crate_name = "hello"]
pub fn apple() {
    println!("callled hello's apple()");
}

fn orange() {
    println!("called hello's orange()");
}

pub fn banana() {
    println!("called hello's banana()");
}

pub fn indirect_access() {
    print!("called hello's 'indirect_access()', that \n"); 
    orange();
}
