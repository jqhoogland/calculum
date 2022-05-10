mod constants {
    #[repr(i8)]
    #[derive(Debug, PartialEq, Clone)]
    pub enum Prefix {
        YOTTA = 24,
        ZETTA = 21,
        EXA = 18,
        PETA = 15,
        TERA = 12,
        GIGA = 9,
        MEGA = 6,
        KILO = 3,
        HECTO = 2,
        DECA = 1,
        NONE = 0,
        DECI = -1,
        CENTI = -2,
        MILLI = -3,
        MICRO = -6,
        NANO = -9,
        PICO = -12,
        FEMTO = -15,
        ATTO = -18,
        ZEPTO = -21,
        YOCTO = -24,
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    pub enum Dimension {
        TIME,
        LENGTH,
        MASS,
        CHARGE,
        TEMP,
        AMOUNT,
        LUMINOSITY,
        OTHER(String)
    }
}

mod unit {
    use super::constants::{Prefix, Dimension}

    #[derive(Debug, PartialEq)]
    pub struct BaseUnit {
        pub name: String,
        pub pow: i8,
        pub dim: Dimension,
    }

    #[derive(Debug, PartialEq)]
    pub struct DerivedUnit {
        pub name: String,
        pub pow: i8,
        pub dim: Dimension,
        pub subunits: Vec<BaseUnit>
    }

    #[derive(Debug, PartialEq)]
    pub enum UnitType {
        BASE(BaseUnit),
        DERIVED(DerivedUnit),
    }

    pub type ToMetricConversion = f64;

    #[derive(Debug, PartialEq)]
    pub enum Unit {
        METRIC(Prefix, UnitType),
        CUSTOMARY(ToMetricConversion, UnitType),
    }

    /// Constructors
    impl Unit {
        pub fn new_atomic(atom: &str) -> Self {
            new(Prefix.NONE, atom, 1)
        }

        pub fn new_bare(atom: &str, power: i8) -> Self {
            new(Prefix.NONE, atom, power)
        }

        pub fn new(pre: Prefix, unit: &str, power: i8) -> Self {
            if is_metric(unit) {
                new_metric(pre, unit, power)
            } else {
                new_customary(unit, power)
            }
        }

        pub fn new_metric(pre: Prefix, unit: &str, power: i8) -> Self {
            Unit.METRIC(
                pre,
                if is_base(unit) {
                    UnitType.BASE(BaseUnit({}))
                }
            )
        }
    }

    /// Comparators
    impl Unit {
        /// Ignore prefixes
        pub fn is_similar(&self, other: &Self) -> bool {
            self.atom == other.atom && self.power == other.power
        }

        /// Consider only dimension
        pub fn is_compatible(&self, other: &Self) -> bool {
            self.dim == other.dim
        }
    }

    fn get_dimension(unit: &str) -> Dimension {
        use Dimension::*;
        match unit {
            "s" => TIME,
            "m" => LENGTH,
            "C" => CHARGE,
            "K" => TEMP,
            "mol" => AMOUNT,
            "cd" => LUMINOSITY,
            _ => OTHER(unit.to_string())
        }
    }
}

mod units {
    use std::error::Error;
    use std::{fmt, convert};
    use super::unit::{self, Unit};

    #[derive(Debug, PartialEq)]
    pub enum Units {
        DIM(Vec<Unit>),
        // ALIAS(alias, Vec<Unit>),
        BASE
    }

    impl Units {
        pub fn new(units: Vec<Unit>) -> Self {
            match units.len() {
                0 => Units::BASE,
                _ => Units::DIM(sort_units(units))
            }
        }
    }

    pub fn sort_units(mut units: Vec<Unit>) -> Vec<Unit> {
        units.sort_by(|a, b| a.dim.partial_cmp(&b.dim).unwrap());
        units
    }
}

mod quantity {
    use super::units;
    use std::str::FromStr;

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
    use super::unit::{Unit, Prefix};
    use super::units::Units;
    use super::quantity::Quantity;

    #[test]
    fn it_normalizes_si_base_unit_order() {
        let value = 1;

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
        let value = 1;

        let m = ||  Unit::new_atomic("m");
        let g = || Unit::new_atomic("g");
        let s2 = || Unit::new_bare("s", -2);
        let n = || Unit::new_bare("N", 1);

        let us1 = Units::new(vec![n()]);
        let us2 = Units::new(vec![g(), m(), s2()]);

        assert_eq!(us1, us2);

        let us3 = Units::new(vec![n(), m()]);
        let us4 = Units::new(vec![g(), m(), s2(), m()]);

    }

    #[test]
    fn it_ignores_prefixes_when_detecting_similarity() {
        let m = || Unit::new_bare("m", 1);
        let cm =  || Unit::new(Prefix.CENTI, "m", 1);
        let km =  || Unit::new(Prefix.KILO, "m", 1);

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
