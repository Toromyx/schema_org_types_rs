use super::*;
/// <https://schema.org/URL>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Url(pub String);
impl std::ops::Deref for Url {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
