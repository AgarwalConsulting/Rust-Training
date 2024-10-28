fn main() {
    let v = 1..1001;

    println!("{:?}", v);

    let v1: Vec<i32> = v.into_iter().map(|el| { el * 2}).collect();

    println!("{:?}", v1);
}
