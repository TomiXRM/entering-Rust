//タプルする
fn main() {
    let t1: (i32, i32) = (12, 24);
    let t2: (i32, f64, i32) = (50, 0.8, 12);
    let t3: (f64, &str, i32, char) = (23.0, "hello", 88, 'あ');
    println!("{} {}", t1.0, t1.1);
    println!("{} {} {}", t2.0, t2.1, t2.2);
    println!("{} {} {} {}", t3.0, t3.1, t3.2, t3.3);

    println!("{:?}", t3); //書式設定する→(23.0, "hello", 88, 'あ')って出る

    let mut t = (3, 2);
    t.0 = 100; //mutableなタプルtを書き換え
    println!("{:?}", t);

    let t4 = (3, 's', "うにょん");
    let (x, y, z) = t4; //タプルから新しいタプルをつくる
    println!("x:{} y:{} z:{}", x, y, z);
}
