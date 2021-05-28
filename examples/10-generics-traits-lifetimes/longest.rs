fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("hello");

    let result;

    {
        let b = String::from("hi");
        result = longest(&a, &b);
    }

    println!("The largest is: {}", result);
}
