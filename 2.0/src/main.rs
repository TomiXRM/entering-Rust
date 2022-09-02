//キャスト変換?
fn main(){
    // let x : i32 = 456;
    // let ref r = &x;
    // println!("{} {} {:?}",x,r,*r);

    let x : i32 = 456;
    let ref r = x;
    println!("{} is typeof {}",x,type_of(x));
    println!("{} is typeof {}",r,type_of(r));
    println!("{} is typeof {}",*r,type_of(*r));
}

fn type_of<T>(_:T) -> & 'static str{
    std::any::type_name::<T>()
}