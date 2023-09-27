/// <https://schema.org/BookFormatType>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum BookFormatType {
    /// <https://schema.org/AudiobookFormat>
    AudiobookFormat,
    /// <https://schema.org/EBook>
    EBook,
    /// <https://schema.org/GraphicNovel>
    GraphicNovel,
    /// <https://schema.org/Hardcover>
    Hardcover,
    /// <https://schema.org/Paperback>
    Paperback,
}
