use super::*;
/// The type of bed to which the BedDetail refers, i.e. the type of bed available in the quantity indicated by quantity.
///
/// https://schema.org/typeOfBed
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TypeOfBedProperty {
    #[cfg(any(feature = "bed-type-schema", feature = "general-schema-section"))]
    BedType(BedType),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
