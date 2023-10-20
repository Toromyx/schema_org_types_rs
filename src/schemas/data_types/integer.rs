use super::*;
/// <https://schema.org/Integer>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Integer(pub crate::number_types::Integer);
impl std::ops::Deref for Integer {
	type Target = crate::number_types::Integer;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
