/// <https://schema.org/MediaManipulationRatingEnumeration>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MediaManipulationRatingEnumeration {
	/// <https://schema.org/DecontextualizedContent>
	DecontextualizedContent,
	/// <https://schema.org/EditedOrCroppedContent>
	EditedOrCroppedContent,
	/// <https://schema.org/OriginalMediaContent>
	OriginalMediaContent,
	/// <https://schema.org/SatireOrParodyContent>
	SatireOrParodyContent,
	/// <https://schema.org/StagedContent>
	StagedContent,
	/// <https://schema.org/TransformedContent>
	TransformedContent,
}
