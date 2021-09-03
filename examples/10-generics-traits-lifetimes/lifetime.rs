// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

fn main() {
    let x = 5;

    {
        let r;
        r = &x;
        println!("r: {}", r);
    }

    println!("x: {}", x);
}
