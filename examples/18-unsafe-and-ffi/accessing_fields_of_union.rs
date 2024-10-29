// The key property of unions is that all fields of a union share common storage

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

fn main() {
    let u = MyUnion { f1: 1 };

    let f = unsafe { u.f1 };

    println!("Output: {}", f);
}

// Union field types are restricted to the following subset of types:

// - Copy types

// - References (&T and &mut T for arbitrary T)

// - ManuallyDrop<T> (for arbitrary T)

// - Tuples and arrays containing only allowed union field types
