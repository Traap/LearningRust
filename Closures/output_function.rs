// Closures as input parameters are possible, so returning closures as output parameters should
// also be possible.  However, anonymous closures types are, by definition, unknown so we have to
// use impl Trait to return them. 
//
// The valid traits for returning a closure are:
//
//   * Fn
//   * FnMut
//   * FnOnce
//
// Beyond this, the move keyword must be used, which signals that all captures occur by value.
// This is required because any captures by reference would be dropped as soon as the function
// exited, leaving invalid reference in the closure.

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();
    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();
    move || println!("This is a: {}", text)
}

fn main() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();

    fn_plain();
    fn_mut();
    fn_once();
}
