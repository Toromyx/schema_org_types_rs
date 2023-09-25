use super::*;
/// The type of bed to which the BedDetail refers, i.e. the type of bed available in the quantity indicated by quantity.
///
/// <https://schema.org/typeOfBed>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum TypeOfBedProperty {
    #[cfg(any(
        any(feature = "bed-type-schema", feature = "general-schema-section"),
        doc
    ))]
    BedType(BedType),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
