/*
struct Map<I, F> {
    iter: I,
    f: F,
}

impl<I: fmt::Dbug, F> fmt::Dbug for Map<I, F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Map")
            .field("iter", &self.iter)
            .finish()
    }
}

trait Iterator {
    type Item;
    fn map<B, F>(self, f: F) -> Map<Self, F> where
        Self: Sized, F: FnMut(Self::Item) -> B {
        Map{iter: self, f: f}
    }
}
*/

pub fn my_map(v: Vec<i32>) -> Vec<i32> {

    v.into_iter().map(|x| x + 1).rev().collect()
}
