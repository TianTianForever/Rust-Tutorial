fn main() {
    let x = 123u32;

    // If the last expression of the ends with a semicolon ';', the return value
    // will be ' () '.
    let y = {
        let x_squared = x * x;
        let x_add = x + x_squared;
        x_add + x_squared
    };
    
    let z = {
        // The semicolon suppresses this expression and '()' is assigned to 'z'.
        x * x;
    };
    println!("There are some values: x: {}, y: {}, z: {:?}", x, y, z);
}   
