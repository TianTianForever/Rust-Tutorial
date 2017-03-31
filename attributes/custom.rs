// Some conditionals like target_os are implicitly provided by rustc,
// but custom conditionals must be passed to rustc using the --cfg flag.
#[cfg(some_condition)]
fn conditional() {
    println!("condition met");
}
fn main() {
    conditional();
}

// $rustc custom.rs && ./custom
// No such file or directory( os error 2)

// $rustc custom.rs && ./custom
// condition met!
