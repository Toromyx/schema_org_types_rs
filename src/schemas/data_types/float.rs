use super::*;
/// <https://schema.org/Float>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Float(pub f64);
impl std::ops::Deref for Float {
    type Target = f64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
