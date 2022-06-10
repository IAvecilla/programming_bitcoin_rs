mod errors;
mod finite_field_element;
mod point;

fn main() {
    let _ = finite_field_element::FiniteFieldElement::<11>::new(2);
    let _ = point::Point::<3, 4>::new(Some(1), Some(2));
}
