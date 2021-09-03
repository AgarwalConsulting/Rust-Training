// #[derive(Debug)]
// struct Pair<T>(T, T);

#[derive(Debug)]
struct Pair<A, B>(A, B);

impl<T: Copy, U> Pair<T, U> {
    fn first(&self) -> T {
        self.0
    }
}

#[derive(Debug)]
struct Person{
    name: String,
}

fn main() {
    let x: i32 = 10;
    let y: f64 = 10.12;

    let p : Pair<i32, f64> = Pair(x, y);
    println!("{:?}", p);
    println!("First: {}", p.first());

    let key: &str = "Moz";
    let value: Pair<i32, i32> = Pair(10, 12);

    let moz_pair: Pair<&str, Pair<i32, i32>> = Pair(key, value);
    println!("{:?}", moz_pair);
    println!("First: {}", moz_pair.first());

    let couple = Pair(String::from("G"), String::from("A"));
    println!("{:?}", couple);
    // println!("First: {}", couple.first());

    let couple = Pair(Person{name: String::from("G")}, Person{name: String::from("A")});

    println!("{:?}", couple);
    // println!("First: {}", couple.first());
}
