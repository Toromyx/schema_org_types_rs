use super::*;
/// A mathematical expression (e.g. 'x^2-3x=0') that may be solved for a specific variable, simplified, or transformed. This can take many formats, e.g. LaTeX, Ascii-Math, or math as you would write with a keyboard.
///
/// https://schema.org/mathExpression
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum MathExpressionProperty {
    #[cfg(any(
        feature = "solve-math-action-schema",
        feature = "pending-schema-section"
    ))]
    SolveMathAction(SolveMathAction),
    #[cfg(any(feature = "text-schema", feature = "general-schema-section"))]
    Text(Text),
}
