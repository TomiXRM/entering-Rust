use std::io;
fn main() {
    println!("整数を入力して下さい");
    //標準入力から1行を読み取り,整数にする
    let mut s = String::new();
    io::stdin().read_line(&mut s).ok();
    let x: i32 = s.trim().parse().unwrap();
    if x > 0 {
        println!("{}は正の数です", x);
    } else if x < 0 {
        println!("{}は負の数です", x);
    } else {
        println!("{}はzeroです", x);
    }
}
