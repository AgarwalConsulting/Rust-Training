// From: https://doc.rust-lang.org/book/ch03-05-control-flow.html

fn main() {
    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    // for c in 'A'..(91 as char) {
    for c in 'A'..'Z' {
        println!("{}", c);
    }
}
