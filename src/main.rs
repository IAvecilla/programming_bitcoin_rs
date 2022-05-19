mod errors;
mod finite_field_element;

fn main() {
    let _ = finite_field_element::FiniteFieldElement::<11>::new(2);
}
