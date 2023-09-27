use super::*;
/// <https://schema.org/step>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
