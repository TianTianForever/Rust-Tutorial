fn main(){
    let v1 = vec![1,2,3];
    let v2 = vec![1,2,3];
//    let(v1,v2,answer) = foo(v1,v2);
    let answer =foo(&v1,&v2);
    let a = vec![2,3,4];
    let b = vec![3,4,5];
    let c = change(&a, &b);
    let d = change2(&v1, &v2);
    let e = change3(&v1, &v2);
}
//fn foo1(v1: Vec<i32>, V2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    //do stuff with v1 and v2


    //hand back ownership, and the result of our function
//    (v1,v2,42)
//}
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    //do stuff with v1 and v2
    //return the answer
    42
}
fn change(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    println!("do stuff with a and b 33");
    33
}
fn change2(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    println!("stuff with v1 and v2 22");
    22
}
fn change3(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    println!("emotional exchange");
    7
}
