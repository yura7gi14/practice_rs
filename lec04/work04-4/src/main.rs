use std::io;
fn main() {
    let height: f64;
    let weight: f64;
    let BMI: f64;

    println!("input height(m) and weight(kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    height = input.trim().parse().expect("Please type a number!");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    weight = input.trim().parse().expect("Please type a number!");

    BMI = weight / (height * height);

    println!("BMI = {}", BMI);
}
