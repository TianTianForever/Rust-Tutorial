fn add_one(x: i32) -> i32 {
    x + 1
}

fn apply<F>(f: F, y: i32) -> i32
    where F: Fn(i32) -> i32
{
    f(y) * y
}

fn factory(x: i32) -> Box<Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}

//diverging function
fn diverges() -> ! {
    panic!("This function never returns!");
}

fn main() {
    let function: fn(i32) -> i32 = add_one;
    let fc = add_one(3i32) * 3;
    let fc1 = apply(add_one,3);
    let fc2 = apply(function, 3);
    println!("{}, {}, {}",fc, fc1, fc2);
     
    let closure = |x: i32| x + 1;
    let cl = closure(3i32) * 3;
    let cl1 = apply(closure, 3);
    let cl2 = apply(|x| x + 1, 3);
    println!("{}, {}, {}",cl, cl1, cl2);

    let box_fn = factory(2i32);
    let b0 = box_fn(2i32) * 3;
    let b1 = (*box_fn)(2i32) * 3;
    let b2 = (&box_fn)(2i32) * 3;
    println!("{}, {}, {}",b0, b1, b2);

    let add_num = &(*box_fn);
    let translate: &Fn(i32) -> i32 = add_num;
    let t0 = add_num(2i32) * 3;
    let t1 = apply(add_num, 3);
    let t2 = apply(translate, 3);
    println!("{}, {}, {}",t0, t1, t2);

    //let x: i32 = diverges();
    assert_eq!(2, add_one(1));

    let num = 5;
    let plus_num = |x: i32| x + num;
     
    let mut number = 5;
        {
            let mut add_num = move |x: i32| number += x;
            add_num(5);
        }
     assert_eq!(5, number);
}
