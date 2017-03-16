/// Iterator::any is a function which when passed an iterator, will return 'true' if any element satisfies
/// the predicate. Otherwise false. its signature:
///pub trait Iterator {
///    type Item;
///    fn any<F>(&mut self, f: F) -> bool where
///        F: FnMut(Self::Item) -> bool {
/// 
///    }
///}
fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    println!("2 in vec1: {}",vec1.iter().any(|&x| x == 3));
    println!("7 in vec2: {}",vec2.into_iter().any(|x| x == 7));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array: {}", array1.iter().any(|&x| x == 3));
    println!("7 in array: {}", array2.into_iter().any(|&x| x == 7));
}
