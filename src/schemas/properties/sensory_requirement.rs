use super::*;
/// A description of any sensory requirements and levels necessary to function on the job, including hearing and vision. Defined terms such as those in O*net may be used, but note that there is no way to specify the level of ability as well as its nature when using a defined term.
///
/// <https://schema.org/sensoryRequirement>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum SensoryRequirementProperty {
    #[cfg(any(
        any(feature = "defined-term-schema", feature = "pending-schema-section"),
        doc
    ))]
    DefinedTerm(DefinedTerm),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
    #[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
    Url(Url),
}
