/// <https://schema.org/MusicReleaseFormatType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MusicReleaseFormatType {
    /// <https://schema.org/CDFormat>
    CdFormat,
    /// <https://schema.org/CassetteFormat>
    CassetteFormat,
    /// <https://schema.org/DVDFormat>
    DvdFormat,
    /// <https://schema.org/DigitalAudioTapeFormat>
    DigitalAudioTapeFormat,
    /// <https://schema.org/DigitalFormat>
    DigitalFormat,
    /// <https://schema.org/LaserDiscFormat>
    LaserDiscFormat,
    /// <https://schema.org/VinylFormat>
    VinylFormat,
}
