use std::io;
fn main() {
    println!("type number");
    loop {
        let mut s: String = String::new();
        let r = io::stdin().read_line(&mut s);
        match r {
            Ok(_) => {}
            Err(_) => {
                println!("type again");
                continue;
            }
        }
        let n: i32 = s.trim().parse().unwrap();
        let b: bool = (n % 2) == 0;
        match b {
            true => println!("奇数です"),
            false => println!("偶数です"),
        }
        loop {}
    }
}
