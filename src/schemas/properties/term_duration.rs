use super::*;
/// The amount of time in a term as defined by the institution. A term is a length of time where students take one or more classes. Semesters and quarters are common units for term.
///
/// https://schema.org/termDuration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum TermDurationProperty {
    #[cfg(any(feature = "duration-schema", feature = "general-schema-section"))]
    Duration(Duration),
}
