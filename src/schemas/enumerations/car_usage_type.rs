/// <https://schema.org/CarUsageType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CarUsageType {
    /// <https://schema.org/DrivingSchoolVehicleUsage>
    DrivingSchoolVehicleUsage,
    /// <https://schema.org/RentalVehicleUsage>
    RentalVehicleUsage,
    /// <https://schema.org/TaxiVehicleUsage>
    TaxiVehicleUsage,
}
