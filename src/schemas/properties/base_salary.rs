use super::*;
/// <https://schema.org/baseSalary>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum BaseSalaryProperty {
	/// <https://schema.org/MonetaryAmount>
	MonetaryAmount(MonetaryAmount),
	/// <https://schema.org/PriceSpecification>
	PriceSpecification(PriceSpecification),
	/// <https://schema.org/Number>
	Number(Number),
	#[cfg(any(all(feature = "fallible", feature = "serde"), doc))]
	SerdeFail(crate::fallible::FailValue),
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for BaseSalaryProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				BaseSalaryProperty::MonetaryAmount(ref inner) => inner.serialize(serializer),
				BaseSalaryProperty::PriceSpecification(ref inner) => inner.serialize(serializer),
				BaseSalaryProperty::Number(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				BaseSalaryProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for BaseSalaryProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<MonetaryAmount as Deserialize>::deserialize(deserializer),
				BaseSalaryProperty::MonetaryAmount,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<PriceSpecification as Deserialize>::deserialize(deserializer),
				BaseSalaryProperty::PriceSpecification,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Number as Deserialize>::deserialize(deserializer),
				BaseSalaryProperty::Number,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				BaseSalaryProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property baseSalary or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property baseSalary";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
