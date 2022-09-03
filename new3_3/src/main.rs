// use std::io;
fn main() {
    'looptop: for i in 0..4 {
        for j in 0..4 {
            if i == 2 && j == 2 {
                break 'looptop;
            }
            println!("{}{}", i, j);
        }
    }
}
