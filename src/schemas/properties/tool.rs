use super::*;
/// <https://schema.org/tool>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ToolProperty {
    #[cfg(any(
        any(feature = "how-to-tool-schema", feature = "general-schema-section"),
        doc
    ))]
    HowToTool(HowToTool),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
