/// A value indicating a special usage of a car, e.g. commercial rental, driving school, or as a taxi.
///
/// https://schema.org/CarUsageType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CarUsageType {
    /// Indicates the usage of the vehicle for driving school.
    ///
    /// https://schema.org/DrivingSchoolVehicleUsage
    DrivingSchoolVehicleUsage,
    /// Indicates the usage of the vehicle as a rental car.
    ///
    /// https://schema.org/RentalVehicleUsage
    RentalVehicleUsage,
    /// Indicates the usage of the car as a taxi.
    ///
    /// https://schema.org/TaxiVehicleUsage
    TaxiVehicleUsage,
}
