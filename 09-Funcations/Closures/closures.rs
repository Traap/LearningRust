// Closures in Rust, also called lambda expressions or lambdas, are functions that can capture the
// enclosing environment.  For example, a closure that captures the x variable:
//
// The syntax and capabilities of closures make them very convenient for on the fly usage.  Calling
// a closure is exactly like calling a function.  However, both input and return types can be
// inferred and input variable names must be specified.
//
// Other characteristics of closures include:
// * using || instead of () around input variables.
// * optional body delimitation ( {} ) for a single expression (mandatory otherwise).
// * the ability to capture the outer environment variables.

fn main() {
    // Increment via closures and functions
    fn function (i: i32) -> i32 { i + 1 }

    // Closures are anonymous, here we are binding them to references.  Annotation is identical to
    // function annotation but is optional as are the '{}' wrapping the body.  These nameless
    // functions are assigned to appropriately name variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1;

    // Call the function and closures.
    // A closure taking no arguments which returns an 'i32'.  The return type is inferred.
    let i = 1;
    let one = || 1;

    println!(" function: {}\nannotated: {}\n inferred: {}\n      one: {}", 
        function(i), 
        closure_annotated(i),
        closure_inferred(i),
        one()
        );
}
