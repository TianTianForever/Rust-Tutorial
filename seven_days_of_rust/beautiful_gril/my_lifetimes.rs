// Using explicit lifetime annotations.
// Function lifetimes
pub fn pass_x_and_y<'a, 'b>(x: &'a i32, y: &'b i32) -> 
    (&'a i32, &'b i32) {
    (x,y)
}

// Unit tuple lifetimes
pub struct Value(pub i32);
impl Value {
    pub fn take_value<'a>(&'a self){
        println!("Take value {}",self.0);
    }
   
    pub fn add_value<'a>(&'a mut self) {
        self.0 += 1
    }
}

// Stuct lifetimes
#[derive(Debug)]
pub struct Person<'a> {
   pub Ages: &'a u32,
   pub Height: &'a f64,
   pub Weight: &'a f64,
}

