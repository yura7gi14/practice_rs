use std::io;
fn main() {
    let a: i32;
    let b: i32;
    let menseki: f64;

    println!("input a and b: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    a = input.trim().parse().expect("Please type a number!");

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    b = input.trim().parse().expect("Please type a number!");

    menseki = (a * b / 2).into();

    println!("menseki = {}", menseki);
}
