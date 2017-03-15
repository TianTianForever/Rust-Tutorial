// Type anonymity
// Observe how using a closure as a function parameter requires generics.

// 'F' must be generic.
fn info<T>(i: T)
    where T: FnOnce() {
    i();
}

fn main() {
    let beautiful_gril = "anny";
    let name = || {
        println!("The beautiful gril name is {}.", beautiful_gril);
    };
    info(name);
}
