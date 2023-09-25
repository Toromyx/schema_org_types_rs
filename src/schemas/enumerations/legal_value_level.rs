/// A list of possible levels for the legal validity of a legislation.
///
/// https://schema.org/LegalValueLevel
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum LegalValueLevel {
    /// Indicates that the publisher gives some special status to the publication of the document. ("The Queens Printer" version of a UK Act of Parliament, or the PDF version of a Directive published by the EU Office of Publications.) Something "Authoritative" is considered to be also [[OfficialLegalValue]].
    ///
    /// https://schema.org/AuthoritativeLegalValue
    AuthoritativeLegalValue,
    /// Indicates a document for which the text is conclusively what the law says and is legally binding. (E.g. the digitally signed version of an Official Journal.)
    /// Something "Definitive" is considered to be also [[AuthoritativeLegalValue]].
    ///
    /// https://schema.org/DefinitiveLegalValue
    DefinitiveLegalValue,
    /// All the documents published by an official publisher should have at least the legal value level "OfficialLegalValue". This indicates that the document was published by an organisation with the public task of making it available (e.g. a consolidated version of an EU directive published by the EU Office of Publications).
    ///
    /// https://schema.org/OfficialLegalValue
    OfficialLegalValue,
    /// Indicates that a document has no particular or special standing (e.g. a republication of a law by a private publisher).
    ///
    /// https://schema.org/UnofficialLegalValue
    UnofficialLegalValue,
}
