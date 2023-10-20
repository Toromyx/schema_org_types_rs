use super::*;
/// <https://schema.org/Time>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Time(
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::DisplayFromStr>")
	)]
	pub crate::date_types::Time,
);
impl std::ops::Deref for Time {
	type Target = crate::date_types::Time;
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
