// Concrete type 'A'
struct A;

// Generic type 'GenericVal'.
struct GenericVal<T>(T,);

// impl of GenericVal where we explicitly expcify type parameters

// Explicitly 'i32'
impl GenericVal<i32> {} 

// Explicitly 'f64'
impl GenericVal<f64> { }

// Explicitly 'char'
impl GenericVal<char> { }

struct Val {
    val: f64
}

struct GenVal<T> {
    gen_val: T
}

// Implement of Val
impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

// Implement of GenVal for a generic type 'T'

impl <T> GenVal<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 1.0 };
    
    let y = GenVal { gen_val: 3i32 };
    
    println!("{}, {}", x.value(), y.value());
}
