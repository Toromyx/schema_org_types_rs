use super::*;
/// A sub property of instrument. An object used (but not consumed) when performing instructions or a direction.
///
/// https://schema.org/tool
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum ToolProperty {
    #[cfg(any(feature = "how-to-tool-schema", feature = "general-schema-section"))]
    HowToTool(HowToTool),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
