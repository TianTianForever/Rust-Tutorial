fn main() {
    // s is store date on the stack!
    let s = "hello world!";
    println!("{}",s);
    // s2 is store date on the heap!
    let s2 = String::from("Hello World!");
    println!("{}",s2);
}
