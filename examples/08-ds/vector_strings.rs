fn main() {
    // let mut v = Vec::new();
    let mut v: Vec<&mut String> = Vec::new();

    let a = String::from("Anushka");
    let p = String::from("Prashant");
    let r = String::from("Ranjith");
    let s = String::from("Subham");

    v.push(a);
    v.push(p);
    v.push(r);
    v.push(s);

    println!("{:?}", v);

    v[3].push_str(" Mishra");

    println!("{:?}", v);

    println!("{}", s);
}
