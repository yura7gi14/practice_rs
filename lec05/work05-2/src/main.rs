use std::io;
fn main() {
    let x:i32;
    println!("xを入力せよ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    x = input.trim().parse().expect("Please type a number!");

    if x >= 10{
        println!("xは10以上の整数");
    }else if x >= 0{
        println!("xは0以上10未満の整数");
    }else{
        println!("xは負の整数");
    }
}
