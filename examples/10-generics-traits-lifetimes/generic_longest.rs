fn largest<T>(v: &Vec<T>) -> T {
    let mut max = v[0];
    for el in v.iter() {
        if *el > max {
            max = *el;
        }
    }
    max
}

fn main() {
    let v = vec!(10, 12, 121, 1257, 36, 26, 84, 23);

    // let max = largest_int(&v);
    let max = largest(&v);

    println!("Largest Number: {}", max);

    let v = vec!('A', 'Z', '😋');

    // let max = largest_char(&v);
    let max = largest(&v);

    println!("Largest Character: {}", max);
}
