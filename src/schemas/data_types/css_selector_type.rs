use super::*;
/// Text representing a CSS selector.
///
/// https://schema.org/CssSelectorType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct CssSelectorType(pub String);
impl std::ops::Deref for CssSelectorType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
