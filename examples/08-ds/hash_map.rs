use std::collections::HashMap;

fn main() {
    let mut m: HashMap<&String, &mut i32> = HashMap::new();

    let s: String = String::from("Gaurav");

    let mut i = 100;

    m.insert(&s, &mut i);

    // println!("Value associated with s: {:?}", m.get(&s));

    if let Some(val) = m.get(&s) {
        println!("Found value: {}", val);
    }

    if let Some(val) = m.get(&String::from("Gaurav")) {
        println!("Found value: {}", val);
    } else {
        println!("Not found!");
    }

    // m.insert(&s, &99); // Discarding old value from hashmap!

    let r = String::from("Ranjith");
    let mut r_val = 99;
    m.entry(&r).or_insert(&mut r_val);

    // s.push_str(" Agarwal");
    if let Some(val) = m.get_mut(&r) {
        **val += 1;
    }

    println!("{:?}", m);
}
