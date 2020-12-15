// The use declaration can be used to bind a full path to a new name,for easier access.  It is
// often used like this:

// use crate::deeply::neested::{
//     my_first_function,
//     my_second_function,
//     AndATraitType
// };

// fn main() {
//     my_first_function();
// }

// You can use the as keyword tobind imports to a different name:
//
// Bind the 'deeply::nested::function' path to 'start_timer'.
use deeply::nested::start_timer as starter;
use deeply::nested::end_timer as ender;

fn start_timer() {
    println!("'start_timer()' called");
}

mod deeply {
    pub mod nested {
        pub fn start_timer() { println!("'deeply::nested:start_timer()' called."); }
        pub fn end_timer()   { println!("'deeply::nested:end_timer()' called"); }
    }
}

fn main() {
    // Easier access to 'deeply::nested::start_timer'
    starter();

    println!("Entering block");
    {
        // This is equivalant to 'use deeply::nested::function as function'
        // This 'function()' will shadow that outer one.
        use crate::deeply::nested::start_timer;

        // 'use' bindings have a local scope.  In this case, the shadowing of 
        // 'function()' is only in this bloc.
        start_timer();
        println!("Leaving block");
    }

    start_timer();
    ender();
}
