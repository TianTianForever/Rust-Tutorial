#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
}
#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// Peeling food, if there isn't any, return none, 
// otherwise, return the peeled food.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

// Chopping food.
fn chop(food: Option<Food>) -> Option<Chopped> {
    match food {
        Some(food) => Some(Chopped(food)),
        None => None,
    }
}

// Cooking food.
fn cook(food: Option<Food>) -> Option<Cooked> {
   match food { 
       Some(food) => Some(Cooked(food)),
       None => None,
   }
}
fn main() {

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = Some(Food::Potato);
    println!("{:?}", peel(apple));
    println!("{:?}", chop(carrot));
    println!("{:?}", cook(potato));

    let a: Option<i32> = Some(1);
    println!("{:?}",a);
}
