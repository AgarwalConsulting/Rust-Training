// From: https://doc.rust-lang.org/reference/types/slice.html

fn main() {
    // A heap-allocated array, coerced to a slice
    let mut boxed_array: Box<[i32]> = Box::new([1, 2, 3]);

    // Shared Slice => slice
    let slice = &boxed_array[..];

    println!("slice: {:?}", slice);

    // A (shared) slice into an array
    let slice: &[i32] = &boxed_array[1..2];

    println!("slice: {:?}", slice);

    boxed_array[0] = 10;

    // println!("slice: {:?}", slice);
    println!("boxed array: {:?}", boxed_array);
}
