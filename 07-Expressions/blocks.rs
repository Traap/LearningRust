// last expression of the block ends with a semicolon, the return valu ewill be ().

fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`.
        x_cube + x_squared + x
    };

    // #[warn(unused_must_use)]
    let z = {
         // The semicolon suppresses the expression and `()` is assigned to `z`.
         2 * x
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
