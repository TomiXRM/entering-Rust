fn main(){
    //RustにはStringとstr型があるらしい
    //strは文字列スライスっていうらしい。
    //strではメモリ上の文字列の先頭と長さを保持しているらしい


    // strを表示
    let msg:&str = "Hello world!!" ;
    println!("msg={}",msg);


    //文字スライスの長さはlen()を使って取得できるらしい
    println!("len={}",msg.len());
    


    let s = &msg[2..5]; // 3文字目(0スタート)から5文字目までを取得
    println!("3..5={}",s);
}