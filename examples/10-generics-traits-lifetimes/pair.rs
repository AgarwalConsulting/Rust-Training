// #[derive(Debug)]
// struct Pair<T>(T, T);

#[derive(Debug)]
struct Pair<A, B>(A, B);

fn main() {
    let x: i32 = 10;
    let y: f64 = 10.12;

    let p : Pair<i32, f64> = Pair(x, y);
    println!("{:?}", p);

    let key = "Moz";
    let value: Pair<i32, i32> = Pair(10, 12);

    let moz_pair: Pair<&str, Pair<i32, i32>> = Pair(key, value);
    println!("{:?}", moz_pair);

    // let p = Pair(x, 12);
    // let p1 = Pair(10.0, y);

    // println!("{:?}", p);
    // println!("{:?}", p1);
}
