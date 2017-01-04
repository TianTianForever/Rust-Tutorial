fn main() {
    let x = 5;
    let y = if x == 5 { 10} else { 15 };
    assert_eq!(y, 10);

    for i in x..20 {
        let mut z: Vec<i32> = Vec::new();
        z.push(i);
    }
    
    'outer: loop {
       println!("entered the outer loop");
       
       'inter: loop {
          println!("entered the inter loop");
          break 'outer;
       }   
       println!("this point will never be reached");
    }
    println!("exited the outer loop");
    
}
