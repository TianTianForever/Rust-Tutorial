mod mutability;
use mutability::*;
use std::cell::Cell;

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangColor(i32, i32, i32),
}

struct QuitMessage;  // Unit struct
struct MoveMessage { x: i32, y: i32}
struct WriteMessage(String); // Tuple struct
struct ChangeColor(i32, i32, i32);

impl Message {
    fn call(&self) {  }
}

fn main() {
    let message = Message::Write(
        String::from("Life was a box of chocolates. you never know what you're gonna get.")
    );
    message.call();
    // Using 'ref' keyword create a reference.
    let reference = 12;
    let ref use_ref = reference;

    let x: Option<i32> = Some(5);
    let z: Option<f64> = None;
    // Match pattern
    let y = match x {
        Some(x) => println!("Got value {}", x),
        None => println!("Empty value"),
    };
    let point = Point {x: 5, y: Cell::new(5)};
    println!("{:?}", point);
    point.y.set(24);
    println!("{:?}", point);

    let mut origin = Origin {x: 0, y: 0};
    println!("{:?}", origin);
    let point_ref = PointRef {
        x: &mut origin.x,
        y: &mut origin.y,
    };
    *point_ref.x = 1;
    *point_ref.y = 1;
    println!("{:?}", point_ref); 
    
    let value = &100;
    // Using asterisk '*' to access the contents of a reference.
    let get_value = match *value {
        1        => println!("One"),
        2 | 99   => println!("2 or 99"),
        100      => println!("Got the value"), 
        _        => println!("Something else"),
    };
    
    let node1 = Node;
    let node2 = Node;
    let graph = MyGraph;
    graph.has_edge(&node1, &node2);
    graph.edge(&node1);    
}
