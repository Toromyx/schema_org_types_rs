use super::*;
/// <https://schema.org/step>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum StepProperty {
    #[cfg(any(
        any(feature = "creative-work-schema", feature = "general-schema-section"),
        doc
    ))]
    CreativeWork(CreativeWork),
    #[cfg(any(
        any(feature = "how-to-section-schema", feature = "general-schema-section"),
        doc
    ))]
    HowToSection(HowToSection),
    #[cfg(any(
        any(feature = "how-to-step-schema", feature = "general-schema-section"),
        doc
    ))]
    HowToStep(HowToStep),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
