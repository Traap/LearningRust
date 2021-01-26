// One of the more common types to convert to a string into a number.  The idiomatic approach to
// this is to use the parse function and either to arrange for type inference or to specify the
// type to parse using the 'turbofish' syntax.  Both alternatives are shown in the following
// example.
//
// This will convert the string into the type specified so long as the FromStr trait is implemented
// for that type.  This is implemented for numerous types within the standard library.  To obtain
// this functionality on a user defined type simply implement the FromStr trait for that type.
// let parsed: i32 = "5".parse().unwrap();
// let turbo_parsed = "10".parse::<i32>().unwrap();
//
// let sum = parsed + turbo_parsed;
// println!("Sum: {:?}", sum);

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}
