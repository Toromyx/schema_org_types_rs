use super::*;
/// <https://schema.org/NutritionInformation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct NutritionInformation {
	/// <https://schema.org/calories>
	pub r#calories: Vec<CaloriesProperty>,
	/// <https://schema.org/carbohydrateContent>
	pub r#carbohydrate_content: Vec<CarbohydrateContentProperty>,
	/// <https://schema.org/cholesterolContent>
	pub r#cholesterol_content: Vec<CholesterolContentProperty>,
	/// <https://schema.org/fatContent>
	pub r#fat_content: Vec<FatContentProperty>,
	/// <https://schema.org/fiberContent>
	pub r#fiber_content: Vec<FiberContentProperty>,
	/// <https://schema.org/proteinContent>
	pub r#protein_content: Vec<ProteinContentProperty>,
	/// <https://schema.org/saturatedFatContent>
	pub r#saturated_fat_content: Vec<SaturatedFatContentProperty>,
	/// <https://schema.org/servingSize>
	pub r#serving_size: Vec<ServingSizeProperty>,
	/// <https://schema.org/sodiumContent>
	pub r#sodium_content: Vec<SodiumContentProperty>,
	/// <https://schema.org/sugarContent>
	pub r#sugar_content: Vec<SugarContentProperty>,
	/// <https://schema.org/transFatContent>
	pub r#trans_fat_content: Vec<TransFatContentProperty>,
	/// <https://schema.org/unsaturatedFatContent>
	pub r#unsaturated_fat_content: Vec<UnsaturatedFatContentProperty>,
	/// <https://schema.org/additionalType>
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	/// <https://schema.org/alternateName>
	pub r#alternate_name: Vec<AlternateNameProperty>,
	/// <https://schema.org/description>
	pub r#description: Vec<DescriptionProperty>,
	/// <https://schema.org/disambiguatingDescription>
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	/// <https://schema.org/identifier>
	pub r#identifier: Vec<IdentifierProperty>,
	/// <https://schema.org/image>
	pub r#image: Vec<ImageProperty>,
	/// <https://schema.org/mainEntityOfPage>
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	/// <https://schema.org/name>
	pub r#name: Vec<NameProperty>,
	/// <https://schema.org/potentialAction>
	pub r#potential_action: Vec<PotentialActionProperty>,
	/// <https://schema.org/sameAs>
	pub r#same_as: Vec<SameAsProperty>,
	/// <https://schema.org/subjectOf>
	pub r#subject_of: Vec<SubjectOfProperty>,
	/// <https://schema.org/url>
	pub r#url: Vec<UrlProperty>,
}
/// This trait is for properties from <https://schema.org/NutritionInformation>.
pub trait NutritionInformationTrait {
	/// Get <https://schema.org/calories> from [`Self`] as borrowed slice.
	fn get_calories(&self) -> &[CaloriesProperty];
	/// Take <https://schema.org/calories> from [`Self`] as owned vector.
	fn take_calories(&mut self) -> Vec<CaloriesProperty>;
	/// Get <https://schema.org/carbohydrateContent> from [`Self`] as borrowed slice.
	fn get_carbohydrate_content(&self) -> &[CarbohydrateContentProperty];
	/// Take <https://schema.org/carbohydrateContent> from [`Self`] as owned vector.
	fn take_carbohydrate_content(&mut self) -> Vec<CarbohydrateContentProperty>;
	/// Get <https://schema.org/cholesterolContent> from [`Self`] as borrowed slice.
	fn get_cholesterol_content(&self) -> &[CholesterolContentProperty];
	/// Take <https://schema.org/cholesterolContent> from [`Self`] as owned vector.
	fn take_cholesterol_content(&mut self) -> Vec<CholesterolContentProperty>;
	/// Get <https://schema.org/fatContent> from [`Self`] as borrowed slice.
	fn get_fat_content(&self) -> &[FatContentProperty];
	/// Take <https://schema.org/fatContent> from [`Self`] as owned vector.
	fn take_fat_content(&mut self) -> Vec<FatContentProperty>;
	/// Get <https://schema.org/fiberContent> from [`Self`] as borrowed slice.
	fn get_fiber_content(&self) -> &[FiberContentProperty];
	/// Take <https://schema.org/fiberContent> from [`Self`] as owned vector.
	fn take_fiber_content(&mut self) -> Vec<FiberContentProperty>;
	/// Get <https://schema.org/proteinContent> from [`Self`] as borrowed slice.
	fn get_protein_content(&self) -> &[ProteinContentProperty];
	/// Take <https://schema.org/proteinContent> from [`Self`] as owned vector.
	fn take_protein_content(&mut self) -> Vec<ProteinContentProperty>;
	/// Get <https://schema.org/saturatedFatContent> from [`Self`] as borrowed slice.
	fn get_saturated_fat_content(&self) -> &[SaturatedFatContentProperty];
	/// Take <https://schema.org/saturatedFatContent> from [`Self`] as owned vector.
	fn take_saturated_fat_content(&mut self) -> Vec<SaturatedFatContentProperty>;
	/// Get <https://schema.org/servingSize> from [`Self`] as borrowed slice.
	fn get_serving_size(&self) -> &[ServingSizeProperty];
	/// Take <https://schema.org/servingSize> from [`Self`] as owned vector.
	fn take_serving_size(&mut self) -> Vec<ServingSizeProperty>;
	/// Get <https://schema.org/sodiumContent> from [`Self`] as borrowed slice.
	fn get_sodium_content(&self) -> &[SodiumContentProperty];
	/// Take <https://schema.org/sodiumContent> from [`Self`] as owned vector.
	fn take_sodium_content(&mut self) -> Vec<SodiumContentProperty>;
	/// Get <https://schema.org/sugarContent> from [`Self`] as borrowed slice.
	fn get_sugar_content(&self) -> &[SugarContentProperty];
	/// Take <https://schema.org/sugarContent> from [`Self`] as owned vector.
	fn take_sugar_content(&mut self) -> Vec<SugarContentProperty>;
	/// Get <https://schema.org/transFatContent> from [`Self`] as borrowed slice.
	fn get_trans_fat_content(&self) -> &[TransFatContentProperty];
	/// Take <https://schema.org/transFatContent> from [`Self`] as owned vector.
	fn take_trans_fat_content(&mut self) -> Vec<TransFatContentProperty>;
	/// Get <https://schema.org/unsaturatedFatContent> from [`Self`] as borrowed slice.
	fn get_unsaturated_fat_content(&self) -> &[UnsaturatedFatContentProperty];
	/// Take <https://schema.org/unsaturatedFatContent> from [`Self`] as owned vector.
	fn take_unsaturated_fat_content(&mut self) -> Vec<UnsaturatedFatContentProperty>;
}
impl NutritionInformationTrait for NutritionInformation {
	fn get_calories(&self) -> &[CaloriesProperty] {
		self.r#calories.as_slice()
	}
	fn take_calories(&mut self) -> Vec<CaloriesProperty> {
		std::mem::take(&mut self.r#calories)
	}
	fn get_carbohydrate_content(&self) -> &[CarbohydrateContentProperty] {
		self.r#carbohydrate_content.as_slice()
	}
	fn take_carbohydrate_content(&mut self) -> Vec<CarbohydrateContentProperty> {
		std::mem::take(&mut self.r#carbohydrate_content)
	}
	fn get_cholesterol_content(&self) -> &[CholesterolContentProperty] {
		self.r#cholesterol_content.as_slice()
	}
	fn take_cholesterol_content(&mut self) -> Vec<CholesterolContentProperty> {
		std::mem::take(&mut self.r#cholesterol_content)
	}
	fn get_fat_content(&self) -> &[FatContentProperty] {
		self.r#fat_content.as_slice()
	}
	fn take_fat_content(&mut self) -> Vec<FatContentProperty> {
		std::mem::take(&mut self.r#fat_content)
	}
	fn get_fiber_content(&self) -> &[FiberContentProperty] {
		self.r#fiber_content.as_slice()
	}
	fn take_fiber_content(&mut self) -> Vec<FiberContentProperty> {
		std::mem::take(&mut self.r#fiber_content)
	}
	fn get_protein_content(&self) -> &[ProteinContentProperty] {
		self.r#protein_content.as_slice()
	}
	fn take_protein_content(&mut self) -> Vec<ProteinContentProperty> {
		std::mem::take(&mut self.r#protein_content)
	}
	fn get_saturated_fat_content(&self) -> &[SaturatedFatContentProperty] {
		self.r#saturated_fat_content.as_slice()
	}
	fn take_saturated_fat_content(&mut self) -> Vec<SaturatedFatContentProperty> {
		std::mem::take(&mut self.r#saturated_fat_content)
	}
	fn get_serving_size(&self) -> &[ServingSizeProperty] {
		self.r#serving_size.as_slice()
	}
	fn take_serving_size(&mut self) -> Vec<ServingSizeProperty> {
		std::mem::take(&mut self.r#serving_size)
	}
	fn get_sodium_content(&self) -> &[SodiumContentProperty] {
		self.r#sodium_content.as_slice()
	}
	fn take_sodium_content(&mut self) -> Vec<SodiumContentProperty> {
		std::mem::take(&mut self.r#sodium_content)
	}
	fn get_sugar_content(&self) -> &[SugarContentProperty] {
		self.r#sugar_content.as_slice()
	}
	fn take_sugar_content(&mut self) -> Vec<SugarContentProperty> {
		std::mem::take(&mut self.r#sugar_content)
	}
	fn get_trans_fat_content(&self) -> &[TransFatContentProperty] {
		self.r#trans_fat_content.as_slice()
	}
	fn take_trans_fat_content(&mut self) -> Vec<TransFatContentProperty> {
		std::mem::take(&mut self.r#trans_fat_content)
	}
	fn get_unsaturated_fat_content(&self) -> &[UnsaturatedFatContentProperty] {
		self.r#unsaturated_fat_content.as_slice()
	}
	fn take_unsaturated_fat_content(&mut self) -> Vec<UnsaturatedFatContentProperty> {
		std::mem::take(&mut self.r#unsaturated_fat_content)
	}
}
impl StructuredValueTrait for NutritionInformation {}
impl ThingTrait for NutritionInformation {
	fn get_additional_type(&self) -> &[AdditionalTypeProperty] {
		self.r#additional_type.as_slice()
	}
	fn take_additional_type(&mut self) -> Vec<AdditionalTypeProperty> {
		std::mem::take(&mut self.r#additional_type)
	}
	fn get_alternate_name(&self) -> &[AlternateNameProperty] {
		self.r#alternate_name.as_slice()
	}
	fn take_alternate_name(&mut self) -> Vec<AlternateNameProperty> {
		std::mem::take(&mut self.r#alternate_name)
	}
	fn get_description(&self) -> &[DescriptionProperty] {
		self.r#description.as_slice()
	}
	fn take_description(&mut self) -> Vec<DescriptionProperty> {
		std::mem::take(&mut self.r#description)
	}
	fn get_disambiguating_description(&self) -> &[DisambiguatingDescriptionProperty] {
		self.r#disambiguating_description.as_slice()
	}
	fn take_disambiguating_description(&mut self) -> Vec<DisambiguatingDescriptionProperty> {
		std::mem::take(&mut self.r#disambiguating_description)
	}
	fn get_identifier(&self) -> &[IdentifierProperty] {
		self.r#identifier.as_slice()
	}
	fn take_identifier(&mut self) -> Vec<IdentifierProperty> {
		std::mem::take(&mut self.r#identifier)
	}
	fn get_image(&self) -> &[ImageProperty] {
		self.r#image.as_slice()
	}
	fn take_image(&mut self) -> Vec<ImageProperty> {
		std::mem::take(&mut self.r#image)
	}
	fn get_main_entity_of_page(&self) -> &[MainEntityOfPageProperty] {
		self.r#main_entity_of_page.as_slice()
	}
	fn take_main_entity_of_page(&mut self) -> Vec<MainEntityOfPageProperty> {
		std::mem::take(&mut self.r#main_entity_of_page)
	}
	fn get_name(&self) -> &[NameProperty] {
		self.r#name.as_slice()
	}
	fn take_name(&mut self) -> Vec<NameProperty> {
		std::mem::take(&mut self.r#name)
	}
	fn get_potential_action(&self) -> &[PotentialActionProperty] {
		self.r#potential_action.as_slice()
	}
	fn take_potential_action(&mut self) -> Vec<PotentialActionProperty> {
		std::mem::take(&mut self.r#potential_action)
	}
	fn get_same_as(&self) -> &[SameAsProperty] {
		self.r#same_as.as_slice()
	}
	fn take_same_as(&mut self) -> Vec<SameAsProperty> {
		std::mem::take(&mut self.r#same_as)
	}
	fn get_subject_of(&self) -> &[SubjectOfProperty] {
		self.r#subject_of.as_slice()
	}
	fn take_subject_of(&mut self) -> Vec<SubjectOfProperty> {
		std::mem::take(&mut self.r#subject_of)
	}
	fn get_url(&self) -> &[UrlProperty] {
		self.r#url.as_slice()
	}
	fn take_url(&mut self) -> Vec<UrlProperty> {
		std::mem::take(&mut self.r#url)
	}
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for NutritionInformation {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#calories) as usize,
				!Vec::is_empty(&self.r#carbohydrate_content) as usize,
				!Vec::is_empty(&self.r#cholesterol_content) as usize,
				!Vec::is_empty(&self.r#fat_content) as usize,
				!Vec::is_empty(&self.r#fiber_content) as usize,
				!Vec::is_empty(&self.r#protein_content) as usize,
				!Vec::is_empty(&self.r#saturated_fat_content) as usize,
				!Vec::is_empty(&self.r#serving_size) as usize,
				!Vec::is_empty(&self.r#sodium_content) as usize,
				!Vec::is_empty(&self.r#sugar_content) as usize,
				!Vec::is_empty(&self.r#trans_fat_content) as usize,
				!Vec::is_empty(&self.r#unsaturated_fat_content) as usize,
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#url) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "NutritionInformation", len)?;
			if !Vec::is_empty(&self.r#calories) {
				serialize_struct.serialize_field("calories", {
					struct SerializeWith<'a>(&'a Vec<CaloriesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#calories)
				})?;
			} else {
				serialize_struct.skip_field("calories")?;
			}
			if !Vec::is_empty(&self.r#carbohydrate_content) {
				serialize_struct.serialize_field("carbohydrateContent", {
					struct SerializeWith<'a>(&'a Vec<CarbohydrateContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#carbohydrate_content)
				})?;
			} else {
				serialize_struct.skip_field("carbohydrateContent")?;
			}
			if !Vec::is_empty(&self.r#cholesterol_content) {
				serialize_struct.serialize_field("cholesterolContent", {
					struct SerializeWith<'a>(&'a Vec<CholesterolContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cholesterol_content)
				})?;
			} else {
				serialize_struct.skip_field("cholesterolContent")?;
			}
			if !Vec::is_empty(&self.r#fat_content) {
				serialize_struct.serialize_field("fatContent", {
					struct SerializeWith<'a>(&'a Vec<FatContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fat_content)
				})?;
			} else {
				serialize_struct.skip_field("fatContent")?;
			}
			if !Vec::is_empty(&self.r#fiber_content) {
				serialize_struct.serialize_field("fiberContent", {
					struct SerializeWith<'a>(&'a Vec<FiberContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#fiber_content)
				})?;
			} else {
				serialize_struct.skip_field("fiberContent")?;
			}
			if !Vec::is_empty(&self.r#protein_content) {
				serialize_struct.serialize_field("proteinContent", {
					struct SerializeWith<'a>(&'a Vec<ProteinContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#protein_content)
				})?;
			} else {
				serialize_struct.skip_field("proteinContent")?;
			}
			if !Vec::is_empty(&self.r#saturated_fat_content) {
				serialize_struct.serialize_field("saturatedFatContent", {
					struct SerializeWith<'a>(&'a Vec<SaturatedFatContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#saturated_fat_content)
				})?;
			} else {
				serialize_struct.skip_field("saturatedFatContent")?;
			}
			if !Vec::is_empty(&self.r#serving_size) {
				serialize_struct.serialize_field("servingSize", {
					struct SerializeWith<'a>(&'a Vec<ServingSizeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#serving_size)
				})?;
			} else {
				serialize_struct.skip_field("servingSize")?;
			}
			if !Vec::is_empty(&self.r#sodium_content) {
				serialize_struct.serialize_field("sodiumContent", {
					struct SerializeWith<'a>(&'a Vec<SodiumContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sodium_content)
				})?;
			} else {
				serialize_struct.skip_field("sodiumContent")?;
			}
			if !Vec::is_empty(&self.r#sugar_content) {
				serialize_struct.serialize_field("sugarContent", {
					struct SerializeWith<'a>(&'a Vec<SugarContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sugar_content)
				})?;
			} else {
				serialize_struct.skip_field("sugarContent")?;
			}
			if !Vec::is_empty(&self.r#trans_fat_content) {
				serialize_struct.serialize_field("transFatContent", {
					struct SerializeWith<'a>(&'a Vec<TransFatContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#trans_fat_content)
				})?;
			} else {
				serialize_struct.skip_field("transFatContent")?;
			}
			if !Vec::is_empty(&self.r#unsaturated_fat_content) {
				serialize_struct.serialize_field("unsaturatedFatContent", {
					struct SerializeWith<'a>(&'a Vec<UnsaturatedFatContentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#unsaturated_fat_content)
				})?;
			} else {
				serialize_struct.skip_field("unsaturatedFatContent")?;
			}
			if !Vec::is_empty(&self.r#additional_type) {
				serialize_struct.serialize_field("additionalType", {
					struct SerializeWith<'a>(&'a Vec<AdditionalTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#additional_type)
				})?;
			} else {
				serialize_struct.skip_field("additionalType")?;
			}
			if !Vec::is_empty(&self.r#alternate_name) {
				serialize_struct.serialize_field("alternateName", {
					struct SerializeWith<'a>(&'a Vec<AlternateNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#alternate_name)
				})?;
			} else {
				serialize_struct.skip_field("alternateName")?;
			}
			if !Vec::is_empty(&self.r#description) {
				serialize_struct.serialize_field("description", {
					struct SerializeWith<'a>(&'a Vec<DescriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#description)
				})?;
			} else {
				serialize_struct.skip_field("description")?;
			}
			if !Vec::is_empty(&self.r#disambiguating_description) {
				serialize_struct.serialize_field("disambiguatingDescription", {
					struct SerializeWith<'a>(&'a Vec<DisambiguatingDescriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#disambiguating_description)
				})?;
			} else {
				serialize_struct.skip_field("disambiguatingDescription")?;
			}
			if !Vec::is_empty(&self.r#identifier) {
				serialize_struct.serialize_field("identifier", {
					struct SerializeWith<'a>(&'a Vec<IdentifierProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#identifier)
				})?;
			} else {
				serialize_struct.skip_field("identifier")?;
			}
			if !Vec::is_empty(&self.r#image) {
				serialize_struct.serialize_field("image", {
					struct SerializeWith<'a>(&'a Vec<ImageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#image)
				})?;
			} else {
				serialize_struct.skip_field("image")?;
			}
			if !Vec::is_empty(&self.r#main_entity_of_page) {
				serialize_struct.serialize_field("mainEntityOfPage", {
					struct SerializeWith<'a>(&'a Vec<MainEntityOfPageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#main_entity_of_page)
				})?;
			} else {
				serialize_struct.skip_field("mainEntityOfPage")?;
			}
			if !Vec::is_empty(&self.r#name) {
				serialize_struct.serialize_field("name", {
					struct SerializeWith<'a>(&'a Vec<NameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#name)
				})?;
			} else {
				serialize_struct.skip_field("name")?;
			}
			if !Vec::is_empty(&self.r#potential_action) {
				serialize_struct.serialize_field("potentialAction", {
					struct SerializeWith<'a>(&'a Vec<PotentialActionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#potential_action)
				})?;
			} else {
				serialize_struct.skip_field("potentialAction")?;
			}
			if !Vec::is_empty(&self.r#same_as) {
				serialize_struct.serialize_field("sameAs", {
					struct SerializeWith<'a>(&'a Vec<SameAsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#same_as)
				})?;
			} else {
				serialize_struct.skip_field("sameAs")?;
			}
			if !Vec::is_empty(&self.r#subject_of) {
				serialize_struct.serialize_field("subjectOf", {
					struct SerializeWith<'a>(&'a Vec<SubjectOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#subject_of)
				})?;
			} else {
				serialize_struct.skip_field("subjectOf")?;
			}
			if !Vec::is_empty(&self.r#url) {
				serialize_struct.serialize_field("url", {
					struct SerializeWith<'a>(&'a Vec<UrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#url)
				})?;
			} else {
				serialize_struct.skip_field("url")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for NutritionInformation {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Calories,
				CarbohydrateContent,
				CholesterolContent,
				FatContent,
				FiberContent,
				ProteinContent,
				SaturatedFatContent,
				ServingSize,
				SodiumContent,
				SugarContent,
				TransFatContent,
				UnsaturatedFatContent,
				AdditionalType,
				AlternateName,
				Description,
				DisambiguatingDescription,
				Identifier,
				Image,
				MainEntityOfPage,
				Name,
				PotentialAction,
				SameAs,
				SubjectOf,
				Url,
				Ignore,
			}
			struct FieldVisitor;
			impl<'de> Visitor<'de> for FieldVisitor {
				type Value = Field;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("field identifier")
				}
				fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						"calories" => Ok(Field::Calories),
						"carbohydrateContent" => Ok(Field::CarbohydrateContent),
						"cholesterolContent" => Ok(Field::CholesterolContent),
						"fatContent" => Ok(Field::FatContent),
						"fiberContent" => Ok(Field::FiberContent),
						"proteinContent" => Ok(Field::ProteinContent),
						"saturatedFatContent" => Ok(Field::SaturatedFatContent),
						"servingSize" => Ok(Field::ServingSize),
						"sodiumContent" => Ok(Field::SodiumContent),
						"sugarContent" => Ok(Field::SugarContent),
						"transFatContent" => Ok(Field::TransFatContent),
						"unsaturatedFatContent" => Ok(Field::UnsaturatedFatContent),
						"additionalType" => Ok(Field::AdditionalType),
						"alternateName" => Ok(Field::AlternateName),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"name" => Ok(Field::Name),
						"potentialAction" => Ok(Field::PotentialAction),
						"sameAs" => Ok(Field::SameAs),
						"subjectOf" => Ok(Field::SubjectOf),
						"url" => Ok(Field::Url),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"calories" => Ok(Field::Calories),
						b"carbohydrateContent" => Ok(Field::CarbohydrateContent),
						b"cholesterolContent" => Ok(Field::CholesterolContent),
						b"fatContent" => Ok(Field::FatContent),
						b"fiberContent" => Ok(Field::FiberContent),
						b"proteinContent" => Ok(Field::ProteinContent),
						b"saturatedFatContent" => Ok(Field::SaturatedFatContent),
						b"servingSize" => Ok(Field::ServingSize),
						b"sodiumContent" => Ok(Field::SodiumContent),
						b"sugarContent" => Ok(Field::SugarContent),
						b"transFatContent" => Ok(Field::TransFatContent),
						b"unsaturatedFatContent" => Ok(Field::UnsaturatedFatContent),
						b"additionalType" => Ok(Field::AdditionalType),
						b"alternateName" => Ok(Field::AlternateName),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"name" => Ok(Field::Name),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"sameAs" => Ok(Field::SameAs),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"url" => Ok(Field::Url),
						_ => Ok(Field::Ignore),
					}
				}
			}
			impl<'de> Deserialize<'de> for Field {
				fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where
					D: Deserializer<'de>,
				{
					deserializer.deserialize_identifier(FieldVisitor)
				}
			}
			struct ClassVisitor;
			impl<'de> Visitor<'de> for ClassVisitor {
				type Value = NutritionInformation;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema NutritionInformation")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#calories_property = None;
					let mut r#carbohydrate_content_property = None;
					let mut r#cholesterol_content_property = None;
					let mut r#fat_content_property = None;
					let mut r#fiber_content_property = None;
					let mut r#protein_content_property = None;
					let mut r#saturated_fat_content_property = None;
					let mut r#serving_size_property = None;
					let mut r#sodium_content_property = None;
					let mut r#sugar_content_property = None;
					let mut r#trans_fat_content_property = None;
					let mut r#unsaturated_fat_content_property = None;
					let mut r#additional_type_property = None;
					let mut r#alternate_name_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#name_property = None;
					let mut r#potential_action_property = None;
					let mut r#same_as_property = None;
					let mut r#subject_of_property = None;
					let mut r#url_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							Field::Calories => {
								if r#calories_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"calories",
									));
								}
								r#calories_property = Some({
									struct DeserializeWith(Vec<CaloriesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CarbohydrateContent => {
								if r#carbohydrate_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"carbohydrateContent",
									));
								}
								r#carbohydrate_content_property = Some({
									struct DeserializeWith(Vec<CarbohydrateContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CholesterolContent => {
								if r#cholesterol_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cholesterolContent",
									));
								}
								r#cholesterol_content_property = Some({
									struct DeserializeWith(Vec<CholesterolContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::FatContent => {
								if r#fat_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fatContent",
									));
								}
								r#fat_content_property = Some({
									struct DeserializeWith(Vec<FatContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::FiberContent => {
								if r#fiber_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fiberContent",
									));
								}
								r#fiber_content_property = Some({
									struct DeserializeWith(Vec<FiberContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ProteinContent => {
								if r#protein_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"proteinContent",
									));
								}
								r#protein_content_property = Some({
									struct DeserializeWith(Vec<ProteinContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SaturatedFatContent => {
								if r#saturated_fat_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"saturatedFatContent",
									));
								}
								r#saturated_fat_content_property = Some({
									struct DeserializeWith(Vec<SaturatedFatContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ServingSize => {
								if r#serving_size_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"servingSize",
									));
								}
								r#serving_size_property = Some({
									struct DeserializeWith(Vec<ServingSizeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SodiumContent => {
								if r#sodium_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sodiumContent",
									));
								}
								r#sodium_content_property = Some({
									struct DeserializeWith(Vec<SodiumContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SugarContent => {
								if r#sugar_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sugarContent",
									));
								}
								r#sugar_content_property = Some({
									struct DeserializeWith(Vec<SugarContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::TransFatContent => {
								if r#trans_fat_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"transFatContent",
									));
								}
								r#trans_fat_content_property = Some({
									struct DeserializeWith(Vec<TransFatContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::UnsaturatedFatContent => {
								if r#unsaturated_fat_content_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"unsaturatedFatContent",
									));
								}
								r#unsaturated_fat_content_property = Some({
									struct DeserializeWith(Vec<UnsaturatedFatContentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AdditionalType => {
								if r#additional_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalType",
									));
								}
								r#additional_type_property = Some({
									struct DeserializeWith(Vec<AdditionalTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AlternateName => {
								if r#alternate_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"alternateName",
									));
								}
								r#alternate_name_property = Some({
									struct DeserializeWith(Vec<AlternateNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Description => {
								if r#description_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"description",
									));
								}
								r#description_property = Some({
									struct DeserializeWith(Vec<DescriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::DisambiguatingDescription => {
								if r#disambiguating_description_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"disambiguatingDescription",
									));
								}
								r#disambiguating_description_property = Some({
									struct DeserializeWith(Vec<DisambiguatingDescriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Identifier => {
								if r#identifier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"identifier",
									));
								}
								r#identifier_property = Some({
									struct DeserializeWith(Vec<IdentifierProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Image => {
								if r#image_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("image"));
								}
								r#image_property = Some({
									struct DeserializeWith(Vec<ImageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::MainEntityOfPage => {
								if r#main_entity_of_page_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mainEntityOfPage",
									));
								}
								r#main_entity_of_page_property = Some({
									struct DeserializeWith(Vec<MainEntityOfPageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Name => {
								if r#name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("name"));
								}
								r#name_property = Some({
									struct DeserializeWith(Vec<NameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PotentialAction => {
								if r#potential_action_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"potentialAction",
									));
								}
								r#potential_action_property = Some({
									struct DeserializeWith(Vec<PotentialActionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SameAs => {
								if r#same_as_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("sameAs"));
								}
								r#same_as_property = Some({
									struct DeserializeWith(Vec<SameAsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SubjectOf => {
								if r#subject_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subjectOf",
									));
								}
								r#subject_of_property = Some({
									struct DeserializeWith(Vec<SubjectOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Url => {
								if r#url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("url"));
								}
								r#url_property = Some({
									struct DeserializeWith(Vec<UrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(NutritionInformation {
						r#calories: r#calories_property.unwrap_or_default(),
						r#carbohydrate_content: r#carbohydrate_content_property.unwrap_or_default(),
						r#cholesterol_content: r#cholesterol_content_property.unwrap_or_default(),
						r#fat_content: r#fat_content_property.unwrap_or_default(),
						r#fiber_content: r#fiber_content_property.unwrap_or_default(),
						r#protein_content: r#protein_content_property.unwrap_or_default(),
						r#saturated_fat_content: r#saturated_fat_content_property
							.unwrap_or_default(),
						r#serving_size: r#serving_size_property.unwrap_or_default(),
						r#sodium_content: r#sodium_content_property.unwrap_or_default(),
						r#sugar_content: r#sugar_content_property.unwrap_or_default(),
						r#trans_fat_content: r#trans_fat_content_property.unwrap_or_default(),
						r#unsaturated_fat_content: r#unsaturated_fat_content_property
							.unwrap_or_default(),
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"calories",
				"carbohydrateContent",
				"cholesterolContent",
				"fatContent",
				"fiberContent",
				"proteinContent",
				"saturatedFatContent",
				"servingSize",
				"sodiumContent",
				"sugarContent",
				"transFatContent",
				"unsaturatedFatContent",
				"additionalType",
				"alternateName",
				"description",
				"disambiguatingDescription",
				"identifier",
				"image",
				"mainEntityOfPage",
				"name",
				"potentialAction",
				"sameAs",
				"subjectOf",
				"url",
			];
			deserializer.deserialize_struct("NutritionInformation", FIELDS, ClassVisitor)
		}
	}
}
