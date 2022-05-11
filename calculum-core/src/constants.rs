use crate::constants::dimensions;
use crate::constants::metric;
use crate::constants::customary;


impl UnitAtom {
    pub fn to_dimension(&self) -> Dimension {
        use Dimension;
        use UnitAtom::*;

        match self {
            Second | Hour | Day | Week => Dimension::Time,
            Meter  => Dimension::Length,
            Coulomb => Dimension::Charge,
            Kelvin => Dimension::Temp,
            Mole => Dimension::Amount,
            Candela => Dimension::Luminosity,
            _ => Dimension::Other(self.clone()) // .to_string() ?
        }
    }
}

impl From<String> for UnitAtom {
    fn from(s: String) -> Self {
        UnitAtom::Other(s)
    }
}
