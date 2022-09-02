use std::io;
fn main() {
    println!("数値を入力してください");
    let mut s: String = String::new();
    io::stdin().read_line(&mut s).ok();
    let n: i32 = s.trim().parse().unwrap();
    let x: i32 = n % 2;
    match x {
        0 => println!("{}は偶数です", n),
        1 => println!("{}は奇数です", n),
        _ => println!("{}は奇数でも偶数でもありません", n),
    }
}
