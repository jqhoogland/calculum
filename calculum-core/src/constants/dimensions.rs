#[derive(Debug, PartialEq, PartialOrd)]
pub enum Dimension<T> {
    // Base
    Time,
    Length,
    Mass,
    Charge,
    Temp,
    Amount,
    Luminosity,

    // // "Dimensionless"
    // PlaneAngle,
    // SolidAngle,
    //
    // // "Derived"
    //
    // // From Length
    // Area,
    // Volume,
    // LineicNum,
    //
    // // From Time
    // Frequency,
    //
    // // Mechanics
    // Velocity,
    // Acceleration,
    Force,
    // Pressure,
    // ViscosityDynamic,
    // ViscosityKinematic,
    // Energy,
    // Action,
    // Power,
    //
    // // Electricity & Magnetism
    // Current,
    // Potential,
    // Capacitance,
    // Resistance,
    // Conductance,
    // FluxMagnetic,
    // Inductance,
    // PermittivityElectric,
    // PermittivityMagnetic,
    //
    // FluxLuminous,
    // Illuminance,
    // Brightness,
    //
    // // Radioactivity
    // Radioactivity,
    // IonDose,
    // EnergyDose,
    // DoseEquivalent,

    Other(T)
}