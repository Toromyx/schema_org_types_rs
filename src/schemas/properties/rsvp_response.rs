use super::*;
/// <https://schema.org/rsvpResponse>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RsvpResponseProperty {
	#[cfg(any(
		any(
			feature = "rsvp-response-type-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	RsvpResponseType(RsvpResponseType),
}
