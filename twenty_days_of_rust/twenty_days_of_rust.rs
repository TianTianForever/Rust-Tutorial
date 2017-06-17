/*
pub enum Option<T> {
    Some(T),
    None,
}
*/

// All gifts are handled explicitly using 'match'.
fn gifts(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("Yuck, I'm throwing that snake in a fire"),
        Some(like) => println!("{}? How nice.",like),
        None => println!("No gift? Oh well"),
    }
}

// All gifts are handled implicitly using 'unwrap'.
fn implicity_handle_gifts(gift: Option<&str>) {
    // 'unwrap' returns a 'panic' when it receives a 'None'.
    let im = gift.unwrap();
    if im == "snake" {
        panic!("AAAAAAAAAaaaaaaaaa!!!!");
    } else {
        println!("I love {}s!!!!", im);
    }
}

// Error handling!
fn give_gift<'a>(gift: &'a str) {
    // children hate snakes, so we need to stop if they disapproves!
    if gift == "snake" {
        panic!("AAAAAaaaaaaa");  // explicitly call panic.
    } else {
        println!(" children love {}s!!!",gift);
    }
}

fn main() {
    give_gift("teddy bear");
    //give_gift("snake");    // paniced here.

    // Call function 'gift'.
    let you_gift = Some("teddy bear");
    let snake = Some("snake");
    let empty = None;
    gifts(you_gift);
    gifts(snake);
    gifts(empty);

    // Call function 'implicity_handle_gifts'.
    //let nothing = None;
    //let receives_gift = Some("snake");
    let like_gift = Some("teddy bear");
    //implicity_handle_gifts(nothing);      // paniced here.
    //implicity_handle_gifts(receives_gift);
    implicity_handle_gifts(like_gift);
    
}
