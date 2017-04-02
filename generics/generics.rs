// A concrete type "A".
struct Apple;
struct Single(Apple);

#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    let _s = Single(Apple);
    let _char: SingleGen<char> = SingleGen('a');

    let _i32: SingleGen<i32> = SingleGen(12);
  
    let _a = SingleGen(Apple);
    let _u32 = SingleGen(0.11);
    println!("{:?}", _u32);
}
