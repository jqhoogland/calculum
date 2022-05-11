use crate::constants::metric::{UnitPrefix, Dimension, Unit, MetricDerivedUnit};

// -- Dimensions

impl Dimension<T> {
    pub fn to_base_units(&self) -> Vec<Unit::Base> { 
        use MetricBaseUnitAtom::*;
        use UnitPrefix::*;
        
        match self {
            // Simple
            Time => vec![Unit::new_bare(Second, 1i8)],
            Length => vec![Unit::new_bare(Meter, 1i8)],
            Mass => vec![Unit::new_bare(Mass, 1i8)],
            Charge => vec![Unit::new_bare(Coulomb, 1i8)],
            Temp => vec![Unit::new_bare(Kelvin, 1i8)],
            Amount => vec![Unit::new_bare(Mole, 1i8)],
            Luminosity => vec![Unit::new_bare(Candela, 1i8)],

            // Composite
            Force => vec![Unit::new(Kilo, Gram, 1i8), Unit::new_bare(Meter, 1i8), Unit::new_bare(Second, -2i8)],
        }
    }
}



// -- Units

pub trait UnitEq<T> {
    /// Up to a dimensionless scaling factor
    fn is_similar(&self, other: T) -> bool;

    /// Can be converted (i.e., has the same dimension)
    fn is_commensurable(&self, other: T) -> bool;
}

pub struct ReducedForm {
    amount: f64,
    units: Vec<MetricBaseUnit>,
}

pub trait ReducibleUnit {
    fn get_dimensions(&self) -> Vec<Dimension>;
    fn to_base_units(&self) -> Option<ReducedForm>;
}

// -- Implementations

impl ReducibleUnit for MetricBaseUnit {
    fn get_dimensions(&self) -> Vec<Dimension> {
        use MetricBaseUnitAtom::*;
        use Dimension::*;
        
        match self {            
            Meter => vec![Length],
            Second=> vec![Time],
            Gram => vec![Mass],
            Coulomb => vec![Charge],
            Kelvin => vec![Temp],
            Mole => vec![Amount],
            Candela => vec![Luminosity],
        }
    }
    
    fn to_base_units(&self) -> Option<ReducedForm> {
        None
    }
}

impl ReducibleUnit for MetricDerivedUnit {
    fn get_dimensions(&self) -> (f64, Vec<Dimension>) {
        use MetricDerivedUnit::*;
        use Dimension::{self, *};

        match self {
            Newton => (1.0, vec![Dimension::Force]),
            _ => (1.0, vec![Dimension::Other(self)])
        }
    }

    fn to_base_units(&self) -> Option<ReducedForm> {
        let mut mag = 1;

        self.get_dimensions()
            .iter()
            .flat_map(|m, d| {
                mag *= m;
                d.to_base_units()
            })
    }
}


/// Returns None if already a base unit
pub fn to_base_units(atom: &UnitAtom) -> ConversionResult {
    use UnitAtom::*;

    match atom {
        Newton => ConversionResult::Composite(
            vec![Unit::new(Prefix::Kilo, Gram, 1i8),
                Unit::new_bare(Meter, 1i8),
                Unit::new_bare(Second, -2i8)],
        ),
        // ucum::parse("kg.m/s2"),
        // Hertz => ucum::parse("/s"),
        // Pascal => ucum::parse("N/m2"),
        // Joule => ucum::parse("N/m2"),
        // Watt => ucum::parse("J/s"),
        // Ampere => ucum::parse("C/s"),
        // Volt => ucum::parse("J/C"),
        // Farad => ucum::parse("C/V"),
        // Ohm => ucum::parse("V/A"),
        // Siemens => ucum::parse("1/Ohm"),
        // Weber => ucum::parse("V.s"),
        // Tesla => ucum::parse("Wb/m2"),
        // Henry => ucum::parse("Wb/A"),
        // Lumen => ucum::parse("cd.sr"),
        // Lux => ucum::parse("lm/m2"),
        // Becquerel => ucum::parse("s-1"),
        // Gray => ucum::parse("J/kg"),
        // Sievert => ucum::parse("J/kg"),
        // GonGrade => ucum::parse("0.9deg"),
        // Degree => ucum::parse("2[pi].rad/360"),
        // MinuteAngle => ucum::parse("deg/60"),
        // SecondAngle => ucum::parse("'/60"),
        // Liter => ucum::parse("dm3"),
        // Are => ucum::parse("100m2"),
        // Minute => ucum::parse("60s"),
        // Hour => ucum::parse("60min"),
        // Day => ucum::parse("24h"),
        // YearTropical => ucum::parse("365.242_19d"),
        // YearMeanJulian => ucum::parse("365.25d"),
        // YearMeanGregorian => ucum::parse("365.242_5d"),
        // Year => ucum::parse("a_j"),
        // Week => ucum::parse("7d"),
        // MonthSynodal => ucum::parse("29.53059d"),
        // MonthMeanJulian => ucum::parse("a_j/12"),
        // MonthMeanGregorian => ucum::parse("a_g/12"),
        // Month => ucum::parse("mo_j"),
        // Tonne => ucum::parse("10^3.kg"),
        // Bar => ucum::parse("10^5.Pa"),
        // Amu => ucum::parse("1.660_540_2*10^-24.g"),
        // ElectronVolt => ucum::parse("[e].V"),
        // AstronomicUnit => ucum::parse("149_597.870_691.Mm"),
        // Parsec => ucum::parse("3.085_678*10^16m"),
        // DegreeCelsius => ucum::parse("cel(1 K)"),
        Other(_) => ConversionResult::None,
    }
}
