use super::*;
/// Boolean: True or False.
///
/// https://schema.org/Boolean
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Boolean(pub bool);
impl std::ops::Deref for Boolean {
    type Target = bool;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
