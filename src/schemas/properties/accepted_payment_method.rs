use super::*;
/// <https://schema.org/acceptedPaymentMethod>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum AcceptedPaymentMethodProperty {
	#[cfg(any(
		any(feature = "loan-or-credit-schema", feature = "general-schema-section"),
		doc
	))]
	LoanOrCredit(LoanOrCredit),
	#[cfg(any(
		any(feature = "payment-method-schema", feature = "general-schema-section"),
		doc
	))]
	PaymentMethod(PaymentMethod),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for AcceptedPaymentMethodProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "loan-or-credit-schema", feature = "general-schema-section"),
					doc
				))]
				AcceptedPaymentMethodProperty::LoanOrCredit(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "payment-method-schema", feature = "general-schema-section"),
					doc
				))]
				AcceptedPaymentMethodProperty::PaymentMethod(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for AcceptedPaymentMethodProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "loan-or-credit-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<LoanOrCredit as Deserialize>::deserialize(deserializer),
				AcceptedPaymentMethodProperty::LoanOrCredit,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "payment-method-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<PaymentMethod as Deserialize>::deserialize(deserializer),
				AcceptedPaymentMethodProperty::PaymentMethod,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property acceptedPaymentMethod",
			))
		}
	}
}
