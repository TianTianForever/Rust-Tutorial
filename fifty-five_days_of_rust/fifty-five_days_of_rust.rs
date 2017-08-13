extern crate wrapper;
use wrapper::MyFuture;
fn main() {
    println!("It work.");
    let tomorrow = wrapper::Tomorrow;
    let future = wrapper::Future;
    let my = wrapper::Myself;
 
    MyFuture::my_future(&my, &tomorrow, &future);
    wrapper::hard(&my, &tomorrow, &future);

    let today = wrapper::Today{ time: 24 };
    let yesterday = wrapper::Yesterday{time: 24};
    let now = today + yesterday;
    println!("now: {:?}", now);

    let my_wrapper = wrapper::Wrapper(vec![String::from("tiantian"), String::from("forever")]);
    println!("my_wrapper: {}", my_wrapper); 
}
