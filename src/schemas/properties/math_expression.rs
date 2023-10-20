use super::*;
/// <https://schema.org/mathExpression>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
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
