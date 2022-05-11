use super::units;

#[derive(Debug, PartialEq)]
pub struct Quantity<T> {
    value: T,
    units: units::Units,
}

impl<T> Quantity<T> {
    pub fn new(value: T, units: units::Units) -> Self {
        Quantity {
            value,
            units
        }
    }
}
