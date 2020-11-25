// The while keyword can be used to run a loop while a condition is true.
//
// Let's write the infamous FizzBuzz using a while loop.

fn main() {
    // A counter variable
    let mut n = 1;

    // Let while 'n' is less than 101.
    while n < 101 {
        if      n % 15 == 0 { println!("fizzbuzz"); } 
        else if n %  3 == 0 { println!("fizz"); } 
        else if n %  5 == 0 { println!("buzz"); } 
        else                { println!("{}", n); }
        n += 1; 
    }
}
