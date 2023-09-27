use super::*;
/// <https://schema.org/Integer>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct Integer(pub i64);
impl std::ops::Deref for Integer {
    type Target = i64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
