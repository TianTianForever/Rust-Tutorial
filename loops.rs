fn main(){
    let mut a = 0 ;
    loop{
        a += 1;
        print!("{} ",a);
        if a == 10{ break;}
    }

    let mut x = 0;
    let mut y = false;
    while !y {
        x += 1;
        println!("{} ",x);
        if x ==10 { break; }  // if x == 100 {y = true;}
    }

    for x in 1..20 {
        if x % 2 == 0{ continue; }
        println!("{} ", x);
    }

    // enumerate fuction
    for (i,j) in (1..10).enumerate(){
        println!("i = {} and j = {}",i, j);
    }
}
