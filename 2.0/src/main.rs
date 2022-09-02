//コンソール入力
use std::io;

fn main() {
    println!("整数を入力してください");
    //標準入力から1行を読み取り、整数にする
    let mut s = String::new();

    let rslt = io::stdin().read_line(&mut s); //読み取り

    match rslt {
        //エラーがあればチェック
        Ok(v) => println!("読みとり成功:{:?}", v),
        Err(e) => println!("読み取り失敗:{:?}", e),
    } //ちょっと関数型っぽさ

    let n: i32 = s.trim().parse().ok().unwrap(); //ここで文字がきた時に対処考えてみたい
    println!("{}", n);
}
