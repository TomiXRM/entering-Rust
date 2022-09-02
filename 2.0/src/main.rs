fn main(){
    let x : f32 = 7.0;
    let y : f64 = 0.3;
    println!("x*y={}",x*y);//型が異なるのでエラー
    //no implementation for f32 * f64
}

fn type_of<T>(_:T) -> &'static str{
    std::any::type_name::<T>()
}