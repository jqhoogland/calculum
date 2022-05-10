
impl From<String> for UnitAtom {
    pub fn from(s: String) -> Self {
        UnitAtom.METER
    }
}

mod units {
    use calculum::units::*;

    impl convert::TryFrom for Unit
        where T: {
        type Error = ParseUnitsError;

        fn try_from(s: String) -> Result<Self, Self::Error> {
            Ok(Unit {unit: s})
        }
    }

    impl<'a> Units<'a> {
        fn split_units(s: &'a str) -> Vec<String> {
            let mut units: Vec<String> = vec![];

            let mut unit_start: usize = 0;
            let mut unit_end: usize = 0;

            let chars: Vec<char> = s.chars().collect();

            // How else to iterate over s.chars() without taking ownership?
            'outer: while unit_end < chars.len() {
                unit_start = unit_end;

                // Loop until we encounter a separator ('.' or '/')
                while unit_end <= chars.len() {
                    match prev_char {
                        '.' => {
                            units.push(chars[unit_start..unit_end]);
                            unit_end += 1;
                            continue 'outer;
                        },
                        '/' => {
                            units.push(chars[unit_start..(unit_end + 1)]);
                            unit_end += 1;
                            continue 'outer;
                        },
                        _ => {
                            unit_end += 1
                        }
                    }
                }
            }
            units
        }
    }

    impl<'a> convert::TryFrom<&'a str> for Units<'a> {
        type Error = ParseUnitsError;

        // from_str isn't liking the way these lifetimes are passed around.
        fn try_from(s: &'a str) -> Result<Self, Self::Error> {
            let units: Vec<Unit<'a>> = Units::split_units(s).iter()
                .map(|s: String| Unit::try_from(s).unwrap()).collect();

            match s {
                "" => Ok(Units::BASE),
                _ => Ok(Units::DIM(units)),
            }
        }
    }



    #[derive(Debug)]
    pub struct ParseUnitsError {}

    impl fmt::Display for ParseUnitsError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Units parser encountered a formatting error")
        }
    }

    impl Error for ParseUnitsError {}
}

mod quantity {
    use calculum::quantity::*;

    impl<'a, T> Quantity<'a, T> {
        pub fn new(value: T, s: &'a str) -> Quantity<'a, T> {
            Quantity {
                value,
                units: units::Units::try_from(s).unwrap()
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::quantity::Quantity;
    use super::units::Units;
    use super::units::Unit;

    #[test]
    fn it_parses_dimensionality_of_units() {
        assert_eq!(Units::try_from("").unwrap(), Units::BASE);
        assert_eq!(Units::try_from("m").unwrap(),
                   Units::DIM(vec![Unit::try_from("m").unwrap()]));
    }

    #[test]
    fn it_splits_on_unit_multiplication() {
        assert_eq!(Units::try_from("m.kg").unwrap(),
                   Units::DIM(vec![Unit::try_from("m").unwrap(),
                                   Unit::try_from("kg").unwrap()]));
    }

    #[test]
    fn it_splits_on_unit_division() {
        // Need to keep this info!
        assert_eq!(Units::try_from("m/kg").unwrap(),
                   Units::DIM(vec![Unit::try_from("m").unwrap(),
                                   Unit::try_from("kg").unwrap()]));
    }

}