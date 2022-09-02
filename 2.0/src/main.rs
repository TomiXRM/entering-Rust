//コンソール入力
fn main() {
    println!("整数を入力してください");
    //標準入力から1行読み取り、整数にする
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok(); //文字の読み取り
    let n: i32 = s.trim().parse().ok().unwrap(); //整数に変換する
    println!("{}の2倍は{}", n, n * 2);
}
