extern crate associated_type;
use associated_type::*;
use std::ops::Add;

#[derive(Debug)]
struct Timeline<T> {
    today: T,
    tomorrow: T,
    future: T,
}

impl<T: Add<Output=T>> Add for Timeline<T> {
    type Output = Timeline<T>;
    fn add(self, other: Timeline<T>) -> Timeline<T> {
        Timeline {
            today: self.today + other.today,
            tomorrow: self.tomorrow + other.tomorrow,
            future: self.future + other.future, 
        }
    }
}

fn main() {
    let yesterday = Yesterday;
    let today = Today;
    let tomorrow = Tomorrow;
    let future = Future;
    let my_future = MyFuture;

    my_future.from_yesterday_to_today(&yesterday, &today);
    my_future.create_timeline(&tomorrow, &future);    
}
