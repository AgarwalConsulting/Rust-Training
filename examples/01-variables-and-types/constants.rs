fn main() {
    let the_answer = 42;
    const THE_ANSWER: i32 = 42;

    println!("The answer to the life, the universe and everything is {}", the_answer);
    println!("The answer to the life, the universe and everything is {}", THE_ANSWER);

    let the_answer = 10;
    // let THE_ANSWER = 10;

    println!("The answer to the life, the universe and everything is {}", the_answer);
    println!("The answer to the life, the universe and everything is {}", THE_ANSWER);
}
