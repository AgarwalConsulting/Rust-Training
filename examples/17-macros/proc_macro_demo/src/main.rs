use proc_macro_demo::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes {
    x: f64,
    y: f64,
}

// impl HelloMacro for Pancakes {
//     fn hello_macro() {
//         println!("Hello, Macro! My name is Pancakes!");
//     }
// }

// TODO: Fix Me! https://doc.rust-lang.org/book/ch19-06-macros.html#how-to-write-a-custom-derive-macro

fn main() {
    Pancakes::hello_macro();
}
