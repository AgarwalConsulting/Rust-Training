use calc;

#[test]
fn add_ig_test() {
    let input = (10, 4);
    let expected = 14;

    assert_eq!(expected, calc::calc::add(input.0, input.1)); // Panics in case of not equal!
}

// #[test]
// #[ignore]
// fn mul_2_numbers() {
//     let input = (10, 7);
//     let expected = 70;

//     assert_eq!(expected, calc::calc::mul(input.0, input.1));
// }
