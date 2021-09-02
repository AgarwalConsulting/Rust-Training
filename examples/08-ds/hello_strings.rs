fn print_str(s: &str) {
    println!("{}", s);
}

fn main() {
    // String as an alias for Vec<u8>
    let s = String::from("hello😋");

    let v = &s[1..2];

    print_str(v);
    // println!("{}", v);

    // for el in s.bytes() {
    //     println!("{}", el as char);
    // }
}
