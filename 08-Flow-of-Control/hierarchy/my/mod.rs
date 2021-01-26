// Similarly 'mod inaccessible' and 'mod nested' will locate the 'nested.rs' and 'ineccessible.rs'
// files and insert them here under their respective modules.

mod inaccessible;
pub mod nested;

pub fn function() {
    println!("my::nested::function() called");
}

fn private_function() {
    println!("my::private_function() called");
}

pub fn indirect_access() {
    println!("my::indirect_access() called)");
    private_function();
}
