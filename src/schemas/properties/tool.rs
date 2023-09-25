use super::*;
/// A sub property of instrument. An object used (but not consumed) when performing instructions or a direction.
///
/// <https://schema.org/tool>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ToolProperty {
    #[cfg(any(
        any(feature = "how-to-tool-schema", feature = "general-schema-section"),
        doc
    ))]
    HowToTool(HowToTool),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
