// The super and self keywords can be use in the path to remvoe ambiguity when accessing items
// and to prevent unnecessary hardcoding of paths.

fn function() {
    println!("\tfunction() called");
}

mod cool {
    pub fn function() {
        println!("\tcool::function() called");
    }
}

mod my {
    fn function() {
        println!("\tmy::function() called");
    }

    mod cool {
        pub fn function() {
          println!("\tmy::cool::function() called");
        }
    }

    pub fn indirect_call() {
        // Lets access all the functions names function from this scope!
        println!("my::indirect_call() called");

        // The self keyword refers to the current module scope - in this case my.
        // Calling self::function() and calling function() directly both give the 
        // same result, because they refer to the same function.
        println!("call with self::");
        self::function();

        println!("call without modifier");
        function();

        // We can also use self to access another module inside my:
        println!("call with self::cool");
        self::cool::function();

        // The super keyword refers to the parent scope (outside the my module).
        println!("call with super::");
        super::function();

        // This was bind to the cool::function in the *crate* scope.  In this case the 
        // crate scope is the outermost scope.
        {
            use crate::cool::function as root_function;
            println!("call with alias:");
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
