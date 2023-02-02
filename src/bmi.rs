fn calculate_bmi(weight: f32, height: f32) -> f32 {
    let height = height / 100.0;
    weight / (height * height)
}

fn check_bmi(bmi: f32) {
    let message = match bmi {
        bmi if bmi < 16.0 => "You are underweight. Please speak to a doctor or a dietician.",
        bmi if bmi >= 16.0 && bmi <= 25.0 => "Your weight is normal. Keep up the good work!",
        _ => "You're overweight",
    };

    println!("{}", message);
}

pub fn calculate(weight: f32, height: f32) {
    let bmi = calculate_bmi(weight, height);
    println!("Your BMI is: {:.2}", bmi);
    check_bmi(bmi);
}