/// <https://schema.org/BookFormatType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
