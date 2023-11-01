use super::*;
/// <https://schema.org/FloorPlan>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct FloorPlan {
	#[cfg(any(
		any(
			feature = "additional-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	#[cfg(any(
		any(
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(
			feature = "amenity-feature-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#amenity_feature: Vec<AmenityFeatureProperty>,
	#[cfg(any(
		any(
			feature = "description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#description: Vec<DescriptionProperty>,
	#[cfg(any(
		any(
			feature = "disambiguating-description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	#[cfg(any(
		any(
			feature = "floor-size-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#floor_size: Vec<FloorSizeProperty>,
	#[cfg(any(
		any(
			feature = "identifier-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#identifier: Vec<IdentifierProperty>,
	#[cfg(any(
		any(feature = "image-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#image: Vec<ImageProperty>,
	#[cfg(any(
		any(
			feature = "is-plan-for-apartment-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#is_plan_for_apartment: Vec<IsPlanForApartmentProperty>,
	#[cfg(any(
		any(
			feature = "layout-image-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#layout_image: Vec<LayoutImageProperty>,
	#[cfg(any(
		any(
			feature = "main-entity-of-page-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "number-of-accommodation-units-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#number_of_accommodation_units: Vec<NumberOfAccommodationUnitsProperty>,
	#[cfg(any(
		any(
			feature = "number-of-available-accommodation-units-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#number_of_available_accommodation_units: Vec<NumberOfAvailableAccommodationUnitsProperty>,
	#[cfg(any(
		any(
			feature = "number-of-bathrooms-total-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#number_of_bathrooms_total: Vec<NumberOfBathroomsTotalProperty>,
	#[cfg(any(
		any(
			feature = "number-of-bedrooms-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#number_of_bedrooms: Vec<NumberOfBedroomsProperty>,
	#[cfg(any(
		any(
			feature = "number-of-full-bathrooms-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#number_of_full_bathrooms: Vec<NumberOfFullBathroomsProperty>,
	#[cfg(any(
		any(
			feature = "number-of-partial-bathrooms-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#number_of_partial_bathrooms: Vec<NumberOfPartialBathroomsProperty>,
	#[cfg(any(
		any(
			feature = "number-of-rooms-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#number_of_rooms: Vec<NumberOfRoomsProperty>,
	#[cfg(any(
		any(
			feature = "pets-allowed-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#pets_allowed: Vec<PetsAllowedProperty>,
	#[cfg(any(
		any(
			feature = "potential-action-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#potential_action: Vec<PotentialActionProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(
			feature = "subject-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#subject_of: Vec<SubjectOfProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
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
				if cfg!(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#additional_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#alternate_name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "amenity-feature-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#amenity_feature) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#description) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#disambiguating_description) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "floor-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#floor_size) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#identifier) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "image-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#image) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-plan-for-apartment-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_plan_for_apartment) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "layout-image-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#layout_image) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#main_entity_of_page) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-accommodation-units-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_accommodation_units) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-available-accommodation-units-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_available_accommodation_units) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-bathrooms-total-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_bathrooms_total) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-bedrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_bedrooms) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-full-bathrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_full_bathrooms) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-partial-bathrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_partial_bathrooms) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-rooms-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_rooms) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "pets-allowed-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#pets_allowed) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#potential_action) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#same_as) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#subject_of) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#url) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "FloorPlan", len)?;
			#[cfg(any(
				any(
					feature = "additional-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "alternate-name-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "amenity-feature-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "description-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "disambiguating-description-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "floor-size-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "identifier-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "image-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-plan-for-apartment-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "layout-image-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "main-entity-of-page-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "name-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "number-of-accommodation-units-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "number-of-available-accommodation-units-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "number-of-bathrooms-total-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "number-of-bedrooms-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "number-of-full-bathrooms-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "number-of-partial-bathrooms-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "number-of-rooms-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "pets-allowed-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "potential-action-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "same-as-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "subject-of-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "url-property-schema", feature = "general-schema-section"),
				doc
			))]
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
				#[cfg(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdditionalType,
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternateName,
				#[cfg(any(
					any(
						feature = "amenity-feature-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AmenityFeature,
				#[cfg(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Description,
				#[cfg(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DisambiguatingDescription,
				#[cfg(any(
					any(
						feature = "floor-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FloorSize,
				#[cfg(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Identifier,
				#[cfg(any(
					any(feature = "image-property-schema", feature = "general-schema-section"),
					doc
				))]
				Image,
				#[cfg(any(
					any(
						feature = "is-plan-for-apartment-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IsPlanForApartment,
				#[cfg(any(
					any(
						feature = "layout-image-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				LayoutImage,
				#[cfg(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MainEntityOfPage,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(
						feature = "number-of-accommodation-units-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NumberOfAccommodationUnits,
				#[cfg(any(
					any(
						feature = "number-of-available-accommodation-units-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NumberOfAvailableAccommodationUnits,
				#[cfg(any(
					any(
						feature = "number-of-bathrooms-total-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NumberOfBathroomsTotal,
				#[cfg(any(
					any(
						feature = "number-of-bedrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NumberOfBedrooms,
				#[cfg(any(
					any(
						feature = "number-of-full-bathrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NumberOfFullBathrooms,
				#[cfg(any(
					any(
						feature = "number-of-partial-bathrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NumberOfPartialBathrooms,
				#[cfg(any(
					any(
						feature = "number-of-rooms-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NumberOfRooms,
				#[cfg(any(
					any(
						feature = "pets-allowed-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PetsAllowed,
				#[cfg(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PotentialAction,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubjectOf,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
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
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalType" => Ok(Field::AdditionalType),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "amenity-feature-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"amenityFeature" => Ok(Field::AmenityFeature),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"description" => Ok(Field::Description),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						#[cfg(any(
							any(
								feature = "floor-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"floorSize" => Ok(Field::FloorSize),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"identifier" => Ok(Field::Identifier),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"image" => Ok(Field::Image),
						#[cfg(any(
							any(
								feature = "is-plan-for-apartment-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"isPlanForApartment" => Ok(Field::IsPlanForApartment),
						#[cfg(any(
							any(
								feature = "layout-image-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"layoutImage" => Ok(Field::LayoutImage),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"name" => Ok(Field::Name),
						#[cfg(any(
							any(
								feature = "number-of-accommodation-units-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"numberOfAccommodationUnits" => Ok(Field::NumberOfAccommodationUnits),
						#[cfg(any(
							any(
								feature = "number-of-available-accommodation-units-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"numberOfAvailableAccommodationUnits" => Ok(Field::NumberOfAvailableAccommodationUnits),
						#[cfg(any(
							any(
								feature = "number-of-bathrooms-total-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"numberOfBathroomsTotal" => Ok(Field::NumberOfBathroomsTotal),
						#[cfg(any(
							any(
								feature = "number-of-bedrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"numberOfBedrooms" => Ok(Field::NumberOfBedrooms),
						#[cfg(any(
							any(
								feature = "number-of-full-bathrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"numberOfFullBathrooms" => Ok(Field::NumberOfFullBathrooms),
						#[cfg(any(
							any(
								feature = "number-of-partial-bathrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"numberOfPartialBathrooms" => Ok(Field::NumberOfPartialBathrooms),
						#[cfg(any(
							any(
								feature = "number-of-rooms-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"numberOfRooms" => Ok(Field::NumberOfRooms),
						#[cfg(any(
							any(
								feature = "pets-allowed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"petsAllowed" => Ok(Field::PetsAllowed),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"subjectOf" => Ok(Field::SubjectOf),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"url" => Ok(Field::Url),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalType" => Ok(Field::AdditionalType),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "amenity-feature-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"amenityFeature" => Ok(Field::AmenityFeature),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"description" => Ok(Field::Description),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						#[cfg(any(
							any(
								feature = "floor-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"floorSize" => Ok(Field::FloorSize),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"identifier" => Ok(Field::Identifier),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"image" => Ok(Field::Image),
						#[cfg(any(
							any(
								feature = "is-plan-for-apartment-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"isPlanForApartment" => Ok(Field::IsPlanForApartment),
						#[cfg(any(
							any(
								feature = "layout-image-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"layoutImage" => Ok(Field::LayoutImage),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"name" => Ok(Field::Name),
						#[cfg(any(
							any(
								feature = "number-of-accommodation-units-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"numberOfAccommodationUnits" => Ok(Field::NumberOfAccommodationUnits),
						#[cfg(any(
							any(
								feature = "number-of-available-accommodation-units-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"numberOfAvailableAccommodationUnits" => Ok(Field::NumberOfAvailableAccommodationUnits),
						#[cfg(any(
							any(
								feature = "number-of-bathrooms-total-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"numberOfBathroomsTotal" => Ok(Field::NumberOfBathroomsTotal),
						#[cfg(any(
							any(
								feature = "number-of-bedrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"numberOfBedrooms" => Ok(Field::NumberOfBedrooms),
						#[cfg(any(
							any(
								feature = "number-of-full-bathrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"numberOfFullBathrooms" => Ok(Field::NumberOfFullBathrooms),
						#[cfg(any(
							any(
								feature = "number-of-partial-bathrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"numberOfPartialBathrooms" => Ok(Field::NumberOfPartialBathrooms),
						#[cfg(any(
							any(
								feature = "number-of-rooms-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"numberOfRooms" => Ok(Field::NumberOfRooms),
						#[cfg(any(
							any(
								feature = "pets-allowed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"petsAllowed" => Ok(Field::PetsAllowed),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sameAs" => Ok(Field::SameAs),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"subjectOf" => Ok(Field::SubjectOf),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
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
				type Value = FloorPlan;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema FloorPlan")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "additional-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#additional_type_property = None;
					#[cfg(any(
						any(
							feature = "alternate-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alternate_name_property = None;
					#[cfg(any(
						any(
							feature = "amenity-feature-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#amenity_feature_property = None;
					#[cfg(any(
						any(
							feature = "description-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#description_property = None;
					#[cfg(any(
						any(
							feature = "disambiguating-description-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#disambiguating_description_property = None;
					#[cfg(any(
						any(
							feature = "floor-size-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#floor_size_property = None;
					#[cfg(any(
						any(
							feature = "identifier-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#identifier_property = None;
					#[cfg(any(
						any(feature = "image-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#image_property = None;
					#[cfg(any(
						any(
							feature = "is-plan-for-apartment-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#is_plan_for_apartment_property = None;
					#[cfg(any(
						any(
							feature = "layout-image-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#layout_image_property = None;
					#[cfg(any(
						any(
							feature = "main-entity-of-page-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#main_entity_of_page_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "number-of-accommodation-units-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#number_of_accommodation_units_property = None;
					#[cfg(any(
						any(
							feature = "number-of-available-accommodation-units-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#number_of_available_accommodation_units_property = None;
					#[cfg(any(
						any(
							feature = "number-of-bathrooms-total-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#number_of_bathrooms_total_property = None;
					#[cfg(any(
						any(
							feature = "number-of-bedrooms-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#number_of_bedrooms_property = None;
					#[cfg(any(
						any(
							feature = "number-of-full-bathrooms-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#number_of_full_bathrooms_property = None;
					#[cfg(any(
						any(
							feature = "number-of-partial-bathrooms-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#number_of_partial_bathrooms_property = None;
					#[cfg(any(
						any(
							feature = "number-of-rooms-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#number_of_rooms_property = None;
					#[cfg(any(
						any(
							feature = "pets-allowed-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#pets_allowed_property = None;
					#[cfg(any(
						any(
							feature = "potential-action-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#potential_action_property = None;
					#[cfg(any(
						any(
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(
							feature = "subject-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#subject_of_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "additional-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "alternate-name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "amenity-feature-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "description-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "disambiguating-description-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "floor-size-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "identifier-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "image-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-plan-for-apartment-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "layout-image-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "main-entity-of-page-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "number-of-accommodation-units-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "number-of-available-accommodation-units-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "number-of-bathrooms-total-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "number-of-bedrooms-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "number-of-full-bathrooms-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "number-of-partial-bathrooms-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "number-of-rooms-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "pets-allowed-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "potential-action-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "same-as-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "subject-of-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
					Ok(FloorPlan {
						#[cfg(any(
							any(
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "amenity-feature-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#amenity_feature: r#amenity_feature_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#description: r#description_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "floor-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#floor_size: r#floor_size_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "identifier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#identifier: r#identifier_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "image-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#image: r#image_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-plan-for-apartment-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#is_plan_for_apartment: r#is_plan_for_apartment_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "layout-image-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#layout_image: r#layout_image_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#name: r#name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-accommodation-units-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#number_of_accommodation_units: r#number_of_accommodation_units_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-available-accommodation-units-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#number_of_available_accommodation_units:
							r#number_of_available_accommodation_units_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-bathrooms-total-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#number_of_bathrooms_total: r#number_of_bathrooms_total_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-bedrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#number_of_bedrooms: r#number_of_bedrooms_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-full-bathrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#number_of_full_bathrooms: r#number_of_full_bathrooms_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-partial-bathrooms-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#number_of_partial_bathrooms: r#number_of_partial_bathrooms_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-rooms-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#number_of_rooms: r#number_of_rooms_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "pets-allowed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#pets_allowed: r#pets_allowed_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#same_as: r#same_as_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "subject-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "additional-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalType",
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternateName",
				#[cfg(any(
					any(
						feature = "amenity-feature-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"amenityFeature",
				#[cfg(any(
					any(
						feature = "description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"description",
				#[cfg(any(
					any(
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"disambiguatingDescription",
				#[cfg(any(
					any(
						feature = "floor-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"floorSize",
				#[cfg(any(
					any(
						feature = "identifier-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"identifier",
				#[cfg(any(
					any(feature = "image-property-schema", feature = "general-schema-section"),
					doc
				))]
				"image",
				#[cfg(any(
					any(
						feature = "is-plan-for-apartment-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"isPlanForApartment",
				#[cfg(any(
					any(
						feature = "layout-image-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"layoutImage",
				#[cfg(any(
					any(
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mainEntityOfPage",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(
						feature = "number-of-accommodation-units-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"numberOfAccommodationUnits",
				#[cfg(any(
					any(
						feature = "number-of-available-accommodation-units-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"numberOfAvailableAccommodationUnits",
				#[cfg(any(
					any(
						feature = "number-of-bathrooms-total-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"numberOfBathroomsTotal",
				#[cfg(any(
					any(
						feature = "number-of-bedrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"numberOfBedrooms",
				#[cfg(any(
					any(
						feature = "number-of-full-bathrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"numberOfFullBathrooms",
				#[cfg(any(
					any(
						feature = "number-of-partial-bathrooms-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"numberOfPartialBathrooms",
				#[cfg(any(
					any(
						feature = "number-of-rooms-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"numberOfRooms",
				#[cfg(any(
					any(
						feature = "pets-allowed-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"petsAllowed",
				#[cfg(any(
					any(
						feature = "potential-action-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"potentialAction",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subjectOf",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
			];
			deserializer.deserialize_struct("FloorPlan", FIELDS, ClassVisitor)
		}
	}
}
