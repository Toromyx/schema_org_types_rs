use super::*;
/// Data type: URL.
///
/// https://schema.org/URL
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Url(pub String);
impl std::ops::Deref for Url {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
