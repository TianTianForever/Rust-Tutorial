#[derive(Debug)] enum MyFood{ CordonBleu, Steak, Sushi }
#[derive(Debug)] enum Day {Monday, Tuesday, Wednesday }
#[derive(Debug)] enum Food {Apple, Potato}
#[derive(Debug)] struct Cook(Food);

// I don't have the ingredients to make Sushi.
fn have_ingredients(food: MyFood) -> Option<MyFood> {
    match food {
        MyFood::Sushi => None,
        _             => Some(food),
    }
}

// I know how to make everything except Cord Bleu.
fn can_cook(food: MyFood) -> Option<MyFood> {
    match food {
        MyFood::CordonBleu => None,
        _                  => Some(food),
    }
}

// To make meal, I require both the ingredients and the ability to make that meal,
// which is only possible when both are ture; thus successes chain.
// Conveniently, this can be rewritten more compactly with 'and_then()'.
fn cookable(food: MyFood) -> Option<MyFood> {
    match have_ingredients(food) {
        None       => None,
        Some(food) => match can_cook(food) {
            None       => None,
            Some(food) => Some(food),
        }
    }
}
// Using 'and_then()'.
fn cookable1(food: MyFood) -> Option<MyFood> {
    have_ingredients(food).and_then(can_cook)
}
fn eat(food: MyFood, day: Day) {
    match cookable1(food) {
        Some(food) => println!("On {:?}, I get to eat {:?}.", day, food),
        None       => println!("I don't get to eat on {:?}", day),
    }
}
fn is_work(food: Option<Food>) -> Option<Cook> {
    match food {
        Some(f) => Some(Cook(f)),
        None => None,
    }
}
fn is_ok(food: Option<Food>) -> Option<Cook> {
    food.map(|f| Cook(f))
}
fn main() {
    let food = Some(Food::Apple);
    println!("{:?}", is_work(food));
    let food1 = Some(Food::Potato);
    println!("{:?}", is_ok(food1));
    let (cordon_bleu, steak, sushi) = (MyFood::CordonBleu, MyFood::Steak, MyFood::Sushi);
    eat(steak, Day::Tuesday);
    eat(cordon_bleu, Day::Monday);
}
