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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum MetricBaseUnitAtom {
    Meter,
    Second,
    Gram,
    Radian,
    Kelvin,
    Coulomb,
    Candela,
    Mole,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum DimensionlessUnitAtom {
    Pi,
    TenTo(i8),
    Percent,
    PPTH,
    PPM,
    PPB,
    PPTR,
}

#[repr(i8)]
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum MetricDerivedUnitAtom {
    // SI
    Newton,
    // Steradian,
    // Hertz,
    // Pascal,
    // Joule,
    // Watt,
    // Ampere,
    // Volt,
    // Farad,
    // Ohm,
    // Siemens,
    // Weber,
    // DegreeCelsius,
    // Tesla,
    // Henry,
    // Lumen,
    // Lux,
    // Becquerel,
    // Gray,
    // Sievert,
    //
    // // ISO 1000, ISO 2955, ANSI X3.50
    // GonGrade,
    // Degree,
    // MinuteAngle,
    // SecondAngle,
    // Liter,
    // Are,
    // Minute,
    // Hour,
    // Day,
    // YearTropical,
    // YearMeanJulian,
    // YearMeanGregorian,
    // Year,
    // Week,
    // MonthSynodal,
    // MonthMeanJulian,
    // MonthMeanGregorian,
    // Month,
    // Tonne,
    // Bar,
    // Amu,
    // ElectronVolt,
    // AstronomicUnit,
    // Parsec,
}
