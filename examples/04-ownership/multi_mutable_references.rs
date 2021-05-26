fn calculate_len(s: &String) -> usize {
    s.len()
}

fn main() {
    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &s;

    println!("The value of s1 is: {}", s1);
    println!("Length of s1: {}", calculate_len(s1));
    println!("The value of s2 is: {}", s2);
    println!("Length of s2: {}", calculate_len(s2));

    let s3 = &mut s; // s3 is immutable; s3 value is a mutable reference!

    s3.push_str(" world");

    println!("The value of s3 is: {}", s3);

    println!("Length of s3: {}", calculate_len(s3));

    println!("The value of s is: {}", s);
}
