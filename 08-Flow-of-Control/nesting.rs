// It's possible to break or continue outer loops when dealing with nested loops. In these cases,
// the loop must be annotated with some 'label, and the label must be passed to the break
// continue.

#![allow(unreachable_code)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            // This would break only the inner loop.
            // break;
            // This breaks the outer loop.
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}
