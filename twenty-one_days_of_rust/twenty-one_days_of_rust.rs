
#[derive(Debug)] enum Food { Apple, Carrot, Potato}
#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

// Peeling food. If there isn't any, then just return 'None'.
// Otherwise, renturn the peeled food.
fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(food) => Some(Peeled(food)),
        None => None,
    }
}

fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
    match peeled {
        Some(Peeled(food)) => Some(Chopped(food)),
        None => None,
    }
}

// Cooking food. using 'match' for case handling.
fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
    match chopped {
        Some(Chopped(food)) => Some(Cooked(food)),
        None => None,
    }
}

// Cooking food.  using 'map()' instead of 'match' for case handling.
fn instead_cook(chopped: Option<Chopped>) -> Option<Cooked> {
    chopped.map(|Chopped(food)| Cooked(food))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|food| Peeled(food))
        .map(|Peeled(food)| Chopped(food))
        .map(|Chopped(food)| Cooked(food))    
}

fn is_string(s: Option<&str>) {
    match s { 
        Some(s) => println!("{}", s),
        Some("is string") => println!("Is string"),
        None => println!("Null"),
    }
}
fn eat(food: Option<Cooked>) {
    match food {
        Some(food) => println!("I love {:?}", food),
        None => println!(" Oh no! It wasn't edible."),
    }
}
fn main() {
    is_string(Some("intelligent"));
    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;
    let cooked_apple = instead_cook(chop(peel(apple)));
    let cooked_carrot = instead_cook(chop(peel(carrot)));
    //let cooked_potato = instead_cook(chop(peel(potato)));
    let cooked_potato = process(potato);
    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);    
}
