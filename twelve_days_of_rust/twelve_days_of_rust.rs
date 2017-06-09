extern crate hash_maps;
use hash_maps::*;
use std::collections::HashMap;

// HashMap<K, V> 
fn main() {
    let mut point = HashMap::new();
    // Using 'insert' method add elements.
    point.insert(String::from("x"), 1.0);
    point.insert(String::from("y"), 1.0);
    println!("point: {:?}", point);
    
    // pub enum Option<T> {
    //     None,
    //     Some<T>,
    // }
    let point_key = String::from("x");
    
    // It will return Some(1), if there's no value for that key in the hash map 'point',
    // 'get' method will return 'None'.
    let point_value = point.get(&point_key);
    println!("{:?}", &point_value);
    
    //point.entry(String::from("x")).or_insert(2.1);
    for (key, value) in &point {
        println!("{} : {}",key, value);
    }

    // Another way of constructing a hash map is by using the 'collect' method
    // on a vector of tuples.
    let key = vec![String::from("x"), String::from("y")];
    let value = vec![1.0, 1.0];
    let origin: HashMap<_, _> = key.iter().zip(value.iter()).collect();
    println!("origin: {:?}",origin);

    let text = "hash map hash map";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
    
    // Using 'hash_maps' library module.
    let mut my_point = HashMap::new();
    my_point.insert(Point::new(1, 2), 3);
    my_point.insert(Point::new(3, 4), 7);
    my_point.insert(Point::new(5, 6), 11);
    for (key, value) in &my_point {
        println!("{:?}, {:?}", key, value);
    }
}
