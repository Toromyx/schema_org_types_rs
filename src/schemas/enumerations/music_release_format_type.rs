/// Format of this release (the type of recording media used, i.e. compact disc, digital media, LP, etc.).
///
/// <https://schema.org/MusicReleaseFormatType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum MusicReleaseFormatType {
    /// CDFormat.
    ///
    /// <https://schema.org/CDFormat>
    CdFormat,
    /// CassetteFormat.
    ///
    /// <https://schema.org/CassetteFormat>
    CassetteFormat,
    /// DVDFormat.
    ///
    /// <https://schema.org/DVDFormat>
    DvdFormat,
    /// DigitalAudioTapeFormat.
    ///
    /// <https://schema.org/DigitalAudioTapeFormat>
    DigitalAudioTapeFormat,
    /// DigitalFormat.
    ///
    /// <https://schema.org/DigitalFormat>
    DigitalFormat,
    /// LaserDiscFormat.
    ///
    /// <https://schema.org/LaserDiscFormat>
    LaserDiscFormat,
    /// VinylFormat.
    ///
    /// <https://schema.org/VinylFormat>
    VinylFormat,
}
