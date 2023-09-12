use super::*;
/// Either the actual menu as a structured representation, as text, or a URL of the menu.
///
/// https://schema.org/hasMenu
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum HasMenuProperty {
    #[cfg(any(feature = "menu-schema", feature = "general-schema-section"))]
    Menu(Menu),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
    #[cfg(any(feature = "url-schema", feature = "general-schema-section"))]
    Url(Url),
}
