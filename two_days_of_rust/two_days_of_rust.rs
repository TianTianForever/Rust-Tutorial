use std::ops::Mul;
struct A;          // Concrete type 'A'.
struct Animal(A);  // Concrete type 'Animal'.

// The 'DefineType' is a concrete type.
struct DefineType(Animal);

// This 'GenType' is a generic type.
struct GenType<T>(T);

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Define a value_x method on the Point struct.
    fn value_x(&self) -> &f64 {
        &self.x                  // &self: Taking self by reference.
    }

    fn value_y(&self) -> f64 {
        self.y                   // self: Taking ownership of self.
    }
   
}

/*
// Define generic type 'Rectangle'.
#[derive(Eq)]
#[derive(PartialEq,Debug)]
struct Rectangle<T> {
    length: T,
    width: T,
}

// impl Rectangle for a generic type 'T'.
impl <T> Rectangle<T> {
    fn new(&self) -> T {
        self.length * self.width
    }
}
*/
struct Val(f64,); // Concrete type 'Val'.
impl Val {
    // Define a 'take_value' method on the 'Val' struct.
    fn take_value(&self) -> &f64 {
        &self.0
    }
}

// Define generic type 'ValGen'.
struct ValGen<T>(T,);

// impl of ValGen for a generic type 'T'.
impl <T> ValGen<T> {
    fn take_value(&self) -> &T {
        &self.0
    }   
}


// impl of GenType where explicitly specify type parameter.
impl GenType<i32> { }       // Specify integer type 'i32'.
impl GenType<u32> { }       // Specify 'u32'.

impl GenType<A> { }         // Specify 'A'.
impl GenType<Animal> { }    // Specify 'Animal' as defined above.

// impl of GenType for a generic type 'T'.
impl <T> GenType<T> { }

#[allow(unused_variables)]
// Define a function nemed to '_owner' that takes an argument 's' of type 'Animal'.
fn _owner(a: Animal) { }

fn _type(s: DefineType) { }

// Define a function 'generic' that takes an argument 't' of type 'GenType'.
fn generic<T>(t: GenType<T>) { }


fn main() {
    // Using the  non-generic function.
    _owner(Animal(A));

    _type(DefineType(Animal(A)));

   // Using the Generic function.
   generic(GenType(A));
   generic(GenType(Animal));

   // Explicitly specified type parameter 'String' to 'generic()'.
   generic::<&str>(GenType("Explicitly"));

   // Implicitly specified type parameter 'integer' to 'generic()'.
   generic(GenType(6));

   // Create a variable '_integer' of type 'GenType<i32>' and give it the value 'GenType(32)'.
   let _integer: GenType<i32> = GenType(32);
   
   let point = Point {x: 1.1, y: 1.0};
   println!(
       "The Point x:{}, y: {}",
       point.value_x(), point.value_y()

   );
  
   let my_value = Val(1.1111111111);
   let my_value2 = ValGen(3u32);
   println!(
       "my_value is {}, my_value2 is {}", 
        my_value.take_value(), my_value2.take_value()

   );

/*
   let rectangle1 = Rectangle {
       length: 4i32, 
       width: 4i32
   };
   let rectangle2 = Rectangle {
       length: 1.11111f64,
       width: 2.222222f64,
   };
   println!(
       "The area of the rectangle1 is {}",
       rectangle1.area()
    );
   println!(
       "The area of the rectangle2 is {}",
       rectangle2.area()
   );
*/
}

