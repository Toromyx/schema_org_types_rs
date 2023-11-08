use super::*;
/// <https://schema.org/category>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum CategoryProperty {
	#[cfg(any(
		any(feature = "category-code-schema", feature = "pending-schema-section"),
		doc
	))]
	CategoryCode(CategoryCode),
	#[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
	Thing(Thing),
	#[cfg(any(
		any(
			feature = "physical-activity-category-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	PhysicalActivityCategory(PhysicalActivityCategory),
	#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
	Url(Url),
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
	impl Serialize for CategoryProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				#[cfg(any(
					any(feature = "category-code-schema", feature = "pending-schema-section"),
					doc
				))]
				CategoryProperty::CategoryCode(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
				CategoryProperty::Thing(ref inner) => inner.serialize(serializer),
				#[cfg(any(
					any(
						feature = "physical-activity-category-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				CategoryProperty::PhysicalActivityCategory(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
				CategoryProperty::Url(ref inner) => inner.serialize(serializer),
				#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
				CategoryProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				CategoryProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for CategoryProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			#[cfg(any(
				any(feature = "category-code-schema", feature = "pending-schema-section"),
				doc
			))]
			if let Ok(ok) = Result::map(
				<CategoryCode as Deserialize>::deserialize(deserializer),
				CategoryProperty::CategoryCode,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "thing-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Thing as Deserialize>::deserialize(deserializer),
				CategoryProperty::Thing,
			) {
				return Ok(ok);
			}
			#[cfg(any(
				any(
					feature = "physical-activity-category-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if let Ok(ok) = Result::map(
				<PhysicalActivityCategory as Deserialize>::deserialize(deserializer),
				CategoryProperty::PhysicalActivityCategory,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "url-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Url as Deserialize>::deserialize(deserializer),
				CategoryProperty::Url,
			) {
				return Ok(ok);
			}
			#[cfg(any(any(feature = "text-schema", feature = "general-schema-section"), doc))]
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				CategoryProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				CategoryProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property category or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property category";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
