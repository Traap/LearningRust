// One of the uses of a loop is to retry an operation until it succeeds.  If the operation returns
// a value though, you might make to pass it to the rest of the code: put it after the break, and
// it will be returned by the loop expression.

fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("counter: {}", counter);
    println!(" result: {}", result);
    assert_eq!(result, 20);
}
