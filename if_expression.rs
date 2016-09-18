fn main() {
    let a = 5;
    if a == 5 {
        println!("x is five");
    }else if a == 6 {
        println!("x is six");
    }else{
        println!("x is seven");
    }
    let x = 6;
    let y = if x == 6{ 10 }else{15};
    println!("y is {}",y);
}
