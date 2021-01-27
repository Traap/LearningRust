// Documentation
//
// Use 'cargo doc' to build documentation in 'target/doc'.
//
// User 'cargo test' to run all tesets (including documentation tests), and 'cargo test --doc' to
// only run documentation tests.
//
// The commands will appropriately invoke 'rustdoc (and rustc)' as required.
//
// Doc comments
//
// Doc comments are very useful for big projects that require documentation. When nrunning
// 'rustdoc', these are the comments that get compiled into documentation.  They are denoted by
// ///, and support 'Markdown'.
//
#![crate_name = "doc"]

/// A human being is represented here.
pub struct Person {
    /// A person must have a name, no mapper how much Juliet may hate it.
    name: String, 
}

impl Person {
    /// Returns a person with the name given them.
    ///
    /// # Arguments
    ///
    /// * `name` - A strign slice that holds the name of the person
    ///
    /// # Examples
    ///
    /// ```
    /// // You can have rust code between fences inside the comments.
    /// // If you pass --test to `rustdoc`, it will even test it for your!
    /// ```
    pub fn new(name: &str) -> Person {
        Person {
            name: name.to_string(),
        }
    }

    /// Gives a friendly hello!
    ///
    /// Says "Hello, [name]" to the `Person` it is callen on.
    pub fn hello(& self) {
        println!("Hello, {}!", self.name);
    }
}

fn main() {
    let john = Person::new("John");

    john.hello();
}
