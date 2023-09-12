use super::*;
/// The response (yes, no, maybe) to the RSVP.
///
/// https://schema.org/rsvpResponse
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum RsvpResponseProperty {
    #[cfg(any(
        feature = "rsvp-response-type-schema",
        feature = "general-schema-section"
    ))]
    RsvpResponseType(RsvpResponseType),
}
