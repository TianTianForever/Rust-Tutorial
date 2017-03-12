fn main() {
    // Increment via closures and functions.
    fn functions (i: i32) -> i32 {
        i + 1     
    }

   let closure_annotated = |i: i32| -> i32 { i + 1 };
   let closure_inferred  = |i     |        { i + 1 };
   let i = 1;
   // Call the cloures and functions.
   println!("cloure_annotated: {}", closure_annotated(i));
   println!("closure_inferred: {}", closure_inferred(i));
   println!("functions: {}", functions(i));
}
