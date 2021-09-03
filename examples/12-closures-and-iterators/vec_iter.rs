fn main() {
    let mut l = Vec::new();

    l.push(1);
    l.push(2);
    l.push(3);
    l.push(5);
    l.push(7);

    for el in l.iter() {
        println!("\t{}", el)
    }

    println!("{:?}", l);

    let l1: Vec<i32> = l.iter().map(|el| el * 2).collect();

    println!("{:?}", l1);
    println!("{:?}", l)
}
