enum Animal {
    Lion{legs: i32},
    Ostrich{speed: i32},
    Dolhpin,
}

fn main() {
    let v: Vec<Animal> = Vec::new();

    match v[0] {
        Lion => {},
        Ostrich => {},
        Dolphin => {},
    }
}
