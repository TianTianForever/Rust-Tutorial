// Similarly , a struct can be destructured as shown.
fn main() {
    struct Point {
        x: (u32, u32),
        y: u32,
    }
    // Destructure the members of the struct.
    let point = Point{x: (1, 2), y: 3};

    let Point {x: (a, b), y} = point;
    println!("a = {}，b = {}, c = {}",a, b, y);
    // You can destructure  struct and rename variables.
    // the order is not important
    let Point {x: i, y: j} = point;
    println!("i = {:?}, j = {:?}", i, j);
    // and you can also ignore some variables.
   
    let Point{y, ..} = point;
    println!("y = {:?}", y);
    // This will give an error: pattern does not mention field 'x'.
    //let Point {y} = point;

    struct Person {
        age: u32,
        height: u32,
        weight: u32,
    }
    
    // Destructure the members of the struct.
    let person = Person {age: 22, height: 170, weight: 125};
    let Person {age: x, height: y, weight: z} = person;
    println!("age is {},height is {}, weight is {}",x, y, z);
    
    // You can destructure struct and rename variables.
    let Person {age: a, height: h, weight: w} = person;
    println!("a is {}, h is {}, w is {}", a, h, w);
    
   // and you can also ignore some variables.
   let Person {age: a, ..} = person;
   println!("age is {}", a);
   let Person {height: h, ..} = person;
   println!("height is {}", h);
   let Person {weight: w, ..} = person;
   println!("weight is {}", w);
}
