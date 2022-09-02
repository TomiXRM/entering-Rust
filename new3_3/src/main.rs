use std::io;
fn main() {
    println!("enter a number");
    let mut s: String = String::new();
    let rslt = io::stdin().read_line(&mut s);
    match rslt {
        Ok(_v) => {}
        Err(e) => {
            println!("{}", e,);
        }
    }

    let num: i16 = s.trim().parse().unwrap();//trim()がないと改行で死ぬっぽ
    println!("{} * 2 = {}", num, num * 2);
}
