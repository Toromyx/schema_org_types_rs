use super::*;
/// <https://schema.org/loanRepaymentForm>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum LoanRepaymentFormProperty {
	#[cfg(any(
		any(
			feature = "repayment-specification-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	RepaymentSpecification(RepaymentSpecification),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for LoanRepaymentFormProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(
						feature = "repayment-specification-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				LoanRepaymentFormProperty::RepaymentSpecification(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for LoanRepaymentFormProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(
					feature = "repayment-specification-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<RepaymentSpecification as Deserialize>::deserialize(deserializer),
				LoanRepaymentFormProperty::RepaymentSpecification,
			) {
				return Ok(ok);
			}
			Err(de::Error::custom(
				"data did not match any variant of schema.org property loanRepaymentForm",
			))
		}
	}
}
