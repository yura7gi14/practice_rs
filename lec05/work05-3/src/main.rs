use std::io;
fn main() {
    let x: i32;

    println!("input x: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    x = input.trim().parse().expect("Please type a number!");

    //xの絶対値を求める
    let abs_x = if x >= 0 { x } else { -x };

    println!("|x| = {}", abs_x);
}
