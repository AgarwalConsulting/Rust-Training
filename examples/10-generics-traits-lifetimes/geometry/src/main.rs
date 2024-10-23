use std::fmt::Debug;


// fn print_shape_and_area<T: Debug + Shape>(a: T) {

fn print_shape_and_area<T>(a: &T)
    where T : Debug + scratchpad::Shape
{
    use scratchpad::Shape;
    println!("{:?}", a);
    println!("\tArea: {}", a.area());
}

fn print_shape3d_and_area<T>(a: &T)
    where T : Debug + scratchpad::Shape3D
{
    use scratchpad::Shape3D;
    println!("{:?}", a);
    println!("\tArea: {}", a.area());

    println!("\tVolume: {}", a.volume());
}


fn main() {
    let s = scratchpad::square::Square(10);

    print_shape_and_area(&s);

    let mut r = scratchpad::rectangle::Rectangle{length: 2, breadth: 3};

    print_shape_and_area(&r);

    let c = scratchpad::cube::Cube(3);

    print_shape_and_area(&c);

    print_shape3d_and_area(&c);
}
