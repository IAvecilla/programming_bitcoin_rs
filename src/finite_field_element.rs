use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FiniteFieldElement<const P: u128> {
    pub value: i128,
}

impl<const P: u128> From<i128> for FiniteFieldElement<P> {
    fn from(value: i128) -> Self {
        Self { value }
    }
}

impl<const P: u128> Add<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    type Output = Self;

    fn add(self, other_number: Self) -> Self {
        Self {
            value: (self.value + other_number.value).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Sub<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    type Output = Self;

    fn sub(self, other_number: Self) -> Self {
        Self {
            value: (self.value - other_number.value).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Mul<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    type Output = Self;

    fn mul(self, other_number: Self) -> Self {
        Self {
            value: (self.value * other_number.value).rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> FiniteFieldElement<P> {
    pub fn pow(&self, n: i128) -> Self {
        let exp = n.rem_euclid((P - 1_u128) as i128);
        let mut value = self.value;
        for _ in 1..exp {
            value *= self.value;
        }
        Self {
            value: value.rem_euclid(P as i128),
        }
    }
}

impl<const P: u128> Div<FiniteFieldElement<P>> for FiniteFieldElement<P> {
    type Output = Self;

    fn div(self, other_number: Self) -> Self {
        self * other_number.pow((P - 2_u128) as i128)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_two_finite_field_elements() {
        let first_field_element = FiniteFieldElement::<11>::from(1);
        let second_field_element = FiniteFieldElement::<11>::from(20);

        assert_eq!(
            first_field_element + second_field_element,
            FiniteFieldElement::<11>::from(10)
        );
    }

    #[test]
    fn test_sub_two_finite_field_elements() {
        let first_field_element = FiniteFieldElement::<11>::from(1);
        let second_field_element = FiniteFieldElement::<11>::from(20);

        assert_eq!(
            first_field_element - second_field_element,
            FiniteFieldElement::<11>::from(3)
        );
    }

    #[test]
    fn test_mul_two_finite_field_elements() {
        let first_field_element = FiniteFieldElement::<11>::from(1);
        let second_field_element = FiniteFieldElement::<11>::from(20);

        assert_eq!(
            first_field_element * second_field_element,
            FiniteFieldElement::<11>::from(9)
        );
    }

    #[test]
    fn test_pow_a_finite_field_with_a_number() {
        let first_field_element = FiniteFieldElement::<11>::from(3);

        assert_eq!(
            first_field_element.pow(3),
            FiniteFieldElement::<11>::from(5)
        );
    }

    #[test]
    fn test_div_two_finite_field_elements() {
        let first_field_element = FiniteFieldElement::<11>::from(1);
        let second_field_element = FiniteFieldElement::<11>::from(20);

        assert_eq!(
            first_field_element / second_field_element,
            FiniteFieldElement::<11>::from(5)
        );
    }
}
