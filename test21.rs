// An attribute to hide warrings for unsed code.
#![allow(dead_code)]  
enum Status {
    Warm,
    Cold,
}
enum Work {
    North,
    South,
    West,
    East,
}

fn main() {
    // Explicitly 'use' each name so they are available without manual scoping.
    use Status::{Warm, Cold};
    // Automatically 'use' each name inside 'Work'.
    use Work::*;
    
    // Equivalent to 'Status::Warm'.
    let status = Cold;
    // Equivalent to 'Work::North'.
    let work = North;
   
    match status {
        // Note the lack of scoping because of the explicit 'use' above.
        Warm => println!("It's often warm in the south."),
        Cold => println!("It's often cold in the north."),
    }

    match work {
        // Note again lack of scoping.
        North => println!("north"),
        South => println!("south"),
        West => println!("west"),
        East => println!("east"),
    }
} 
