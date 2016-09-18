fn main(){
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {0} and value of y is {1}",x,y);
    }
    //printlin!("The value of x is {0} and value of y is {1}",x,y) //This won't work

}
