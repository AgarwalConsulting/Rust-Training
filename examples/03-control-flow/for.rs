// From: https://doc.rust-lang.org/book/ch03-05-control-flow.html
// Listing 3-4: Looping through each element of a collection using a while loop
// Listing 3-5: Looping through each element of a collection using a for loop

fn main() {
    let a = [10, 20, 30, 40, 50];

    // let mut index = 0;
    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // for let mut index = 0; index < a.len(); index++ {
    //     let element = a[index]
    //     println!("the value is: {}", element);
    // }

}
