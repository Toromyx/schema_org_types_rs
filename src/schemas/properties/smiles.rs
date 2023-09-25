use super::*;
/// A specification in form of a line notation for describing the structure of chemical species using short ASCII strings.  Double bond stereochemistry \ indicators may need to be escaped in the string in formats where the backslash is an escape character.
///
/// <https://schema.org/smiles>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SmilesProperty {
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
