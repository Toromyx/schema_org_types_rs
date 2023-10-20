use super::*;
/// <https://schema.org/Number>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Number(pub crate::number_types::Number);
impl std::ops::Deref for Number {
	type Target = crate::number_types::Number;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
