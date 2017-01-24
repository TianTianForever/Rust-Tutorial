use std::mem;

// This fuction borrows a slice.
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}",slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // Fixed-size array -- (type signature is superfluous)
    let x: [i32; 6] = [1, 2, 3, 4, 5, 6];

    // all elements can be initalized to the same value.
    let y: [i32; 500] = [0; 500];
    
    // Indexing start at 0.
    println!("first element of the array: {}", x[0]);
    println!("second element of the array: {}", x[1]);

    // 'len' returns the size of the array.
    println!("array size: {}", x.len());

    // Arrays are stack allocated.
    println!("array occupies {} bytes", mem::size_of_val(&x));  
   
    // Arrays can be automatically borrowed as slices. 
    println!("borrow the whole array as a slice");
    analyze_slice(&x);

    // Slices can point to a section of an array.
    println!("borrow a section of the array as a slice");
    analyze_slice(&y[0 .. 499]);

   // Out of bound indexing yields a panic.
    println!("{}", x[7]);

}
