use super::*;
/// <https://schema.org/Number>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Number(pub f64);
impl std::ops::Deref for Number {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
