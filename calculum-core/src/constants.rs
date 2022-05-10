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
    // Base
    TIME,
    LENGTH,
    MASS,
    CHARGE,
    TEMP,
    AMOUNT,
    LUMINOUS_INTENSITY,

    // "Dimensionless"
    PLANE_ANGLE,
    SOLID_ANGLE,

    // "Derived"

    // From LENGTH
    AREA,
    VOLUME,
    LINEIC_NUM,

    // From TIME
    FREQUENCY,

    // MECHANICS
    VELOCITY,
    ACCELERATION,
    FORCE,
    PRESSURE,
    VISCOSITY_DYNAMIC,
    VISCOSITY_KINEMATIC,
    ENERGY,
    ACTION,
    POWER,

    // ELECTRICITY & MAGNETISM
    CURRENT,
    POTENTIAL,
    CAPACITANCE,
    RESISTANCE,
    CONDUCTANCE,
    FLUX_MAGNETIC,
    INDUCTANCE,
    PERMITTIVITY_ELECTRIC,
    PERMITTIVITY_MAGNETIC,

    FLUX_LUMINOUS,
    ILLUMINANCE,
    BRIGHTNESS,

    // RADIOACTIVITY
    RADIOACTIVITY,
    ION_DOSE,
    ENERGY_DOSE,
    DOSE_EQUIVALENT,


    OTHER(String)
}

#[derive(Deubg)]
pub enum UnitAtom {
    // Base Units
    METER,
    SECOND,
    GRAM,
    RADIAN,
    KELVIN,
    COULOMB,
    CANDELA,

    // Dimensionless
    PI,
    TEN_TO(i8),
    PERCENT,
    PPTH,
    PPM,
    PPB,
    PPTR,

    // SI
    MOLE,
    STERADIAN,
    HERTZ,
    NEWTON,
    PASCAL,
    JOULE,
    WATT,
    AMPERE,
    VOLT,
    FARAD,
    OHM,
    SIEMENS,
    WEBER,
    DEGREE_CELSIUS,
    TESLA,
    HENRY,
    LUMEN,
    LUX,
    BECQUEREL,
    GRAY,
    SIEVERT,

    // ISO 1000, ISO 2955, ANSI X3.50
    GON_GRADE,
    DEGREE,
    MINUTE_ANGLE,
    SECOND_ANGLE,
    LITER,
    ARE,
    MINUTE,
    HOUR,
    DAY,
    YEAR_TROPICAL,
    YEAR_MEAN_JULIAN,
    YEAR_MEAN_GREGORIAN,
    YEAR,
    WEEK,
    MONTH_SYNODAL,
    MONTH_MEAN_JULIAN,
    MONTH_MEAN_GREGORIAN,
    MONTH,
    TONNE,
    BAR,
    AMU,
    ELECTRON_VOLT,
    ASTRONOMIC_UNIT,
    PARSEC

    // CGS
    // Customary
}

impl UnitAtom {
    pub fn to_dimension(&self) -> Dimension {
        use Dimension::*;
        use UnitAtom::*;

        match self {
            SECOND | HOUR | DAY | WEEK => TIME,
            METER  => LENGTH,
            COULOMB => CHARGE,
            KELVIN => TEMP,
            MOLE => AMOUNT,
            CANDELA => LUMINOSITY,
            _ => OTHER(self.unit.to_string())
        }
    }
}

