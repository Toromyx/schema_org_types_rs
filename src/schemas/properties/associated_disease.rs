use super::*;
/// <https://schema.org/associatedDisease>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum AssociatedDiseaseProperty {
    #[cfg(any(
        any(
            feature = "medical-condition-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalCondition(MedicalCondition),
    #[cfg(any(
        any(feature = "property-value-schema", feature = "general-schema-section"),
        doc
    ))]
    PropertyValue(PropertyValue),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
