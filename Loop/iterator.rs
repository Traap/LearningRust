// The for in construct can be used to iterate through an Iterator. One of the easiest way to
// create an iterator is to use the range notation a..b.  This yields value from a (inclusive) to
// b (exclusive) in stepsof one.

// Let's write FizzBuzz using for instead of while.

fn main() {
    // 'n' will take the values: 1, 2, ..., 100 in each iteration. 
    for n in 1..100 {
        if      n % 15 == 0 { println!("fizzbuzz"); } 
        else if n %  3 == 0 { println!("fizz"); } 
        else if n %  5 == 0 { println!("buzz"); } 
        else                { println!("{}", n); }
    }
}
