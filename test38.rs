/// Iterator::find is a function which when passed an iterator,
/// will return the first element which satisfies as an Option. 
///Its signature:
///pub trait Iterator {
///    type Item;
///    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
///        P: FnMut(&Self::item) -> bool {
///
///    }
///}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 'iter()' for vecs yields '&i32'.
    let mut iter = vec1.iter();

    // 'into_iter()' for vecs yields 'i32'.
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));
    //println!("Find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
    //println!("Find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));
}

