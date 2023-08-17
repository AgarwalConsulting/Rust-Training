fn main() {
    let st = String::from("नमस्ते"); // Na-Ma-S-Te // char => unicode code point
    // let st = String::from("hello 😀");
    // let st = String::from("hello");

    println!("Sub string: {}", &st[0..3]); //

    println!("No. of bytes: {}", st.len()); // ? bytes
    println!("No. of characters: {}", st.chars().count());

    for el in st.chars() {
        println!("{}", el); //
    }

    println!("{:?}", st.bytes());
}
