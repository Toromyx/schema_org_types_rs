use super::*;
/// <https://schema.org/Float>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Float(pub crate::number_types::Number);
impl std::ops::Deref for Float {
	type Target = crate::number_types::Number;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
