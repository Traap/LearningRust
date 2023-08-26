fn main() {
    println!("Hello, world!");
    another_function();
    yet_another_function(5);
    print_labeled_measurements(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

fn yet_another_function(x: i32) {
    println!("Yet another function: x = {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char) {
    println!("Print labeled measurements is: {value}{unit_label}");
}
