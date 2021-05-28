fn main() {
    let l = vec!(4, 5, 3, 9, 3, 1, 9, 5, 0, 3, 4, 3, 6, 4, 6, 7);

    let sum = l.iter().rev().enumerate().fold(0, |sum, (idx, el)| {
        if idx %2 == 1 {
            let val = if el * 2 > 9 { el * 2 - 9 } else { el * 2 };
            sum + val
        } else {
            sum + *el
        }
    });

    // let first_step: Vec<i32> = l.iter().rev().enumerate().map(|(idx, el)| -> _ {
    //     if idx % 2 == 1 {
    //         el * 2
    //     } else {
    //         *el
    //     }
    // }).map(|el| -> _ {
    //     if el > 9 { el - 9} else { el }
    // }).rev().collect();

    // println!("{:?}", first_step);

    // println!("Sum: {}", first_step.iter().sum::<i32>());
    println!("Sum: {}", sum);
    println!("{}", sum % 10 == 0);
}
