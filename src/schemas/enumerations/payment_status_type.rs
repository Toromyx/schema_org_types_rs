/// <https://schema.org/PaymentStatusType>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PaymentStatusType {
	/// <https://schema.org/PaymentAutomaticallyApplied>
	PaymentAutomaticallyApplied,
	/// <https://schema.org/PaymentComplete>
	PaymentComplete,
	/// <https://schema.org/PaymentDeclined>
	PaymentDeclined,
	/// <https://schema.org/PaymentDue>
	PaymentDue,
	/// <https://schema.org/PaymentPastDue>
	PaymentPastDue,
}
