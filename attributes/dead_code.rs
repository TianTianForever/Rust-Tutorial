#[allow(dead_code)]
fn information() {
    println!("public information");
}

#[allow(dead_code)]
fn add(i: i32) -> i32 {
    1 + i
}

fn apply<T>(x: T) where 
    T: FnOnce() {
    x();
}

fn main() {
    let x = 7;
    let input = || println!("{}", x);
    apply(input);
}
