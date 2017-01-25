// An attribute to hide warnings for unused code.
#![allow(dead_code)]
enum Person {
    // An 'enum' may either be 'unit-like'.
    Engineer,
    Scientist,
    // Tuple structs.
    Height(i32),
    Weight(i32),
    // Structures.
    Info {name: String, height: i32}
}

// A fuction which takes a 'Person' enmu as an argument and returns nothing.
fn inspect(p: Person) {
    // Usage of an 'enum' must cover all cases (irrefutable).
    match p {
        Person::Engineer => println!("Is an engineer"),
        Person::Scientist => println!("Is a scientist"),

        // Destructue 'i' from inside the 'enum'.
        Person::Height(i) => println!("Has a height of {}.",i),
        Person::Weight(i) => println!("Has a weight of {}.",i),
        
        // Destructure 'Info' into 'name' and 'height'.
        Person::Info { name, height } => {
            println!("{} is {} tall.",name, height);
        },
                   
    }

}
fn main() {
    let person1 = Person::Height(180);
    let person2 = Person::Weight(70);
    let Bob = Person::Info { name: "Bob".to_owned(), height: 175 };
    let e = Person::Engineer;
    let s = Person::Scientist;
    inspect(person1);
    inspect(person2);
    inspect(Bob);
    inspect(e);
    inspect(s);
}   
