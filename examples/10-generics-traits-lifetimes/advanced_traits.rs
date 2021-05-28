use std::fmt::Debug;
// fn display(s: impl Summary) {
//     println!("{}", s.summarize());
// }

trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for i32 {
    fn summarize(&self) -> String {
        format!("{:?}", self)
    }
}

fn display<T, U>(s: T, c: U)
where
T: Summary + Debug + Copy,
U: Drop {
    println!("{}", s.summarize());
}

fn main() {
    display(10, "doesn't matter");
}
