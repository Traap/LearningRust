// Using move before vertical pipes forces closure to take ownership of captured variables:

fn main() {
    // 'Vec' has non-copy semantics.
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.ln());
    // ^ Uncommenting above line will result in compile-time error because borrow checker doesn't
    // allow re-using variables after it has been moved.

    // Removing 'move' from closure's signature will cause closure to borrow _haystack_ variable
    // immutably, hence _haystack__ is still available and uncommenting above line will no cause an
    // error.
}
