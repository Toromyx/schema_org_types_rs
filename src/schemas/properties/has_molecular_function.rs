use super::*;
/// Molecular function performed by this BioChemEntity; please use PropertyValue if you want to include any evidence.
///
/// https://schema.org/hasMolecularFunction
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasMolecularFunctionProperty {
    #[cfg(any(feature = "defined-term-schema", feature = "pending-schema-section"))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(feature = "property-value-schema", feature = "general-schema-section"))]
    PropertyValue(PropertyValue),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
