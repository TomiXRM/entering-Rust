enum DogKind {
    AKITAINU,
    SHIBAINU,
    RETIEVER,
}

fn main() {
    let mut dog = DogKind::AKITAINU;
    match dog {
        DogKind::AKITAINU => println!("これは秋田犬"),
        DogKind::SHIBAINU => println!("これはしば犬"),
        DogKind::RETIEVER => println!("これはレトリーバ"),
    }

    dog = DogKind::SHIBAINU;
    match dog {
        DogKind::AKITAINU => println!("これは秋田犬"),
        DogKind::SHIBAINU => println!("これはしば犬"),
        DogKind::RETIEVER => println!("これはレトリーバ"),
    }

    dog = DogKind::RETIEVER;
    match dog {
        DogKind::AKITAINU => println!("これは秋田犬"),
        DogKind::SHIBAINU => println!("これはしば犬"),
        DogKind::RETIEVER => println!("これはレトリーバ"),
    }
}
