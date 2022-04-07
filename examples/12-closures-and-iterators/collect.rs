fn collect(inp: &Vec<f64>, op: Box<dyn Fn(&f64) -> f64>) -> Vec<f64> {
    let mut out = Vec::new();

    for el in inp {
        out.push(op(el));
    }

    return out
}

// fn collect<T>(inp: &Vec<T>, op: fn(&T) -> T) -> Vec<T> {
//     let mut out = Vec::new();

//     for el in inp {
//         out.push(op(el));
//     }

//     return out
// }

fn main() {
    let primes: Vec::<f64> = vec!(1.0, 2.0, 3.0, 5.0, 7.0);

    let doubled_primes = collect(&primes, Box::new(|el| {*el * 2.0}));
    // let doubled_primes = collect(&primes, |el| {*el * 2.0});

    println!("Doubled: {:?}", doubled_primes);
}
