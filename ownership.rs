fn main(){
//    let v = vec![1,2,3];
//    let v2 = v;
//    println!("v[0] is {:?}",v[0]);
    let a = 1;
    let b = a;
    println!("a is {}, b is {}",a, b);
    let c = 5;
    let y = double(5);
    println!("c is {}, y is {}",c, y);
    let cc = true;
    let yy = change_truth(cc);
    println!("cc is {}, yy is {}",cc, yy);
}
fn double(i: i32) -> i32 {
    i * 5
}

fn change_truth(b: bool) -> bool {
    !b
}
