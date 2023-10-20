use super::*;
/// <https://schema.org/Text>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Text(pub String);
impl std::ops::Deref for Text {
	type Target = String;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
