use super::*;
/// The date of the first registration of the vehicle with the respective public authorities.
///
/// <https://schema.org/dateVehicleFirstRegistered>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum DateVehicleFirstRegisteredProperty {
    #[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
    Date(Date),
}
