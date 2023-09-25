use super::*;
/// Data type: PronounceableText.
///
/// https://schema.org/PronounceableText
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct PronounceableText(pub String);
impl std::ops::Deref for PronounceableText {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
