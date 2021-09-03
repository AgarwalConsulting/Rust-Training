fn main() {
    // let v = vec!(1, 2, 3, 5, 7, 11);
    let mut v = vec!(1, 2, 3, 5, 7, 11);

    // let v1: i32 = v.iter().sum();

    // println!("Sum: {}", v1);

    // let v1: Vec<i32> = v.iter().map(|el| { el * 2 }).collect();
    // println!("Doubled: {:?}", v1);

    v.iter_mut().for_each(|el| { *el = *el * 2 });

    println!("Original: {:?}", v);
}
