fn solve(a: f64, b: f64) -> f64 {
    (a.powi(2) + b.powi(2)).sqrt()
}


fn main() {
    let a: f32 = 3.0;
    let b: f64 = 4.0;

    let a_f64 = a as f64;

    println!();
    println!("hypotenuse: {}", solve(a_f64, b));
    println!();
}
