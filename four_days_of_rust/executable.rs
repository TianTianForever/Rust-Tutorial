extern crate four_days_of_rust as beautiful_gril;

// Non-copyable type.
struct Empty;
struct Null;

// A trait generic over 'T'.
trait MyDrop<T> {
    fn my_drop(self, _: T);
}

// Implment 'MyDrop' for any generic parameter 'T' and caller 'A'.
impl <T, U> MyDrop<T> for U {
   // This method takes ownership of both passed arguments.
   fn my_drop(self, _: T) {

   }
}


fn main() {
    let number = vec![1,23123,121,12,123,112,124];
    println!("The number is {:?}",number);

    let result = beautiful_gril::largest(&number);
    println!("The largest number is {}", result);

    let result2 = beautiful_gril::smallest(&number);
    println!("The smallest number is {}", result2);

    let empty = Empty;
    let null = Null;
    // Deallocate 'empty' and 'null'.
    empty.my_drop(null);

    let string = "compare";
    beautiful_gril::compare_prints(&string);    
    
    let array = [1, 3, 5, 7, 9];
    let vec = vec![1, 3, 5, 7, 9];
    beautiful_gril::compare_types(&array, &vec);   
    
   //beautiful_gril::compare_prints(&array);

   let value = beautiful_gril::Value(1.0f64);
   //value.take_value();
}
