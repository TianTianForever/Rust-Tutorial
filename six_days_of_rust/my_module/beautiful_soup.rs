use std::fmt::Debug;

#[derive(Debug)]
pub struct Value<T>(pub T,);

impl <T> Value<T> where
    T: Debug {
    pub fn take_value(&self) -> &T {
        &self.0
    }
}

// Associate types
pub struct Container(pub i32, pub i32);

pub trait Contains {
   // Define generic types here which methods will be able 
   // to utilize.
   type A;
   type B;
   fn contains(&self, &Self::A, &Self::B) -> bool;
   fn first(&self) -> i32;
   fn last(&self) -> i32;
}
//trait Contains<A, B> {
//    fn contains(&self, &A, &B) -> bool;
//    fn first(&self) -> i32;
//    fn last(&self) -> i32;
//}

impl Contains for Container {
    // specify what types 'A' and 'b' are, If the 'input' type is 
    // 'Container(i32, i32)', the 'output' types are determined.
    type A = i32;
    type B = i32;
    fn contains(&self, number_one: &i32, number_two: &i32) -> bool {
        (&self.0 == number_one) && (&self.1 == number_two)
    } 
    
    fn first(&self) -> i32 {
        self.0
    }
    fn last(&self) -> i32 {
        self.1
    }
}

// Using associated types
pub fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}


