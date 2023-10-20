/// <https://schema.org/DayOfWeek>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DayOfWeek {
	/// <https://schema.org/Friday>
	Friday,
	/// <https://schema.org/Monday>
	Monday,
	/// <https://schema.org/PublicHolidays>
	PublicHolidays,
	/// <https://schema.org/Saturday>
	Saturday,
	/// <https://schema.org/Sunday>
	Sunday,
	/// <https://schema.org/Thursday>
	Thursday,
	/// <https://schema.org/Tuesday>
	Tuesday,
	/// <https://schema.org/Wednesday>
	Wednesday,
}
