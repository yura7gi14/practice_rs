use std::io;
fn main() {
    //整数aと浮動小数点数bを宣言
    let a: i32;
    let b: f64;

    //標準入力から整数aと浮動小数点数bを読み込む
    println!("Input a: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    a = input.trim().parse().expect("Please type a number!");


    println!("Input b: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    b = input.trim().parse().expect("Please type a number!");

    //整数aと浮動小数点数bを出力
    println!("a = {}", a);
    println!("b = {}", b);


}
