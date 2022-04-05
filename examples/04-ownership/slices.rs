// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }
//     s
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn main() {
    let mut s = String::from("Hello world!");

    let word = first_word(&s);

    println!("First word: {}", word); // &&str // Immutable Reference

    s = String::from("");
    // s.push_str(" Some change."); // Mutable Reference

    println!("Value of s: {}", s);

    println!("First word: {}", word);

    // println!("First word: {}", &s[0..index]);
}
