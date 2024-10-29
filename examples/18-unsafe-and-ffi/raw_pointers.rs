fn main() {
    let mut v = vec!(1, 2, 3, 4);

    let ptr = &v[0] as *const i32;

    v.push(5);

    println!("V: {:?}", v);
    println!("Address in Ptr: {:p} | Value in Ptr: {}", ptr, unsafe { *ptr });
}
