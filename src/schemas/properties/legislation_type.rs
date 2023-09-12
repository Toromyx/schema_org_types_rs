use super::*;
/// The type of the legislation. Examples of values are "law", "act", "directive", "decree", "regulation", "statutory instrument", "loi organique", "règlement grand-ducal", etc., depending on the country.
///
/// https://schema.org/legislationType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum LegislationTypeProperty {
    #[cfg(any(feature = "category-code-schema", feature = "pending-schema-section"))]
    CategoryCode(CategoryCode),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
