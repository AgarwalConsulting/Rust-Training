fn give_val() -> String {
    let val = String::from("Hello");
    return val;
}

fn main() {
    let v = give_val();

    println!("{}", v);
}
