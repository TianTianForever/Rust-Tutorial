// Link to 'library',import items from 'generic_container' library.
extern crate associated_types as ass;

mod beautiful_gril;
use beautiful_gril::my_lifetimes;
use beautiful_gril::beautiful_soup;
use ass::{Graph, MyPoint};
fn main() {
    println!("{}",ass::add(1,2));
    let mygraph = ass::MyGraph;
    let m = Box::new(mygraph) as Box<ass::Graph<N=ass::Node, E=ass::Edge>>;

    let x = 1;
    let y = 2;
    let point = ass::Point(x, y);
    println!(
        "The Point x: {} ,  y: {} is {}",
        &x, &y,
        point.contains(&x, &y)
    );
    println!("The difference is: {}", ass::difference(&point));

    // Using 'beautiful_gril::my_lifetimes' module.

    let a = my_lifetimes::pass_x_and_y(&x, &y);
    println!("Pass x and y is {:?}",a);
    let mut value = my_lifetimes::Value(1);
    println!("Take value is {:?}",value.take_value());
    value.add_value();
    println!("Add value {:?}",value.take_value());
   
    let ages: u32 = 23;
    let height: f64 = 170.0;
    let weight: f64 = 120.0;
    let Tom = my_lifetimes::Person {
         Ages: &ages,
         Height: &height,
         Weight: &weight,
    };
    println!("Tom information is '{:?}'.",Tom);

    // Using 'beautiful_gril::beautiful_soup' module.
    let generic_value = 777777;
    let gen_value = beautiful_soup::GenValue(&generic_value);
    beautiful_soup::gen_print(&gen_value);
} 
