use super::*;
/// Molecular function performed by this BioChemEntity; please use PropertyValue if you want to include any evidence.
///
/// <https://schema.org/hasMolecularFunction>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum HasMolecularFunctionProperty {
    #[cfg(any(
        any(feature = "defined-term-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(
        any(feature = "property-value-schema", feature = "general-schema-section"),
        doc
    ))]
    PropertyValue(PropertyValue),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
