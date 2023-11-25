use super::*;
/// <https://schema.org/recipeInstructions>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub enum RecipeInstructionsProperty {
	/// <https://schema.org/CreativeWork>
	CreativeWork(CreativeWork),
	/// <https://schema.org/HowToSection>
	HowToSection(HowToSection),
	/// <https://schema.org/HowToStep>
	HowToStep(HowToStep),
	/// <https://schema.org/ItemList>
	ItemList(ItemList),
	/// <https://schema.org/Text>
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
	impl Serialize for RecipeInstructionsProperty {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			match *self {
				RecipeInstructionsProperty::CreativeWork(ref inner) => inner.serialize(serializer),
				RecipeInstructionsProperty::HowToSection(ref inner) => inner.serialize(serializer),
				RecipeInstructionsProperty::HowToStep(ref inner) => inner.serialize(serializer),
				RecipeInstructionsProperty::ItemList(ref inner) => inner.serialize(serializer),
				RecipeInstructionsProperty::Text(ref inner) => inner.serialize(serializer),
				#[cfg(all(feature = "fallible", feature = "serde"))]
				RecipeInstructionsProperty::SerdeFail(ref inner) => inner.serialize(serializer),
			}
		}
	}
	impl<'de> Deserialize<'de> for RecipeInstructionsProperty {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			let content =
				<::serde::__private::de::Content as Deserialize>::deserialize(deserializer)?;
			let deserializer =
				::serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
			if let Ok(ok) = Result::map(
				<CreativeWork as Deserialize>::deserialize(deserializer),
				RecipeInstructionsProperty::CreativeWork,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<HowToSection as Deserialize>::deserialize(deserializer),
				RecipeInstructionsProperty::HowToSection,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<HowToStep as Deserialize>::deserialize(deserializer),
				RecipeInstructionsProperty::HowToStep,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<ItemList as Deserialize>::deserialize(deserializer),
				RecipeInstructionsProperty::ItemList,
			) {
				return Ok(ok);
			}
			if let Ok(ok) = Result::map(
				<Text as Deserialize>::deserialize(deserializer),
				RecipeInstructionsProperty::Text,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			if let Ok(ok) = Result::map(
				<crate::fallible::FailValue as Deserialize>::deserialize(deserializer),
				RecipeInstructionsProperty::SerdeFail,
			) {
				return Ok(ok);
			}
			#[cfg(all(feature = "fallible", feature = "serde"))]
			const CUSTOM_ERROR: &str = "data did neither match any variant of schema.org property recipeInstructions or was able to be deserialized into a generic value";
			#[cfg(any(not(feature = "fallible"), not(feature = "serde")))]
			const CUSTOM_ERROR: &str =
				"data did not match any variant of schema.org property recipeInstructions";
			Err(de::Error::custom(CUSTOM_ERROR))
		}
	}
}
