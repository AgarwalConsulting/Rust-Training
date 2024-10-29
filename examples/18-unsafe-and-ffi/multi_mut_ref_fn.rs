fn foo(x1: &mut i32, x2: &mut i32) -> i32 {
    *x1 = 42;
    *x2 = 99;
    *x1
}

fn main() {
    let mut a = 0;

    let a_ptr = &mut a as *mut i32;
    let a_ptr1 = unsafe { &mut *a_ptr };
    let a_ptr2 = unsafe { &mut *a_ptr };

    // let out = ;

    // let out = foo(&mut a, &mut a);

    println!("Output: {}", foo(a_ptr1, a_ptr2));
}
