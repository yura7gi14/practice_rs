use std::io;

fn main() {
    let year: i32;
    let wareki: i32;

    println!("Input year: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    year = input.trim().parse().expect("Please type a number!");

    wareki = year - 2018;

    println!("令和 = {}", wareki);
}
