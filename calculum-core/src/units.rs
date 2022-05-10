trait UnitEq<T> {
    /// Up to a dimensionless scaling factor
    fn is_similar(&self, other: T) -> bool;

    /// Can be converted (i.e., has the same dimension)
    fn is_compatible(&self, other: T) -> bool;
}

mod unit {
    use super::UnitEq;
    use crate::constants::{UnitPrefix, Dimension, UnitAtom};

    #[derive(Debug, PartialEq)]
    pub struct Unit {
        pub prefix: Option<UnitPrefix>,
        pub unit: UnitAtom,
        pub power: i8,
    }

    /// Accessors
    impl Unit {
        pub fn is_metric(&self) -> bool {
            match self.prefix {
                Some(_) => true,
                None => false,
            }
        }

        pub fn is_base(&self) -> bool {
            match self.subunits() {
                Some(_) => false,
                None => true,
            }
        }

        pub fn subunits(&self) -> Option<Vec<Unit>> {
            None
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

        fn is_compatible(&self, other: &Self) -> bool {
            self.dimension() == other.dimension()
        }
    }
}

mod units {
    use super::UnitEq;
    use super::unit::{Unit};

    #[derive(Debug, PartialEq)]
    pub enum Units {
        Dim(Vec<Unit>),
        DimLess
    }

    /// Constructors
    impl Units {
        pub fn new(units: Vec<Unit>) -> Self {
            match units.len() {
                0 => Units::DimLess,
                _ => Units::Dim(sort_units(units))
            }
        }
    }

    pub fn sort_units(mut units: Vec<Unit>) -> Vec<Unit> {
        units.sort_by(|a, b| a.dimension().partial_cmp(&b.dimension()).unwrap());
        units
    }

    impl UnitEq<&Units> for Units {
        fn is_similar(&self, other: &Self) -> bool {
            use Units::*;

            match (self, other) {
                (Dim(_self_units), Dim(_other_units)) => false,  // FIXME
                (Dim(_), _) => false,
                (_, Dim(_)) => false,
                (_, _) => true,
            }
        }

        fn is_compatible(&self, other: &Self) -> bool {
            use Units::*;

            match (self, other) {
                (Dim(_self_units), Dim(_other_units)) => false,  // FIXME
                (Dim(_), _) => false,
                (_, Dim(_)) => false,
                (_, _) => true,
            }
        }
    }
}

mod quantity {
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
}


#[cfg(test)]
mod tests {
    use crate::constants::UnitPrefix;
    use super::unit::Unit;
    use super::units::Units;
    use super::UnitEq;


    #[test]
    fn it_normalizes_si_base_unit_order() {
        let _value = 1;

        let m = ||  Unit::new_bare("m", 1);
        // SI uses kg as its base, but that's ridiculous.
        let g = || Unit::new_bare("g", 1);
        let s2 = || Unit::new_bare("s", -2);

        let us1 = Units::new(vec![m(), g(), s2()]);
        let us2 = Units::new(vec![g(), s2(), m()]);
        let us3 = Units::new(vec![s2(), m(), g()]);
        let us4 = Units::new(vec![s2(), g(), m()]);

        assert_eq!(us1, us2);
        assert_eq!(us1, us3);
        assert_eq!(us1, us4);
        assert_eq!(us2, us3);
        assert_eq!(us2, us4);
        assert_eq!(us3, us4);
    }

    #[test]
    fn it_normalizes_si_unit_aliases() {
        let _value = 1;

        let m = ||  Unit::new_atomic("m");
        let g = || Unit::new_atomic("g");
        let s2 = || Unit::new_bare("s", -2);
        let n = || Unit::new_bare("N", 1);

        let us1 = Units::new(vec![n()]);
        let us2 = Units::new(vec![g(), m(), s2()]);

        assert_eq!(us1, us2);

        let _us3 = Units::new(vec![n(), m()]);
        let _us4 = Units::new(vec![g(), m(), s2(), m()]);

    }

    #[test]
    fn it_ignores_prefixes_when_detecting_similarity() {
        let m = || Unit::new_bare("m", 1);
        let cm =  || Unit::new(Some(UnitPrefix::Centi), "m", 1);
        let km =  || Unit::new(Some(UnitPrefix::Kilo), "m", 1);

        assert!(m().is_similar(&cm()));
        assert!(cm().is_similar(&m()));
        assert!(cm().is_similar(&km()));
        assert!(km().is_similar(&cm()));
    }

    #[test]
    fn it_checks_dimensions_when_detecting_compatibility() {
        assert_eq!(true, false);
    }

}
