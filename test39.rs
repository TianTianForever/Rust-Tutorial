use my::WhiteBox as wb;
use my::BlackBox as bb;
mod my {
    pub struct WhiteBox<T> {
        pub contents: T,
    }
   #[allow(dead_code)]
   pub struct BlackBox<T> {
       contents: T,
   }

   impl<T> BlackBox<T> {
       pub fn new(contents: T) -> BlackBox<T> {
           BlackBox {
               contents: contents, 
           }
       }
   }
}
fn main() {
    // let white_box = my::WhiteBox { contents: "public information" };
    let white_box = wb { contents: "public information" };
    println!("The white box contains: {}", white_box.contents);
  
    //let black_box = my::BlackBox::new("classified information");
    let black_box = bb::new("classified information");
    //println!("black box contains: {}",black_box.contents);
}   // A private field of a public struct cannot be accessed.
