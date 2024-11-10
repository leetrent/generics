use num_traits::{Float, ToPrimitive};

fn solve_with_same_type<T: Float>(a: T, b: T) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve_with_different_type<T: Float, U: Float>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn solve_using_to_primitive<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    same_type();
    different_type();
    to_primitive();
}

fn same_type() {
    let a_f32: f32 = 3.0;
    let b_f32: f32 = 4.0;  

    println!();
    println!("[same_type] => (hypotenuse): {}", solve_with_same_type::<f32>(a_f32, b_f32));
    println!();
}

fn different_type() {
    let a_f32: f32 = 3.0;
    let b_f64: f64 = 4.0;  

    println!();
    println!("[different_type] => (hypotenuse): {}", solve_with_different_type::<f32, f64>(a_f32, b_f64));
    println!();
}

fn to_primitive() {
    let a_i32: i32 = 3;
    let b_f64: f64 = 4.0;  

    println!();
    println!("[to_primitive] => (hypotenuse): {}", solve_using_to_primitive::<i32, f64>(a_i32, b_f64));
    println!();
}

