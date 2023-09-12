/// The publication format of the book.
///
/// https://schema.org/BookFormatType
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BookFormatType {
    /// Book format: Audiobook. This is an enumerated value for use with the bookFormat property. There is also a type 'Audiobook' in the bib extension which includes Audiobook specific properties.
    ///
    /// https://schema.org/AudiobookFormat
    AudiobookFormat,
    /// Book format: Ebook.
    ///
    /// https://schema.org/EBook
    EBook,
    /// Book format: GraphicNovel. May represent a bound collection of ComicIssue instances.
    ///
    /// https://schema.org/GraphicNovel
    GraphicNovel,
    /// Book format: Hardcover.
    ///
    /// https://schema.org/Hardcover
    Hardcover,
    /// Book format: Paperback.
    ///
    /// https://schema.org/Paperback
    Paperback,
}
