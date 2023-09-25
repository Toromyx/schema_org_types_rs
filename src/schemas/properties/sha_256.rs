use super::*;
/// The [SHA-2](https://en.wikipedia.org/wiki/SHA-2) SHA256 hash of the content of the item. For example, a zero-length input has value 'e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855'
///
/// <https://schema.org/sha256>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum Sha256Property {
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
