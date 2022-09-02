//コンソール入力
use std::io;

fn main() {
    println!("整数を入力してください");
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let n: i32 = s.trim().parse().ok().unwrap();

    println!("{}の2倍の値は{}", n, n * 2);
}
