fn main() {
    let namaste = String::from("नमस्ते");
    let russian_word = String::from("Здравствуйте");

    for b in namaste.chars() {
        println!("{}", b);
    }

    for b in russian_word.chars() {
        println!("{}", b);
    }

    let partial_word = &russian_word[0..6];

    println!("{}", partial_word);
}
