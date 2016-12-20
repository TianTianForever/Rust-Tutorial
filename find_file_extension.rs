fn find(sea: &str, needle: char) -> Option<usize> {
    for (offset, c) in sea.char_indices() {
       if c == needle {
           return Some(offset);
       }
    }
    None
}
fn main() {
    let file_name = "hello.rs";
    match find(file_name, '.') {
        None => println!("No file extension find"),
        Some(i) => println!("{:?}",&file_name[i+1..]),
    }
}
