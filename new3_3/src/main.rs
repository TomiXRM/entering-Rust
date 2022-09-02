use std::io;
fn main() {
    let mut s: String = String::new();
    println!("名前を入力してください");
    let rslt = io::stdin().read_line(&mut s);
    match rslt {
        Ok(_v) => {}
        Err(_e) => {
            println!("Error");
        }
    }
    let r: String = s.parse().unwrap();
    println!("{}さんこんにちは！", r);
}
