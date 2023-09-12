use super::*;
/// Disease associated to this BioChemEntity. Such disease can be a MedicalCondition or a URL. If you want to add an evidence supporting the association, please use PropertyValue.
///
/// https://schema.org/associatedDisease
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum AssociatedDiseaseProperty {
    #[cfg(any(
        feature = "medical-condition-schema",
        feature = "health-lifesci-schema-section"
    ))]
    MedicalCondition(MedicalCondition),
    #[cfg(any(feature = "property-value-schema", feature = "general-schema-section"))]
    PropertyValue(PropertyValue),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
