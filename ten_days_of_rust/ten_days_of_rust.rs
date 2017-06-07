extern crate manually_implement as mui;

fn main() {
    let email = "someone@example.com";
    let username = "someoneuser666";
    let user = mui::User {
        email: &email,
        username: &username,
        active: true,
        sign_in_count: 1,
    };

    println!("Debug: {:?}", user);
    println!("Display: {}", user);
    
    // Using function 'build_user()'.
    let b_user = mui::build_user(&email, &username);
    println!("Building user: {}",b_user);
    assert_eq!("someone@example.com",b_user.email);

    let x: f64 = 1.00000001;
    let y: f64 = 0.11111111;
    let point = mui::RefPoint {
        x: &x,
        y: &y,
    };
    mui::my_print(&point);
}
