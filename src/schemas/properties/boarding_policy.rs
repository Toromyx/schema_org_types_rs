use super::*;
/// <https://schema.org/boardingPolicy>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum BoardingPolicyProperty {
    #[cfg(any(
        any(
            feature = "boarding-policy-type-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    BoardingPolicyType(BoardingPolicyType),
}
