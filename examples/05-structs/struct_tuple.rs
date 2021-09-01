#[derive(Debug)]
struct Color(i32, i32, i32);

fn main() {
    let c: Color = Color(140, 130, 12);

    let Color(r, g, b) = c;

    println!("Red value is: {}", r);
    println!("The value is: {:?}", c);

    // println!("Red value is: {}", r.red());
    // println!("Green value is: {}", r.green());
    // println!("Blue value is: {}", r.blue());
}
