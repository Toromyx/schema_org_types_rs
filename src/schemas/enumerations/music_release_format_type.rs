/// <https://schema.org/MusicReleaseFormatType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
