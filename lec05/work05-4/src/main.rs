use std::io;
fn main() {
    let report1: i32;
    let report2: i32;

    println!("レポート1とレポート2の成績を入力");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input_numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    if input_numbers.len() != 2 {
        panic!("Please input two numbers!");
    }

    report1 = input_numbers[0];
    report2 = input_numbers[1];

    if report1 >= 60 && report2 >= 60{
        println!("合格");
    }else if report1 >= 60{
        println!("レポート1のみ合格");
    }else if report2 >= 60{
        println!("レポート2のみ合格");
    }else{
        println!("不合格");
    }


}
