fn main() {
    // let c: char = '😋';

    // println!("The value of c is: {}", c as i32);

    let i: u8 = 78;

    // println!("The value of i is: {}", i as char); // Doesn't work!

    let c: char = i.into();

    println!("The value of c is: {}", c); // Doesn't work!
}
