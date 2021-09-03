fn summer() -> Box<dyn FnMut(i32) -> i32> {
    let mut sum = 0;

    return Box::new(move |num: i32| -> i32 {
        sum = sum + num;
        sum
    });
}


fn main() {
    let mut sum1 = summer();
    let mut sum2 = summer();

    println!("{}", sum1(10)); //
    println!("{}", sum1(20)); //
    println!("{}", sum1(30)); //

    println!("{}", sum2(12)); //
    println!("{}", sum2(20)); //
    println!("{}", sum2(30)); //
}
