fn main() {
    let mut v = Vec::new();

    let mut a = String::from("Anushka");
    let mut p = String::from("Prashant");
    let mut r = String::from("Ranjith");
    let mut s = String::from("Subham");

    v.push(&a);
    v.push(&p);
    v.push(&r);
    v.push(&s); // Immutable reference

    println!("{:?}", v);

    println!("{}", &s); // Mutable reference, without mutation

    // println!("{:?}", v);

    s.push_str(" Mishra");

    println!("{}", &s);
}
