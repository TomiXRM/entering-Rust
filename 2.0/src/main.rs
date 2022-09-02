//コンソール入力
fn main() {
    println!("名前を入力してください！");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();

    let name = line.trim().to_string();
    println!("こんにちは{}さん", name);
}
