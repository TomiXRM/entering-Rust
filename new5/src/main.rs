union MyData {
    va: u16,
    vb: u32,
}
fn main() {
    let v = MyData { vb: 0 };
    unsafe {
        //unionはC言語で使われるデータ構造で、メモリの侵害などのリスクから危険なコードを書きやすい
        //危険なのでunsafeで領域を絞って書く
        println!("v.va={}", v.va);
        println!("v.vb={}", v.vb);
    }
}
