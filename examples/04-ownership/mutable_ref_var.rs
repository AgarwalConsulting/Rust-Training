fn main() {
    let mut i = 10;
    let mut j = 20;

    let mut k: &mut i32 = &mut i;

    println!("Value of k: {}", k);

    k += 1;

    println!("Value of i: {}", i);

    k = &mut j;

    println!("Value of k: {}", k);
}
