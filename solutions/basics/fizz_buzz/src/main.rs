fn fizz_buzz(i: i32) -> String {
    if i % 3 == 0 && i % 5 == 0 {
        String::from("FizzBuzz")
    } else if i % 3 == 0 {
        String::from("Fizz")
    } else if i % 5 == 0 {
        String::from("Buzz")
    } else {
        i.to_string()
    }
}

fn main() {
    for el in 1..101 {
        println!("{}", fizz_buzz(el));
    }
}
