fn a() {
    let a = 10;
    b();
    ...
}

fn b() {
    let b = 20;
    c();
    ...
}

fn c() {
    let c = 30;
    // println!("{}, {}, {}", a, b, c)
    println!("{}", c);
    ...
}

fn main() {
    a();
}
