fn main() {
    println!("learning fuction syntax");
    print_number(5);
    say_hello();
    sum(9,5);
    println!("{}", add_one(9));  //print "10"
    diverges();
}

fn say_hello(){
    println!("hello world");
}

fn print_number(x: i32) {
    println!("x is: {}",x);
}

fn sum(x: i32, y: i32){
    println!("sum is: {}",x+y);
}

fn add_one(x: i32) -> i32 {
    x+1
}

//diverging fuction
fn diverges() -> ! {
    panic!("This fuction never returns!");
}
