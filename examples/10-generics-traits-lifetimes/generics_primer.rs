fn main() {
    let v = vec!(10, 12, 121, 1257, 36, 26, 84, 23);

    let mut max = v[0];
    for el in v.iter() {
        if *el > max {
            max = *el;
        }
    }

    println!("Largest Number: {}", max);

    let v = vec!(45, 64, 13, 64, 12, 86, 82, 324, 12);

    let mut max = v[0];
    for el in v.iter() {
        if *el > max {
            max = *el;
        }
    }

    println!("Largest Number: {}", max);
}
