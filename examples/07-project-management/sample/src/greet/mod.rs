pub mod people;

pub fn greeting(name: String) -> String {
    let mut msg = String::from("Hello, ");
    msg.push_str(&name);

    return msg;
}
