use super::*;
/// Runtime platform or script interpreter dependencies (example: Java v1, Python 2.3, .NET Framework 3.0).
///
/// https://schema.org/runtimePlatform
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RuntimePlatformProperty {
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
