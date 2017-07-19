use std::option::Option;
extern crate objects as objs;
use std::thread;
// Error!!!
/*
fn my_push(v: objs::AveragedCollection) {
    for i in 1..101 {
        let child = thread::spawn(move || {
                v.add(i);
                v.update_average();
                println!("{:?}", v);
        });
        child.join();
    }
}
*/
fn result(o: Option<i32>) -> Option<i32> {
    match o {
        Some(value) => Some(value),
        None => None,
    }
}
fn main() {
    let value = Some(10);
    assert_eq!(Some(10),result(value));
    let screen = objs::Screen {
        components: vec![
           Box::new( objs::SelectBox {
              width: 23,
              height: 23,
           }),
        ]
    };
   screen.run();

   let mut average = objs::AveragedCollection {
       list: vec![],
       average: 0.0,
   };

   println!("{:#?}",average);
   average.add(23);
   average.update_average();
   println!("{:?}", average);
   average.add(24);
   average.update_average();
   println!("{:?}", average);
   //my_push(average);
}
