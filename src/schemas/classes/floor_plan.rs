use super::*;
/// <https://schema.org/FloorPlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct FloorPlan {
	/// <https://schema.org/amenityFeature>
	pub r#amenity_feature: Vec<AmenityFeatureProperty>,
	/// <https://schema.org/floorSize>
	pub r#floor_size: Vec<FloorSizeProperty>,
	/// <https://schema.org/isPlanForApartment>
	pub r#is_plan_for_apartment: Vec<IsPlanForApartmentProperty>,
	/// <https://schema.org/layoutImage>
	pub r#layout_image: Vec<LayoutImageProperty>,
	/// <https://schema.org/numberOfAccommodationUnits>
	pub r#number_of_accommodation_units: Vec<NumberOfAccommodationUnitsProperty>,
	/// <https://schema.org/numberOfAvailableAccommodationUnits>
	pub r#number_of_available_accommodation_units: Vec<NumberOfAvailableAccommodationUnitsProperty>,
	/// <https://schema.org/numberOfBathroomsTotal>
	pub r#number_of_bathrooms_total: Vec<NumberOfBathroomsTotalProperty>,
	/// <https://schema.org/numberOfBedrooms>
	pub r#number_of_bedrooms: Vec<NumberOfBedroomsProperty>,
	/// <https://schema.org/numberOfFullBathrooms>
	pub r#number_of_full_bathrooms: Vec<NumberOfFullBathroomsProperty>,
	/// <https://schema.org/numberOfPartialBathrooms>
	pub r#number_of_partial_bathrooms: Vec<NumberOfPartialBathroomsProperty>,
	/// <https://schema.org/numberOfRooms>
	pub r#number_of_rooms: Vec<NumberOfRoomsProperty>,
	/// <https://schema.org/petsAllowed>
	pub r#pets_allowed: Vec<PetsAllowedProperty>,
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
/// This trait is for properties from <https://schema.org/FloorPlan>.
pub trait FloorPlanTrait {
	/// Get <https://schema.org/amenityFeature> from [`Self`] as borrowed slice.
	fn get_amenity_feature(&self) -> &[AmenityFeatureProperty];
	/// Take <https://schema.org/amenityFeature> from [`Self`] as owned vector.
	fn take_amenity_feature(&mut self) -> Vec<AmenityFeatureProperty>;
	/// Get <https://schema.org/floorSize> from [`Self`] as borrowed slice.
	fn get_floor_size(&self) -> &[FloorSizeProperty];
	/// Take <https://schema.org/floorSize> from [`Self`] as owned vector.
	fn take_floor_size(&mut self) -> Vec<FloorSizeProperty>;
	/// Get <https://schema.org/isPlanForApartment> from [`Self`] as borrowed slice.
	fn get_is_plan_for_apartment(&self) -> &[IsPlanForApartmentProperty];
	/// Take <https://schema.org/isPlanForApartment> from [`Self`] as owned vector.
	fn take_is_plan_for_apartment(&mut self) -> Vec<IsPlanForApartmentProperty>;
	/// Get <https://schema.org/layoutImage> from [`Self`] as borrowed slice.
	fn get_layout_image(&self) -> &[LayoutImageProperty];
	/// Take <https://schema.org/layoutImage> from [`Self`] as owned vector.
	fn take_layout_image(&mut self) -> Vec<LayoutImageProperty>;
	/// Get <https://schema.org/numberOfAccommodationUnits> from [`Self`] as borrowed slice.
	fn get_number_of_accommodation_units(&self) -> &[NumberOfAccommodationUnitsProperty];
	/// Take <https://schema.org/numberOfAccommodationUnits> from [`Self`] as owned vector.
	fn take_number_of_accommodation_units(&mut self) -> Vec<NumberOfAccommodationUnitsProperty>;
	/// Get <https://schema.org/numberOfAvailableAccommodationUnits> from [`Self`] as borrowed slice.
	fn get_number_of_available_accommodation_units(
		&self,
	) -> &[NumberOfAvailableAccommodationUnitsProperty];
	/// Take <https://schema.org/numberOfAvailableAccommodationUnits> from [`Self`] as owned vector.
	fn take_number_of_available_accommodation_units(
		&mut self,
	) -> Vec<NumberOfAvailableAccommodationUnitsProperty>;
	/// Get <https://schema.org/numberOfBathroomsTotal> from [`Self`] as borrowed slice.
	fn get_number_of_bathrooms_total(&self) -> &[NumberOfBathroomsTotalProperty];
	/// Take <https://schema.org/numberOfBathroomsTotal> from [`Self`] as owned vector.
	fn take_number_of_bathrooms_total(&mut self) -> Vec<NumberOfBathroomsTotalProperty>;
	/// Get <https://schema.org/numberOfBedrooms> from [`Self`] as borrowed slice.
	fn get_number_of_bedrooms(&self) -> &[NumberOfBedroomsProperty];
	/// Take <https://schema.org/numberOfBedrooms> from [`Self`] as owned vector.
	fn take_number_of_bedrooms(&mut self) -> Vec<NumberOfBedroomsProperty>;
	/// Get <https://schema.org/numberOfFullBathrooms> from [`Self`] as borrowed slice.
	fn get_number_of_full_bathrooms(&self) -> &[NumberOfFullBathroomsProperty];
	/// Take <https://schema.org/numberOfFullBathrooms> from [`Self`] as owned vector.
	fn take_number_of_full_bathrooms(&mut self) -> Vec<NumberOfFullBathroomsProperty>;
	/// Get <https://schema.org/numberOfPartialBathrooms> from [`Self`] as borrowed slice.
	fn get_number_of_partial_bathrooms(&self) -> &[NumberOfPartialBathroomsProperty];
	/// Take <https://schema.org/numberOfPartialBathrooms> from [`Self`] as owned vector.
	fn take_number_of_partial_bathrooms(&mut self) -> Vec<NumberOfPartialBathroomsProperty>;
	/// Get <https://schema.org/numberOfRooms> from [`Self`] as borrowed slice.
	fn get_number_of_rooms(&self) -> &[NumberOfRoomsProperty];
	/// Take <https://schema.org/numberOfRooms> from [`Self`] as owned vector.
	fn take_number_of_rooms(&mut self) -> Vec<NumberOfRoomsProperty>;
	/// Get <https://schema.org/petsAllowed> from [`Self`] as borrowed slice.
	fn get_pets_allowed(&self) -> &[PetsAllowedProperty];
	/// Take <https://schema.org/petsAllowed> from [`Self`] as owned vector.
	fn take_pets_allowed(&mut self) -> Vec<PetsAllowedProperty>;
}
impl FloorPlanTrait for FloorPlan {
	fn get_amenity_feature(&self) -> &[AmenityFeatureProperty] {
		self.r#amenity_feature.as_slice()
	}
	fn take_amenity_feature(&mut self) -> Vec<AmenityFeatureProperty> {
		std::mem::take(&mut self.r#amenity_feature)
	}
	fn get_floor_size(&self) -> &[FloorSizeProperty] {
		self.r#floor_size.as_slice()
	}
	fn take_floor_size(&mut self) -> Vec<FloorSizeProperty> {
		std::mem::take(&mut self.r#floor_size)
	}
	fn get_is_plan_for_apartment(&self) -> &[IsPlanForApartmentProperty] {
		self.r#is_plan_for_apartment.as_slice()
	}
	fn take_is_plan_for_apartment(&mut self) -> Vec<IsPlanForApartmentProperty> {
		std::mem::take(&mut self.r#is_plan_for_apartment)
	}
	fn get_layout_image(&self) -> &[LayoutImageProperty] {
		self.r#layout_image.as_slice()
	}
	fn take_layout_image(&mut self) -> Vec<LayoutImageProperty> {
		std::mem::take(&mut self.r#layout_image)
	}
	fn get_number_of_accommodation_units(&self) -> &[NumberOfAccommodationUnitsProperty] {
		self.r#number_of_accommodation_units.as_slice()
	}
	fn take_number_of_accommodation_units(&mut self) -> Vec<NumberOfAccommodationUnitsProperty> {
		std::mem::take(&mut self.r#number_of_accommodation_units)
	}
	fn get_number_of_available_accommodation_units(
		&self,
	) -> &[NumberOfAvailableAccommodationUnitsProperty] {
		self.r#number_of_available_accommodation_units.as_slice()
	}
	fn take_number_of_available_accommodation_units(
		&mut self,
	) -> Vec<NumberOfAvailableAccommodationUnitsProperty> {
		std::mem::take(&mut self.r#number_of_available_accommodation_units)
	}
	fn get_number_of_bathrooms_total(&self) -> &[NumberOfBathroomsTotalProperty] {
		self.r#number_of_bathrooms_total.as_slice()
	}
	fn take_number_of_bathrooms_total(&mut self) -> Vec<NumberOfBathroomsTotalProperty> {
		std::mem::take(&mut self.r#number_of_bathrooms_total)
	}
	fn get_number_of_bedrooms(&self) -> &[NumberOfBedroomsProperty] {
		self.r#number_of_bedrooms.as_slice()
	}
	fn take_number_of_bedrooms(&mut self) -> Vec<NumberOfBedroomsProperty> {
		std::mem::take(&mut self.r#number_of_bedrooms)
	}
	fn get_number_of_full_bathrooms(&self) -> &[NumberOfFullBathroomsProperty] {
		self.r#number_of_full_bathrooms.as_slice()
	}
	fn take_number_of_full_bathrooms(&mut self) -> Vec<NumberOfFullBathroomsProperty> {
		std::mem::take(&mut self.r#number_of_full_bathrooms)
	}
	fn get_number_of_partial_bathrooms(&self) -> &[NumberOfPartialBathroomsProperty] {
		self.r#number_of_partial_bathrooms.as_slice()
	}
	fn take_number_of_partial_bathrooms(&mut self) -> Vec<NumberOfPartialBathroomsProperty> {
		std::mem::take(&mut self.r#number_of_partial_bathrooms)
	}
	fn get_number_of_rooms(&self) -> &[NumberOfRoomsProperty] {
		self.r#number_of_rooms.as_slice()
	}
	fn take_number_of_rooms(&mut self) -> Vec<NumberOfRoomsProperty> {
		std::mem::take(&mut self.r#number_of_rooms)
	}
	fn get_pets_allowed(&self) -> &[PetsAllowedProperty] {
		self.r#pets_allowed.as_slice()
	}
	fn take_pets_allowed(&mut self) -> Vec<PetsAllowedProperty> {
		std::mem::take(&mut self.r#pets_allowed)
	}
}
impl ThingTrait for FloorPlan {
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
	impl Serialize for FloorPlan {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#amenity_feature) as usize,
				!Vec::is_empty(&self.r#floor_size) as usize,
				!Vec::is_empty(&self.r#is_plan_for_apartment) as usize,
				!Vec::is_empty(&self.r#layout_image) as usize,
				!Vec::is_empty(&self.r#number_of_accommodation_units) as usize,
				!Vec::is_empty(&self.r#number_of_available_accommodation_units) as usize,
				!Vec::is_empty(&self.r#number_of_bathrooms_total) as usize,
				!Vec::is_empty(&self.r#number_of_bedrooms) as usize,
				!Vec::is_empty(&self.r#number_of_full_bathrooms) as usize,
				!Vec::is_empty(&self.r#number_of_partial_bathrooms) as usize,
				!Vec::is_empty(&self.r#number_of_rooms) as usize,
				!Vec::is_empty(&self.r#pets_allowed) as usize,
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
			let mut serialize_struct = Serializer::serialize_struct(serializer, "FloorPlan", len)?;
			if !Vec::is_empty(&self.r#amenity_feature) {
				serialize_struct.serialize_field("amenityFeature", {
					struct SerializeWith<'a>(&'a Vec<AmenityFeatureProperty>);
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
					&SerializeWith(&self.r#amenity_feature)
				})?;
			} else {
				serialize_struct.skip_field("amenityFeature")?;
			}
			if !Vec::is_empty(&self.r#floor_size) {
				serialize_struct.serialize_field("floorSize", {
					struct SerializeWith<'a>(&'a Vec<FloorSizeProperty>);
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
					&SerializeWith(&self.r#floor_size)
				})?;
			} else {
				serialize_struct.skip_field("floorSize")?;
			}
			if !Vec::is_empty(&self.r#is_plan_for_apartment) {
				serialize_struct.serialize_field("isPlanForApartment", {
					struct SerializeWith<'a>(&'a Vec<IsPlanForApartmentProperty>);
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
					&SerializeWith(&self.r#is_plan_for_apartment)
				})?;
			} else {
				serialize_struct.skip_field("isPlanForApartment")?;
			}
			if !Vec::is_empty(&self.r#layout_image) {
				serialize_struct.serialize_field("layoutImage", {
					struct SerializeWith<'a>(&'a Vec<LayoutImageProperty>);
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
					&SerializeWith(&self.r#layout_image)
				})?;
			} else {
				serialize_struct.skip_field("layoutImage")?;
			}
			if !Vec::is_empty(&self.r#number_of_accommodation_units) {
				serialize_struct.serialize_field("numberOfAccommodationUnits", {
					struct SerializeWith<'a>(&'a Vec<NumberOfAccommodationUnitsProperty>);
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
					&SerializeWith(&self.r#number_of_accommodation_units)
				})?;
			} else {
				serialize_struct.skip_field("numberOfAccommodationUnits")?;
			}
			if !Vec::is_empty(&self.r#number_of_available_accommodation_units) {
				serialize_struct.serialize_field("numberOfAvailableAccommodationUnits", {
					struct SerializeWith<'a>(&'a Vec<NumberOfAvailableAccommodationUnitsProperty>);
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
					&SerializeWith(&self.r#number_of_available_accommodation_units)
				})?;
			} else {
				serialize_struct.skip_field("numberOfAvailableAccommodationUnits")?;
			}
			if !Vec::is_empty(&self.r#number_of_bathrooms_total) {
				serialize_struct.serialize_field("numberOfBathroomsTotal", {
					struct SerializeWith<'a>(&'a Vec<NumberOfBathroomsTotalProperty>);
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
					&SerializeWith(&self.r#number_of_bathrooms_total)
				})?;
			} else {
				serialize_struct.skip_field("numberOfBathroomsTotal")?;
			}
			if !Vec::is_empty(&self.r#number_of_bedrooms) {
				serialize_struct.serialize_field("numberOfBedrooms", {
					struct SerializeWith<'a>(&'a Vec<NumberOfBedroomsProperty>);
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
					&SerializeWith(&self.r#number_of_bedrooms)
				})?;
			} else {
				serialize_struct.skip_field("numberOfBedrooms")?;
			}
			if !Vec::is_empty(&self.r#number_of_full_bathrooms) {
				serialize_struct.serialize_field("numberOfFullBathrooms", {
					struct SerializeWith<'a>(&'a Vec<NumberOfFullBathroomsProperty>);
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
					&SerializeWith(&self.r#number_of_full_bathrooms)
				})?;
			} else {
				serialize_struct.skip_field("numberOfFullBathrooms")?;
			}
			if !Vec::is_empty(&self.r#number_of_partial_bathrooms) {
				serialize_struct.serialize_field("numberOfPartialBathrooms", {
					struct SerializeWith<'a>(&'a Vec<NumberOfPartialBathroomsProperty>);
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
					&SerializeWith(&self.r#number_of_partial_bathrooms)
				})?;
			} else {
				serialize_struct.skip_field("numberOfPartialBathrooms")?;
			}
			if !Vec::is_empty(&self.r#number_of_rooms) {
				serialize_struct.serialize_field("numberOfRooms", {
					struct SerializeWith<'a>(&'a Vec<NumberOfRoomsProperty>);
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
					&SerializeWith(&self.r#number_of_rooms)
				})?;
			} else {
				serialize_struct.skip_field("numberOfRooms")?;
			}
			if !Vec::is_empty(&self.r#pets_allowed) {
				serialize_struct.serialize_field("petsAllowed", {
					struct SerializeWith<'a>(&'a Vec<PetsAllowedProperty>);
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
					&SerializeWith(&self.r#pets_allowed)
				})?;
			} else {
				serialize_struct.skip_field("petsAllowed")?;
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
	impl<'de> Deserialize<'de> for FloorPlan {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AmenityFeature,
				FloorSize,
				IsPlanForApartment,
				LayoutImage,
				NumberOfAccommodationUnits,
				NumberOfAvailableAccommodationUnits,
				NumberOfBathroomsTotal,
				NumberOfBedrooms,
				NumberOfFullBathrooms,
				NumberOfPartialBathrooms,
				NumberOfRooms,
				PetsAllowed,
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
						"amenityFeature" => Ok(Field::AmenityFeature),
						"floorSize" => Ok(Field::FloorSize),
						"isPlanForApartment" => Ok(Field::IsPlanForApartment),
						"layoutImage" => Ok(Field::LayoutImage),
						"numberOfAccommodationUnits" => Ok(Field::NumberOfAccommodationUnits),
						"numberOfAvailableAccommodationUnits" => {
							Ok(Field::NumberOfAvailableAccommodationUnits)
						}
						"numberOfBathroomsTotal" => Ok(Field::NumberOfBathroomsTotal),
						"numberOfBedrooms" => Ok(Field::NumberOfBedrooms),
						"numberOfFullBathrooms" => Ok(Field::NumberOfFullBathrooms),
						"numberOfPartialBathrooms" => Ok(Field::NumberOfPartialBathrooms),
						"numberOfRooms" => Ok(Field::NumberOfRooms),
						"petsAllowed" => Ok(Field::PetsAllowed),
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
						"id" | "type" => Ok(Field::Ignore),
						_ => Err(de::Error::unknown_field(value, FIELDS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"amenityFeature" => Ok(Field::AmenityFeature),
						b"floorSize" => Ok(Field::FloorSize),
						b"isPlanForApartment" => Ok(Field::IsPlanForApartment),
						b"layoutImage" => Ok(Field::LayoutImage),
						b"numberOfAccommodationUnits" => Ok(Field::NumberOfAccommodationUnits),
						b"numberOfAvailableAccommodationUnits" => {
							Ok(Field::NumberOfAvailableAccommodationUnits)
						}
						b"numberOfBathroomsTotal" => Ok(Field::NumberOfBathroomsTotal),
						b"numberOfBedrooms" => Ok(Field::NumberOfBedrooms),
						b"numberOfFullBathrooms" => Ok(Field::NumberOfFullBathrooms),
						b"numberOfPartialBathrooms" => Ok(Field::NumberOfPartialBathrooms),
						b"numberOfRooms" => Ok(Field::NumberOfRooms),
						b"petsAllowed" => Ok(Field::PetsAllowed),
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
						b"id" | b"type" => Ok(Field::Ignore),
						_ => {
							let value = &String::from_utf8_lossy(value);
							Err(de::Error::unknown_field(value, FIELDS))
						}
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
				type Value = FloorPlan;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema FloorPlan")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#amenity_feature_property = None;
					let mut r#floor_size_property = None;
					let mut r#is_plan_for_apartment_property = None;
					let mut r#layout_image_property = None;
					let mut r#number_of_accommodation_units_property = None;
					let mut r#number_of_available_accommodation_units_property = None;
					let mut r#number_of_bathrooms_total_property = None;
					let mut r#number_of_bedrooms_property = None;
					let mut r#number_of_full_bathrooms_property = None;
					let mut r#number_of_partial_bathrooms_property = None;
					let mut r#number_of_rooms_property = None;
					let mut r#pets_allowed_property = None;
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
							Field::AmenityFeature => {
								if r#amenity_feature_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"amenityFeature",
									));
								}
								r#amenity_feature_property = Some({
									struct DeserializeWith(Vec<AmenityFeatureProperty>);
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
							Field::FloorSize => {
								if r#floor_size_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"floorSize",
									));
								}
								r#floor_size_property = Some({
									struct DeserializeWith(Vec<FloorSizeProperty>);
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
							Field::IsPlanForApartment => {
								if r#is_plan_for_apartment_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isPlanForApartment",
									));
								}
								r#is_plan_for_apartment_property = Some({
									struct DeserializeWith(Vec<IsPlanForApartmentProperty>);
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
							Field::LayoutImage => {
								if r#layout_image_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"layoutImage",
									));
								}
								r#layout_image_property = Some({
									struct DeserializeWith(Vec<LayoutImageProperty>);
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
							Field::NumberOfAccommodationUnits => {
								if r#number_of_accommodation_units_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfAccommodationUnits",
									));
								}
								r#number_of_accommodation_units_property = Some({
									struct DeserializeWith(Vec<NumberOfAccommodationUnitsProperty>);
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
							Field::NumberOfAvailableAccommodationUnits => {
								if r#number_of_available_accommodation_units_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfAvailableAccommodationUnits",
									));
								}
								r#number_of_available_accommodation_units_property = Some({
									struct DeserializeWith(
										Vec<NumberOfAvailableAccommodationUnitsProperty>,
									);
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
							Field::NumberOfBathroomsTotal => {
								if r#number_of_bathrooms_total_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfBathroomsTotal",
									));
								}
								r#number_of_bathrooms_total_property = Some({
									struct DeserializeWith(Vec<NumberOfBathroomsTotalProperty>);
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
							Field::NumberOfBedrooms => {
								if r#number_of_bedrooms_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfBedrooms",
									));
								}
								r#number_of_bedrooms_property = Some({
									struct DeserializeWith(Vec<NumberOfBedroomsProperty>);
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
							Field::NumberOfFullBathrooms => {
								if r#number_of_full_bathrooms_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfFullBathrooms",
									));
								}
								r#number_of_full_bathrooms_property = Some({
									struct DeserializeWith(Vec<NumberOfFullBathroomsProperty>);
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
							Field::NumberOfPartialBathrooms => {
								if r#number_of_partial_bathrooms_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfPartialBathrooms",
									));
								}
								r#number_of_partial_bathrooms_property = Some({
									struct DeserializeWith(Vec<NumberOfPartialBathroomsProperty>);
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
							Field::NumberOfRooms => {
								if r#number_of_rooms_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfRooms",
									));
								}
								r#number_of_rooms_property = Some({
									struct DeserializeWith(Vec<NumberOfRoomsProperty>);
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
							Field::PetsAllowed => {
								if r#pets_allowed_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"petsAllowed",
									));
								}
								r#pets_allowed_property = Some({
									struct DeserializeWith(Vec<PetsAllowedProperty>);
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
							Field::Ignore => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(FloorPlan {
						r#amenity_feature: r#amenity_feature_property.unwrap_or_default(),
						r#floor_size: r#floor_size_property.unwrap_or_default(),
						r#is_plan_for_apartment: r#is_plan_for_apartment_property
							.unwrap_or_default(),
						r#layout_image: r#layout_image_property.unwrap_or_default(),
						r#number_of_accommodation_units: r#number_of_accommodation_units_property
							.unwrap_or_default(),
						r#number_of_available_accommodation_units:
							r#number_of_available_accommodation_units_property.unwrap_or_default(),
						r#number_of_bathrooms_total: r#number_of_bathrooms_total_property
							.unwrap_or_default(),
						r#number_of_bedrooms: r#number_of_bedrooms_property.unwrap_or_default(),
						r#number_of_full_bathrooms: r#number_of_full_bathrooms_property
							.unwrap_or_default(),
						r#number_of_partial_bathrooms: r#number_of_partial_bathrooms_property
							.unwrap_or_default(),
						r#number_of_rooms: r#number_of_rooms_property.unwrap_or_default(),
						r#pets_allowed: r#pets_allowed_property.unwrap_or_default(),
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
				"amenityFeature",
				"floorSize",
				"isPlanForApartment",
				"layoutImage",
				"numberOfAccommodationUnits",
				"numberOfAvailableAccommodationUnits",
				"numberOfBathroomsTotal",
				"numberOfBedrooms",
				"numberOfFullBathrooms",
				"numberOfPartialBathrooms",
				"numberOfRooms",
				"petsAllowed",
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
			deserializer.deserialize_struct("FloorPlan", FIELDS, ClassVisitor)
		}
	}
}
