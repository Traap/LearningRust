// To convert any type to a String is a simple as implementing the ToString
// trait for the type.  Rather than doing so direcctly, you should implement
// the fmt:Display trait which automagically provides ToString and also allows
// printing the type as discussed in the section on print!.

use std::fmt;
struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle with radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}
