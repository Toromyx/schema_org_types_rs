use super::*;
/// The type of the legislation. Examples of values are "law", "act", "directive", "decree", "regulation", "statutory instrument", "loi organique", "règlement grand-ducal", etc., depending on the country.
///
/// <https://schema.org/legislationType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum LegislationTypeProperty {
    #[cfg(any(
        any(feature = "category-code-schema", feature = "pending-schema-section"),
        doc
    ))]
    CategoryCode(CategoryCode),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
