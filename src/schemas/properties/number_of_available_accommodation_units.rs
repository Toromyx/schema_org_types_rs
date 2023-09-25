use super::*;
/// Indicates the number of available accommodation units in an [[ApartmentComplex]], or the number of accommodation units for a specific [[FloorPlan]] (within its specific [[ApartmentComplex]]). See also [[numberOfAccommodationUnits]].
///
/// <https://schema.org/numberOfAvailableAccommodationUnits>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum NumberOfAvailableAccommodationUnitsProperty {
    #[cfg(any(
        any(
            feature = "quantitative-value-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    QuantitativeValue(QuantitativeValue),
}
