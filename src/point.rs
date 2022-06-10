use anyhow::{bail, ensure};
use std::ops::Add;

#[derive(Debug, Copy)]
pub struct Point<const A: i64, const B: i64> {
    x: Option<i64>,
    y: Option<i64>,
}

impl<const A: i64, const B: i64> Point<A, B> {
    pub fn new(x: Option<i64>, y: Option<i64>) -> Result<Self, anyhow::Error> {
        match (x, y) {
            (Some(x), Some(y)) => {
                ensure!(
                    y.pow(2) == (x.pow(3) + A * x + B),
                    "Point is not on the curve"
                );
                Ok(Self {
                    x: Some(x),
                    y: Some(y),
                })
            }
            (None, None) => Ok(Self { x: None, y: None }),
            _ => bail!("Invalid point"),
        }
    }

    pub fn point_at_infinity() -> Self {
        Point::<A, B>::new(None, None).unwrap()
    }
    // pub slope(self, other: Self) ->Result
}

impl<const A: i64, const B: i64> PartialEq for Point<A, B> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<const A: i64, const B: i64> Add<Point<A, B>> for Point<A, B> {
    type Output = Self;
    fn add(self, other: Point<A, B>) -> Self {
        match (self.x, self.y, other.x, other.y) {
            (None, None, ..) => other,
            (.., None, None) => self,
            (Some(x1), Some(y1), Some(x2), Some(y2)) => {
                if x2 == x1 {
                    return Point::<A, B>::point_at_infinity();
                }

                if self == other {
                    if y1 == 0 || y2 == 0 {
                        return Point::<A, B>::point_at_infinity();
                    }
                    let slope = (3 * x1.pow(2) + A) / (2 * y1);
                    let x3 = slope.pow(2) - (2 * x1);
                    let y3 = slope * (x1 - x3) - y1;
                    return Point::<A, B>::new(Some(x3), Some(y3)).unwrap();
                }

                let slope = (y2 - y1) / (x2 - x1);
                let x3 = slope.pow(2) - x1 - x2;
                let y3 = slope * (x1 - x3) - y1;
                Point::<A, B>::new(Some(x3), Some(y3)).unwrap()
            }
            // This case is never reached
            _ => panic!("Invalid Point"),
        }
    }
}

impl<const A: i64, const B: i64> Clone for Point<A, B> {
    fn clone(&self) -> Self {
        Point::<A, B>::new(self.x, self.y).unwrap()
    }
}

#[cfg(test)]
mod point_tests {
    use super::*;

    #[test]
    fn test00_create_valid_point() {
        assert!(Point::<5, 7>::new(Some(-1), Some(-1)).is_ok());
    }

    #[test]
    fn test01_create_invalid_point() {
        assert!(Point::<5, 7>::new(Some(-1), Some(-2)).is_err());
    }

    #[test]
    fn test02_points_with_same_cords_are_equal() {
        let point1 = Point::<5, 7>::new(Some(-1), Some(-1)).unwrap();
        let point2 = Point::<5, 7>::new(Some(-1), Some(-1)).unwrap();

        assert_eq!(point1, point2);
    }

    #[test]
    fn test03_points_with_different_cords_are_different() {
        let point1 = Point::<5, 7>::new(Some(-1), Some(-1)).unwrap();
        let point2 = Point::<5, 7>::new(Some(18), Some(77)).unwrap();

        assert_ne!(point1, point2);
    }

    #[test]
    fn test04_sum_inifity_and_another_point_returns_the_point() {
        let infinity = Point::<5, 7>::point_at_infinity();
        let point = Point::<5, 7>::new(Some(-1), Some(-1)).unwrap();

        assert_eq!(point + infinity, point);
        assert_eq!(infinity + point, point);
    }

    #[test]
    fn test05_two_different_points_from_the_same_curve_can_be_added() {
        let point1 = Point::<5, 7>::new(Some(2), Some(5)).unwrap();
        let point2 = Point::<5, 7>::new(Some(-1), Some(-1)).unwrap();
        let expected_point3 = Point::<5, 7>::new(Some(3), Some(-7)).unwrap();

        assert_eq!(point1 + point2, expected_point3);
        assert_eq!(point2 + point1, expected_point3);
    }

    #[test]
    fn test06_adding_the_same_point_results_infinity() {
        let point1 = Point::<5, 7>::new(Some(-1), Some(-1)).unwrap();
        let point2 = Point::<5, 7>::new(Some(-1), Some(1)).unwrap();

        let expected_point3 = Point::<5, 7>::point_at_infinity();

        assert_eq!(point1 + point2, expected_point3);
    }
}
