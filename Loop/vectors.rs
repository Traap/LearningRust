// The for in construct is able to interact with an Iterator in several ways.  As discussed in the
// section on the iterator trait, by default the for loop will apply the into_iter function to the
// collection.  However, this is not the only means of converting collections into iterators.
//
// into_iter, iter and iter_mut all handle the conversion of a collection into an iterator in
// different ways, by providing different views on the data within.


// iter - This borrows each element of the collection through each iteration.  Thus leaving the
// collection untouched and available for reuse after the data within.

fn loop_one() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"), 
            _ => println!("Hello {}", name),
        }
    }
}

// into_iter - This consumes the collection so taht on each iteration the exact data is provided.
// Once the collection has ben consumed it is no longer available for reuse as it has been 'moved'
// within the loop.

fn loop_two() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"), 
            _ => println!("Hello {}", name),
        }
    }
}


// iter_mut - This mutable borrows each element of the collection, allowing for the collection to
// be modified in place.

fn loop_three() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!", 
            _ => "Hello",
        }
    }
}

// Kick the tires.
fn main() {
    loop_one();
    loop_two();
    loop_three();
}

