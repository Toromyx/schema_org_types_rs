use super::*;
/// The type of bed or beds included in the accommodation. For the single case of just one bed of a certain type, you use bed directly with a text.
/// If you want to indicate the quantity of a certain kind of bed, use an instance of BedDetails. For more detailed information, use the amenityFeature property.
///
/// https://schema.org/bed
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum BedProperty {
    #[cfg(any(feature = "bed-details-schema", feature = "general-schema-section"))]
    BedDetails(BedDetails),
    #[cfg(any(feature = "bed-type-schema", feature = "general-schema-section"))]
    BedType(BedType),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
