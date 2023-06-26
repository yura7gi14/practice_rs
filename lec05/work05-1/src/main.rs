use std::io;
fn main() {
    let age: i32;
    println!("input your age: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    age = input.trim().parse().expect("Please type a number!");

    if age >= 18{
        println!("選挙権あり");
    }else{
        println!("選挙権なし");
    }


}
