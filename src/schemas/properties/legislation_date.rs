use super::*;
/// The date of adoption or signature of the legislation. This is the date at which the text is officially aknowledged to be a legislation, even though it might not even be published or in force.
///
/// <https://schema.org/legislationDate>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LegislationDateProperty {
    #[cfg(any(any(feature = "date-schema", feature = "general-schema-section"), doc))]
    Date(Date),
}
