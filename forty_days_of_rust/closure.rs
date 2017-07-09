// 'Fn' takes an immutable reference to any captured variable.
fn call_with_one<F>(func: F) -> usize
    where F: Fn(usize) -> usize {
    func(1)        
}

// 'FnMut' takes a mutable reference.
fn do_twice<F>(mut func: F)
    where F: FnMut() {
    func();
    func();
}

// 'FnOnce' by value.
fn consume_capture<F>(func: F)
    where F: FnOnce() -> String {
    println!("Consumed: {}", func());
    println!("Implement FnOnce");
}

fn main() {
    let  a = 1;
    let b = move || a + 1;
    assert_eq!(2, b());
   
    let mul = |x| x * x;
    assert_eq!(1, call_with_one(mul));
    
    let mut x: i32 = 1;
    {
        let add = || x += 2;
        do_twice(add);
    }
    assert_eq!(5, x);
    let string = String::from("consumed");
    let consume = move || string;
    consume_capture(consume);
}
