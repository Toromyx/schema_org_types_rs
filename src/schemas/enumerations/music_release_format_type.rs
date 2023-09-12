/// Format of this release (the type of recording media used, i.e. compact disc, digital media, LP, etc.).
///
/// https://schema.org/MusicReleaseFormatType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MusicReleaseFormatType {
    /// CDFormat.
    ///
    /// https://schema.org/CDFormat
    CdFormat,
    /// CassetteFormat.
    ///
    /// https://schema.org/CassetteFormat
    CassetteFormat,
    /// DVDFormat.
    ///
    /// https://schema.org/DVDFormat
    DvdFormat,
    /// DigitalAudioTapeFormat.
    ///
    /// https://schema.org/DigitalAudioTapeFormat
    DigitalAudioTapeFormat,
    /// DigitalFormat.
    ///
    /// https://schema.org/DigitalFormat
    DigitalFormat,
    /// LaserDiscFormat.
    ///
    /// https://schema.org/LaserDiscFormat
    LaserDiscFormat,
    /// VinylFormat.
    ///
    /// https://schema.org/VinylFormat
    VinylFormat,
}
