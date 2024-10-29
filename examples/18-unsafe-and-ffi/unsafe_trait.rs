unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

fn main() {}

// A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.

// By using unsafe impl, we’re promising that we’ll uphold the invariants that the compiler can’t verify.
