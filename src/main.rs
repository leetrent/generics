use num_traits::{Float};

fn solve<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}


fn main() {
    let a_f32: f32 = 3.0;
    let b_f32: f32 = 4.0;  

    println!();
    println!("hypotenuse: {}", solve::<f32>(a_f32, b_f32));
    println!();
}
