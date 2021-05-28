fn main() {
    let mut l = vec!();

    l.push(1);
    l.push(2);
    l.push(3);
    l.push(5);
    l.push(7);

    // let mut el: &i32;
    // for el in l.iter_mut() {
    //     *el *= 2
    // }

    // let el: i32;
    // for el in l.iter() {
    //     *el *= 2
    // }

    // let el: i32;
    for el in l.into_iter() {
        el * 2;
    }

    println!("{:?}", l);

    // let l2: Vec<i32> = l.iter().map(|mut el| -> _ {
    //     *el *= 2
    // }).collect();

    // println!("{:?}", l);

    // let mut l_iter = l.iter();
    // let mut l_iter = l.iter_mut();

    // println!("{:?}", l_iter.next());
    // println!("{:?}", l_iter.next());
    // println!("{:?}", l_iter.next());
    // println!("{:?}", l_iter.next());
    // println!("{:?}", l_iter.next());
    // println!("{:?}", l_iter.next());

    // println!("{:?}", l);


    // println!("{:?}", l);


    // println!("{:?}", sqrt_vals);

    // println!("{:?}", l.iter().sum());
    // println!("{:?}", l);
}
