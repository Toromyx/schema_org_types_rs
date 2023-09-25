use super::*;
/// The type of bed or beds included in the accommodation. For the single case of just one bed of a certain type, you use bed directly with a text.
/// If you want to indicate the quantity of a certain kind of bed, use an instance of BedDetails. For more detailed information, use the amenityFeature property.
///
/// <https://schema.org/bed>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BedProperty {
    #[cfg(any(
        any(feature = "bed-details-schema", feature = "general-schema-section"),
        doc
    ))]
    BedDetails(BedDetails),
    #[cfg(any(
        any(feature = "bed-type-schema", feature = "general-schema-section"),
        doc
    ))]
    BedType(BedType),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
