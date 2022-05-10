#[repr(i8)]
#[derive(Debug, PartialEq, Clone)]
pub enum UnitPrefix {
    Yotta = 24,
    Zetta = 21,
    Exa = 18,
    Peta = 15,
    Tera = 12,
    Giga = 9,
    Mega = 6,
    Kilo = 3,
    Hecto = 2,
    Deca = 1,
    None = 0,
    Deci = -1,
    Centi = -2,
    Milli = -3,
    Micro = -6,
    Nano = -9,
    Pico = -12,
    Femto = -15,
    Atto = -18,
    Zepto = -21,
    Yocto = -24,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Dimension {
    // Base
    Time,
    Length,
    Mass,
    Charge,
    Temp,
    Amount,
    Luminosity,

    // "Dimensionless"
    PlaneAngle,
    SolidAngle,

    // "Derived"

    // From Length
    Area,
    Volume,
    LineicNum,

    // From Time
    Frequency,

    // Mechanics
    Velocity,
    Acceleration,
    Force,
    Pressure,
    ViscosityDynamic,
    ViscosityKinematic,
    Energy,
    Action,
    Power,

    // Electricity & Magnetism
    Current,
    Potential,
    Capacitance,
    Resistance,
    Conductance,
    FluxMagnetic,
    Inductance,
    PermittivityElectric,
    PermittivityMagnetic,

    FluxLuminous,
    Illuminance,
    Brightness,

    // Radioactivity
    Radioactivity,
    IonDose,
    EnergyDose,
    DoseEquivalent,


    Other(UnitAtom)
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum UnitAtom {
    // Base Units
    Meter,
    Second,
    Gram,
    Radian,
    Kelvin,
    Coulomb,
    Candela,

    // Dimensionless
    Pi,
    TenTo(i8),
    Percent,
    PPTH,
    PPM,
    PPB,
    PPTR,

    // SI
    Mole,
    Steradian,
    Hertz,
    Newton,
    Pascal,
    Joule,
    Watt,
    Ampere,
    Volt,
    Farad,
    Ohm,
    Siemens,
    Weber,
    DegreeCelsius,
    Tesla,
    Henry,
    Lumen,
    Lux,
    Becquerel,
    Gray,
    Sievert,

    // ISO 1000, ISO 2955, ANSI X3.50
    GonGrade,
    Degree,
    MinuteAngle,
    SecondAngle,
    Liter,
    Are,
    Minute,
    Hour,
    Day,
    YearTropical,
    YearMeanJulian,
    YearMeanGregorian,
    Year,
    Week,
    MonthSynodal,
    MonthMeanJulian,
    MonthMeanGregorian,
    Month,
    Tonne,
    Bar,
    Amu,
    ElectronVolt,
    AstronomicUnit,
    Parsec,

    // CGS
    // Customary

    Other(String)
}

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
