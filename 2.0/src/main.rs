fn main(){
    let x = 7.0;
    println!("{}",type_of(x));
}

fn type_of<T>(_:T) -> &'static str{
    std::any::type_name::<T>()
}