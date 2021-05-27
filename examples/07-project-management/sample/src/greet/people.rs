use super::super::people::Person;

pub fn greeting(p: Person) -> String {
// pub fn greeting(p: Person) -> String {
    let mut msg = String::from("Hello, ");
    msg.push_str(&p.name);

    return msg;
}
