fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let t0 = x.0;
    let t1 = x.1;
    let t2 = x.2;

    println!("t0 = {t0}");
    println!("t1 = {t1}");
    println!("t2 = {t2}");
}
