use super::*;
/// URL at which the app may be installed, if different from the URL of the item.
///
/// <https://schema.org/installUrl>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum InstallUrlProperty {
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
