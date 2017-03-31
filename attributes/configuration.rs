// This function only gets compiled if the target OS is linux.
#[cfg(target_os = "linux")]
fn linux() {
    println!("You are running linux!");
}

// And this function only gets compiled if the target OS is 'not' linux.
#[cfg(not(target_os = "linux"))]
fn else_os() {
    println!("You are 'not' running linux!");
}

fn main() {
    linux();
    
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes, It's definitely linux!");
    } else {
        println!("Yes, It's definitely 'not' linux!");
    }
}
