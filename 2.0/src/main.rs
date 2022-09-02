fn main(){
    //所有権
    let x = 12;
    {
        //{}の中で宣言したものは{}の中でしか使えない
        let x = 1232; 
        println!("x={}",x);
    }

    let y = x * 2;
    println!("x={}",x);
    println!("y={}",y);
}