use std::ops::{Add, Sub};

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
}
