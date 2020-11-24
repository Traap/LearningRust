// The into trait is simply the reciprocal of the From trait.  That is, if you
// have implemented the From trait for your type, Into will call it when 
// necessary.
//
// Using the Infor trait will typically require specification of the type
// to convert into as the compiler is unable to determine this most of 
// the time.  However this is a small trade-off considering we get the
// functionality for free.

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from (item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
