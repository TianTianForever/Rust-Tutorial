// A concrete type "Apple".
struct Apple;
struct Single(Apple);

#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    let c = SingleGen(12i32);
    let _aa = Single(Apple);
    let _s = Single(Apple);
    let d = SingleGen::<i32>(12);
    let _f64 = SingleGen::<f64>(1.123);
    let _char: SingleGen<char> = SingleGen('a');

    let _i32: SingleGen<i32> = SingleGen(12);
  
    let _a = SingleGen(Apple);
    let _u32 = SingleGen(0.11);
    println!("{:?}", _u32);
}
