/// Add takes 2 numbers and returns the sum
/// Example:
///
/// ```
/// let expected = 10;
/// assert_eq!(expected, calc::calc::add(8, 2));
/// ```
pub fn add(i: i32, j: i32) -> i32 {
    println!("Input: {} + {}", i, j);
    i+j
}

pub fn mul(_i: i32, _j: i32) -> i32 {
    0
}

#[cfg(test)]
mod calc_test {
    use super::*;

    #[test]
    fn add_2_numbers() {
        let input = (10, 4);
        let expected = 14;

        assert_eq!(expected, add(input.0, input.1)); // Panics in case of not equal!
    }

    #[test]
    #[ignore]
    fn mul_2_numbers() {
        let input = (10, 7);
        let expected = 70;

        assert_eq!(expected, mul(input.0, input.1));
    }
}
