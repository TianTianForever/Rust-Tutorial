mod generics;
use generics::*;
mod point;
use point::*;

fn main() {
    let s = Some(2);
    let a = match s {
        Some(ref a) => println!("{:?}", a),
        None => println!("Empty value"),
    };
   
    assert_eq!(Some(123456789), get_value());
    print_value(get_value());
   
    area(Area(6));
    area(Area(6.0));
    generic::<i32>(Area(2));
    generic::<f64>(Area(2.0));
    let area = Area(1993i32);
    println!("{:?}", area.take_area());
    let rectangle = Rectangle(Graph);
    println!("{:?}", rectangle.output());

    makefile();
    let my_favorite_song = Mp3 {
        audio: vec![1, 3, 5, 7, 9],
        artist: Some(String::from("Jason Donovan")),
        title: Some(String::from("Sealed With A Kiss")),
    };
  
    assert_eq!(vec![1, 3, 5, 7, 9], *my_favorite_song);

    let origin = Origin {x: 0.0 };
    assert_eq!(0.0, *origin);
}
