/// <https://schema.org/MediaManipulationRatingEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
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
