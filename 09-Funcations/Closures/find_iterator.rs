// Iterator::find is a function which iterates over an iterator and searches for the first value
// which satisfies some condition If none of the values satisfy the condition, it returns None. Its
// signature:
pub trait Iterator {
    type Item;

    // 'find' takes '&mut self' meaning the caller may be borrowed and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // 'FnMut' meaning any captured variable may at most be modified, not consumed.
        // '&Self::Item' states it takes arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool {}
}

fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // 'iter()' for vecs yield '&i32'.
    let mut iter = vec1.iter();

    // 'into_iter()' for vec yields 'i32'
    let mut into_iter = vec2.into_iter();

    // 'iter()' for vecs yields '&i32', and we want to reference an one its items, so we have to
    // destructure '&&i32' to 'i32'
    println!("Find 2 in vec1: {:?}", iter.      find(|&&x| x == 2));

    // 'into_iter()' for vecs yiedl 'i32', and we want to reference one of its items, so we have to
    // destructure '&i32' to 'i32'
    println!("Find 2 in vec2: {:?}", into_iter. find(| &x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
}
