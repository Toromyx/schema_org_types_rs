use super::*;
/// <https://schema.org/mathExpression>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
pub enum MathExpressionProperty {
    #[cfg(any(
        any(
            feature = "solve-math-action-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    SolveMathAction(SolveMathAction),
    #[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
    Text(Text),
}
