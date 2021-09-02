#[derive(Debug)]
struct Point<T>(T, T);

impl<T: Copy> Point<T> {
    fn first(&self) -> T {
        self.0
    }
}

// #[derive(Debug)]
// struct Person{
//     name: String,
// }

fn main() {
    let graph_point: Point<i32> = Point(10, 12);
    // let graph_point = Point(10, 12);

    println!("Graph Point: {:?}", graph_point);

    println!("First: {}", graph_point.first());

    let loc: Point<f64> = Point(42.123, 10.12);
    // let loc = Point(42.123, 10.12);

    println!("Location: {:?}", loc);

    println!("First: {}", loc.first());

    // let s: Point<&str> = Point("Hi", "Bye");
    // // let s = Point("Hi", "Bye");

    // let a = s.first();

    // println!("First: {}", a);

    // let p: Point<Person> = Point(Person{name: String::from("G")}, Person{name: String::from("A")});

    // let g = p.first();

    // println!("First: {}", p);
}
