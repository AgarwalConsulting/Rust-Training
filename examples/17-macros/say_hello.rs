// This is a simple macro named `say_hello`.
macro_rules! say_hello {
    // `()` indicates that the macro takes no argument.
    ($($expression:expr),*) => {
        // The macro will expand into the contents of this block.
        // println!("Hello! {}", $expression);
        print!("Hello!");

        $(
            print!(" {}", $expression);
        )*

        print!("\n");

    };
}

fn main() {
    // This call will expand into `println!("Hello!")`
    say_hello!();
    say_hello!(1);
    say_hello!(1, 2);
}
