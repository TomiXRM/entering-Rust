// use std::io;
fn main() {
    for i in 0..10 {
        let mut v = 1;
        for j in 2..=i {
            v = v * j;
        }
        println!("{}の階乗は{}", i, v);
    }
}
