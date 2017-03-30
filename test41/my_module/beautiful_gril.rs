// beautiful_gril.rs
pub fn function() {
    println!("called 'my_module::beautiful_gril::function()'");
}
fn private_function() {
    println!("called 'my_module::beautiful_gril::private_function()'");
}

pub fn indirect_access() {
    print!("Access 'my_module::beautiful_gril::indirect_access()'");
    println!("called 'my_module::beautiful::indirect_access()'");
}
