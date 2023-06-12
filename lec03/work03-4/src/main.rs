fn main() {
    let a = 3;
    let b = 10;
    let c1;
    let mut c2: f32;
    let mut c3: f64;

    c1 = b / a;
    println!("{} / {} = {}", b, a, c1);

    c2 = (b as f32) / (a as f32);
    println!("{} / {} = {}", b, a, c2);

    c2 = (b as f32) / a as f32;
    println!("{} / {} = {}", b, a, c2);

    c2 = b as f32 / a as f32;
    println!("{} / {} = {}", b, a, c2);

    c3 = (b as f64) / (a as f64);
    println!("{} / {} = {}", b, a, c3);
}
