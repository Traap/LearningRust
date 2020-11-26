// Functions are declared using the fn keyword. Its arguments are typed annotated, just like
// variables, and, if the function returns a value, the return type must be specified after an
// arrow ->.
//
// This final expression in the function will be used as return value.  Alternatively, the return
// statement can be used to return a value earlier from within the function, even from inside loop
// or if statements.

// Unlike c/c++, there's no restriction on the order of function definitions.
fn main() {
    // We can use this function here, and define it somewhere later.
    fizzbuzz_to(100);
}

// Functions that returns a boolean value.
fn is_divisable_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        // Trap the corner case upfront.
        return false;
    }

    // This is an expression, the 'return' keyword is not necessary.
    // Notice the expression does not end with a simicolon.
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type '()'.
fn fizzbuzz(n: u32) -> () {
    if      is_divisable_by(n, 15) { println!("fizzbuzz"); }
    else if is_divisable_by(n, 3)  { println!("fizz"); }
    else if is_divisable_by(n, 5)  { println!("buzz"); }
    else                           { println!("{}", n); }
}

// When a function returns '()', the return type can be omitted from the signature.
fn fizzbuzz_to(n: u32) {
    for n in 1..n + 1 {
        fizzbuzz(n);
    }
}
