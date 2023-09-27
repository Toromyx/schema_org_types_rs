use super::*;
/// <https://schema.org/typeOfBed>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TypeOfBedProperty {
    #[cfg(any(
        any(feature = "bed-type-schema", feature = "general-schema-section"),
        doc
    ))]
    BedType(BedType),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
