fn main() {
    let mut v = Vec::new();
    // let mut v: Vec<&mut String> = Vec::new();

    let mut a = String::from("Anushka");
    let mut p = String::from("Prashant");
    let mut r = String::from("Ranjith");
    let mut s = String::from("Subham");

    v.push(&mut a);
    v.push(&mut p);
    v.push(&mut r);
    v.push(&mut s);

    println!("{:?}", v);

    match v.get_mut(5) {
        Some(x) => x.push_str(" Mishra"),
        None => ()
    }

    println!("{:?}", v);

    for val in v {
        println!("Value: {}", val)
    }
}
