fn main() {
    let arr: [i32; 5] = [1, 3, 5, 7, 9];

    let s: &[i32] = &arr[0..3];

    println!("Value of s is: {:?}", s);
}
