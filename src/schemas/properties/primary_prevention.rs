use super::*;
/// A preventative therapy used to prevent an initial occurrence of the medical condition, such as vaccination.
///
/// <https://schema.org/primaryPrevention>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum PrimaryPreventionProperty {
    #[cfg(any(
        any(
            feature = "medical-therapy-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    MedicalTherapy(MedicalTherapy),
}
