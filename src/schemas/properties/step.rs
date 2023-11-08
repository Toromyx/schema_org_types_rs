use super::*;
/// <https://schema.org/step>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum StepProperty {
	#[cfg(any(
		any(feature = "creative-work-schema", feature = "general-schema-section"),
		doc
	))]
	CreativeWork(CreativeWork),
	#[cfg(any(
		any(feature = "how-to-section-schema", feature = "general-schema-section"),
		doc
	))]
	HowToSection(HowToSection),
	#[cfg(any(
		any(feature = "how-to-step-schema", feature = "general-schema-section"),
		doc
	))]
	HowToStep(HowToStep),
	#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
	Text(Text),
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
	impl Serialize for StepProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "creative-work-schema", feature = "general-schema-section"),
					doc
				))]
				StepProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "how-to-section-schema", feature = "general-schema-section"),
					doc
				))]
				StepProperty::HowToSection(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(feature = "how-to-step-schema", feature = "general-schema-section"),
					doc
				))]
				StepProperty::HowToStep(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				StepProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				StepProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for StepProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "creative-work-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				StepProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "how-to-section-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<HowToSection as Deserialize>::deserialize(deserializer),
				StepProperty::HowToSection,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(feature = "how-to-step-schema", feature = "general-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<HowToStep as Deserialize>::deserialize(deserializer),
				StepProperty::HowToStep,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				StepProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				StepProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property step or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str = "data did not match any variant of schema.org property step";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
