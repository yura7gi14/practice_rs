use std::io;
fn main() {
    let tani: i32;

    println!("１年間で習得した単位数を記入してください");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let input_numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    if input_numbers.len() != 1 {
        panic!("Please input one number!");
    }

    tani = input_numbers[0];

    if tani >= 40{
        println!("留年する確率は低い");
    }else if tani >= 30{
        println!("留年する確率は高い");
    }else{
        println!("留年する確率はかなり高い");
    }
}
