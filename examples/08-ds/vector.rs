fn main() {
    // let mut v = Vec::new();
    let v = vec!(1, 2, 3, 5, 7);

    // let a = 1;
    // let p = 2;
    // let r = 3;
    // let s = 5;

    // v.push(a);
    // v.push(p);
    // v.push(r);
    // v.push(s);

    println!("{:?}", v);

    let ov = v.get(8);

    // println!("{:?}", ov);

    if let Some(val) = ov {
        println!("{}", val);
    };
}
