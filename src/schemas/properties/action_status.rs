use super::*;
/// <https://schema.org/actionStatus>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum ActionStatusProperty {
    #[cfg(any(
        any(
            feature = "action-status-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    ActionStatusType(ActionStatusType),
}
