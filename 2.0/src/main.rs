//static 変数
// Staticな変数はどこからでも参照することができる変数らしい。
// ただし大文字で宣言することtと、型を明示する必要がある。
static mut VAL : i32 = 123;

// staticは便利なように思えますが、重大なバグの原因になる可能性を持っているので
// unsafeで囲むことにしている。
fn main(){
    unsafe{
        println!("VAL={}",VAL);
    }

    twice();

    unsafe{
        println!("VAL={}",VAL);
    }
}

fn twice(){
    unsafe{
        VAL = VAL * 2;
        println!("VAL in twice() = {}",VAL);
    }
}