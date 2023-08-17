fn main() {
    let x = 5;
    println!("Initial x = {x}");

    let x = x + 1;
    println!("Shadow x = {x}");

    {
        let x = x * 2;
        println!("Shadow inner scope x = {x}");
    }

    println!("Back to Shadow x = {x}");
}
