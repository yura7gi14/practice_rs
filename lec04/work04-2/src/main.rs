fn main() {
    let mut a = int::new();
    let mut b = double::new();

    println!("input a: ");
    std::io::stdin().read_line(&mut a).ok();
    println!("input b: ");
    std::io::stdin().read_line(&mut b).ok();

    println!("a = {}, b = {}", a, b);
}
