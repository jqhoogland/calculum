use crate::constants::{UnitPrefix, Dimension, UnitAtom};

#[derive(Debug, PartialEq)]
pub struct MetricBaseUnit {  // g, m, s, ...
    pub unit: MetricBaseUnitAtom,
    pub power: i8,
}

#[derive(Debug, PartialEq)]  // kg, mm, h, ...
pub struct MetricDerivedUnit {
    pub prefix: UnitPrefix,
    pub unit: MetricDerivedUnitAtom,
    pub power: i8,
}

#[derive(Debug, PartialEq)]
pub struct CustomaryUnit { // lbs, ft, gal, ...
    pub unit: CustomaryUnitAtom,
    pub power: i8,
}

#[derive(Debug, PartialEq)]
pub struct Arbitrary { // [my_custom_units]
    pub unit: String,
    pub power: i8,
}

#[derive(Debug, PartialEq)]
pub enum Unit {
    Derived(MetricDerivedUnit),
    Base(MetricBaseUnit),
    Customary(CustomaryUnit),
    Arbitrary(ArbitraryUnit)
}

impl Unit {
    pub fn is_metric(&self) -> bool {
        match self {
            Unit::Derived(_) | Unit::Base(_) => true,
            _ => false
        }
    }

    pub fn is_base(&self) -> bool {
        match self {
            Unit::Base(_) => true,
            _ => false
        }
    }

    pub fn dimension(&self) -> Dimension {
        self.unit.to_dimension()
    }
}

/// Constructors
impl Unit {
    pub fn new_atomic(atom: &str) -> Self {
        Unit::new(Some(UnitPrefix::None), atom, 1)
    }

    pub fn new_bare(unit: &str, power: i8) -> Self {
        Unit::new(Some(UnitPrefix::None), unit, power)
    }

    pub fn new(prefix: Option<UnitPrefix>, unit: &str, power: i8) -> Self {
        Unit {
            prefix,
            unit: UnitAtom::from(unit.to_string()),
            power
        }
    }
}

impl UnitEq<&Self> for Unit {
    fn is_similar(&self, other: &Self) -> bool {
        self.unit == other.unit && self.power == other.power
    }

    fn is_commensurable(&self, other: &Self) -> bool {
        self.dimension() == other.dimension()
    }
}


#[cfg(test)]
mod tests {
    use super::Unit;
    use super::units::Units;
    use super::UnitEq;
}