use super::*;
/// Days of the week when the merchant typically operates, indicated via opening hours markup.
///
/// <https://schema.org/businessDays>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BusinessDaysProperty {
    #[cfg(any(
        any(
            feature = "opening-hours-specification-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    OpeningHoursSpecification(OpeningHoursSpecification),
}
