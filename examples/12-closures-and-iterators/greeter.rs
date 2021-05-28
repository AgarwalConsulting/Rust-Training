fn greeter(by: String) -> Box<dyn Fn(String) -> String> {
    return Box::new(move |name: String| -> String {
        let mut result = String::new();
        result.push_str(&by);
        result.push(' ');
        result.push_str(&name);

        result
    })
}

fn main() {
    let say_hi = greeter(String::from("Hi"));
    let say_sayonara = greeter(String::from("Sayonara"));

    println!("{}", say_sayonara(String::from("Gaurav")));
    println!("{}", say_hi(String::from("Ranjith")));
}
