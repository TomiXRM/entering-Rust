//キャスト変換?
fn main(){
    let n : i32 = 32;
    let f : f32 = n as f32;

    println!("integer to float{}→{}",n , f);

    let v = 123.45;
    let n = v as i32;

    println!("float to integer{}→{}",v , n);

    //floatの丸め込みに使えるメソッドに
    // - floor()
    // - ceil()
    // - round()
    // がある
}