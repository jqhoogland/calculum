mod units {
    use std::str::FromStr;
    use std::string::ToString;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    pub struct Unit<'a> {
        unit: &'a str
    }

    #[derive(Debug)]
    pub enum Units<'a> {
        DIM(Vec<Unit<'a>>),
        BASE
    }

    impl<'a> FromStr for Units<'a> {
        type Err = ParseUnitsError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Units{ units: vec![Unit{ unit: s } ]})
        }
    }

    impl<'a> ToString for Units<'a> {
        fn to_string(&self) -> String {
            self.units.join("")
        }
    }

    impl<'a> PartialEq for Units<'a> {
        fn eq(&self, other: &Self) -> bool {
            self.to_string() == other.to_string()
        }
    }

    #[derive(Debug)]
    struct ParseUnitsError {}

    impl fmt::Display for ParseUnitsError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Units parser encountered a formatting error")
        }
    }

    impl Error for ParseUnitsError {}
}

mod quantity {
    use super::units;
    use std::str::FromStr;

    #[derive(Debug, PartialEq)]
    pub struct Quantity<'a, T> {
        value: T,
        units: units::Units<'a>,
    }

    impl<T> Quantity<'static, T> {
        pub fn new(value: T, s: &str) -> Quantity<'static, T> {
            Quantity {
                value,
                units: units::Units::from_str(s).unwrap()
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::quantity::Quantity;

    #[test]
    fn it_compares_without_regard_to_unit_order() {
        let value = 1;

        let q1 = Quantity::new(value, "m.kg/s2");
        let q2 = Quantity::new(value, "kg.m/s2");
        let q3 = Quantity::new(value, "kg/s2.m");

        assert_eq!(q1, q2);
        assert_eq!(q1, q3);
        assert_eq!(q2, q3);
    }
}
