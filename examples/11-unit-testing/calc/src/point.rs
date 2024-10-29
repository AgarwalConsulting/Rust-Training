#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn abs(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}

trait First {
    fn new(x: i32, y: i32) -> Self;
}

impl First for Point {
    fn new(x: i32, y: i32) -> Self {
        Point{x, y}
    }
}

#[cfg(test)]
mod point_test {
    use super::First;
    #[test]
    fn testing_new() {
        let p = super::Point::new(10, 12);

        assert_eq!(p, super::Point{x: 10, y: 12});
    }

    #[test]
    fn test_abs() {
        let sut = super::Point::new(3, 4);

        assert_eq!(sut.abs(), 5.0);
    }
}
