// Let's create a library, and then see how to link it to another crate.

pub fn public_function() {
    println!("rary's public_fuction() called");
}

fn private_function() {
    println!("rary's private_function() called");
}

pub fn indirect_access() {
    println!("rary's rary's_function() called");
    private_function();
}
