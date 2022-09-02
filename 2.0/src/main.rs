fn main(){
    let x : f32 = 7.0;
    let y : f64 = 0.3;
    println!("x*y={}",x as f64 *y);//型が異なるのでエラーだがas f64ってすればいけるらしい
    //no implementation for f32 * f64


    let c:char = 'c';
    let b = b'c';//b'なんとか'をすると「なんとか」の文字が整数に直される(ASCII)

    println!("{} is {} in integer",c,b);
}