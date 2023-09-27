use super::*;
/// <https://schema.org/XPathType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct XPathType(pub String);
impl std::ops::Deref for XPathType {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
