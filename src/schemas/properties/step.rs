use super::*;
/// A single step item (as HowToStep, text, document, video, etc.) or a HowToSection.
///
/// https://schema.org/step
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum StepProperty {
    #[cfg(any(feature = "creative-work-schema", feature = "general-schema-section"))]
    CreativeWork(CreativeWork),
    #[cfg(any(feature = "how-to-section-schema", feature = "general-schema-section"))]
    HowToSection(HowToSection),
    #[cfg(any(feature = "how-to-step-schema", feature = "general-schema-section"))]
    HowToStep(HowToStep),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
