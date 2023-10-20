/// <https://schema.org/RsvpResponseType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RsvpResponseType {
	/// <https://schema.org/RsvpResponseMaybe>
	RsvpResponseMaybe,
	/// <https://schema.org/RsvpResponseNo>
	RsvpResponseNo,
	/// <https://schema.org/RsvpResponseYes>
	RsvpResponseYes,
}
