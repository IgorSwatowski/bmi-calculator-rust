mod bmi;
use std::io;

fn main() {

    let mut weight = String::new();
    let mut height = String::new();

    println!("What's your height?");
    io::stdin().read_line(&mut height)
        .expect("Didn't receive a height");

    println!("What's your weight?");
    std::io::stdin().read_line(&mut weight)
        .expect("Didn't receive weight");

    let weight:f32 = weight.trim().parse().expect("Didn't receive a weight");
    let height:f32 = height.trim().parse().expect("Didn't receive a height");

    bmi::calculate(weight, height);
    
}
