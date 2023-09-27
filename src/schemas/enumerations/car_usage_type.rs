/// <https://schema.org/CarUsageType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum CarUsageType {
    /// <https://schema.org/DrivingSchoolVehicleUsage>
    DrivingSchoolVehicleUsage,
    /// <https://schema.org/RentalVehicleUsage>
    RentalVehicleUsage,
    /// <https://schema.org/TaxiVehicleUsage>
    TaxiVehicleUsage,
}
