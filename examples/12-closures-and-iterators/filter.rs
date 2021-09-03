fn filter<'a, T>(list: &'a Vec<&str>, predicate: T) -> Vec<&'a str>
where T: Fn(&str) -> bool {
    let mut result: Vec<&str> = Vec::new();

    for el in list {
        if predicate(el) {
            result.push(el);
        }
    }

    return result;
}

fn main() {
    let heroes = vec!("Iron Man", "Batman", "Superman", "Spider-man", "Wonder Woman", "Iron Fist", "Daredevil", "Supergirl", "Flash");

    // let heroes_without_man: Vec<_> = heroes.iter().filter(|el| {
    //     !el.contains("man")
    // }).collect();

    let heroes_without_man = filter(&heroes, |el: &str| -> bool {
        !el.contains("man")
    });

    println!("{:?}", heroes_without_man);
}
