fn main() {
    // C-like syntax
    // int a[];

    // let x: [i32; 2];

    // x = [1, 3];

    // println!("The value of x: {:?}", x); // ?

    let x = ['A'; 5];
    let mut x = ['A', 'B', 'C', 'D', 'E'];

    println!("The value of x: {:?}", x);

    println!("The third element: {}", x[2]);

    x[2] = 'F';

    println!("The value of x: {:?}", x);

    println!("Out of bound value: {}", x[x.len() + 1]);
}
