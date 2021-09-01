fn first_word(s: &String) -> usize {
    let mut idx = 0;

    for el in s.bytes() {
        if el == b' ' {
            return idx
        }
        idx+=1;
    }

    return s.len()
}

fn main() {
    let mut s = String::from("We the people!");

    let idx = first_word(&s);

    // s.insert_str(0, "Cisco:");

    println!("Index is: {}", idx);

    println!("First Word: {}", &s[0..idx])
}
