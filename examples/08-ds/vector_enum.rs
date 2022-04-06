enum Animal {
    Lion{legs: i32},
    Ostrich{speed: i32},
    Dolphin,
}

fn main() {
    let v: Vec<Animal> = Vec::new();
    // let mut v = Vec::new();

    v.push(Animal::Lion{legs: 4});
    v.push(Animal::Ostrich{speed: 80});

    for el in v {
        match el {
            // Some(val) => match val {
                Animal::Lion{legs} => println!("Lion has {} legs.", legs),
                Animal::Ostrich{speed} => println!("Ostrich can achieve a speed of {} kmph.", speed),
                Animal::Dolphin => (),
            // },
            // _ => (),
        }

    }

    // match v[0] {
    //     // Some(val) => match val {
    //         Animal::Lion{legs: legs} => println!("Legs on lion: {}", legs),
    //         Animal::Ostrich{speed: speed} => (),
    //         Animal::Dolphin => (),
    //     // },
    //     // _ => (),
    // }
}
