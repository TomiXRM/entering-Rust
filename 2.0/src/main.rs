//キャスト変換?
fn main(){
    // 文字列から数値に変換する方法
    let s = "123";
    let v : f64 = s.parse().unwrap();
    println!("{}",v);
    println!("{}",v*2.0);
}