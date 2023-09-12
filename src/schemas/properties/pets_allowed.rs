use super::*;
/// Indicates whether pets are allowed to enter the accommodation or lodging business. More detailed information can be put in a text value.
///
/// https://schema.org/petsAllowed
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum PetsAllowedProperty {
    #[cfg(any(feature = "boolean-schema", feature = "general-schema-section"))]
    Boolean(Boolean),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
