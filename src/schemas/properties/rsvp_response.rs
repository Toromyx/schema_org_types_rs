use super::*;
/// The response (yes, no, maybe) to the RSVP.
///
/// https://schema.org/rsvpResponse
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
#[cfg_attr(any(feature = "serde", doc), serde(untagged))]
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
