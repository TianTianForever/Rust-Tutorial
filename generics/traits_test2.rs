struct Apple;
struct Fruit;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl <T, U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {

    } 
}

fn main() {
    
    let apple = Apple;
    let fruit = Fruit;

    fruit.double_drop(apple);
}
