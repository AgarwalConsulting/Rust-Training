fn main() {
    let mut l: Vec<String> = vec!();

    l.push(String::from("Gaurav"));
    l.push(String::from("Prashant"));
    l.push(String::from("Ranjith"));
    l.push(String::from("Subham"));
    l.push(String::from("Anushka"));

    let lengths: Vec<usize> = l.iter().map(|el| {
        el.len()
    }).collect();

    println!("{:?}", l);
    println!("{:?}", lengths);
    println!("{:?}", l);
}
