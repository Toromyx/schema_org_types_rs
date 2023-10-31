use super::*;
/// <https://schema.org/Observation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Observation {
	#[cfg(any(
		any(
			feature = "additional-property-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
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
			feature = "main-entity-of-page-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	#[cfg(any(
		any(
			feature = "margin-of-error-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#margin_of_error: Vec<MarginOfErrorProperty>,
	#[cfg(any(
		any(
			feature = "max-value-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#max_value: Vec<MaxValueProperty>,
	#[cfg(any(
		any(
			feature = "measured-property-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#measured_property: Vec<MeasuredPropertyProperty>,
	#[cfg(any(
		any(
			feature = "measurement-denominator-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#measurement_denominator: Vec<MeasurementDenominatorProperty>,
	#[cfg(any(
		any(
			feature = "measurement-method-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#measurement_method: Vec<MeasurementMethodProperty>,
	#[cfg(any(
		any(
			feature = "measurement-qualifier-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#measurement_qualifier: Vec<MeasurementQualifierProperty>,
	#[cfg(any(
		any(
			feature = "measurement-technique-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#measurement_technique: Vec<MeasurementTechniqueProperty>,
	#[cfg(any(
		any(
			feature = "min-value-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#min_value: Vec<MinValueProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "observation-about-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#observation_about: Vec<ObservationAboutProperty>,
	#[cfg(any(
		any(
			feature = "observation-date-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#observation_date: Vec<ObservationDateProperty>,
	#[cfg(any(
		any(
			feature = "observation-period-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#observation_period: Vec<ObservationPeriodProperty>,
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
		any(
			feature = "unit-code-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#unit_code: Vec<UnitCodeProperty>,
	#[cfg(any(
		any(
			feature = "unit-text-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#unit_text: Vec<UnitTextProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
	#[cfg(any(
		any(feature = "value-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#value: Vec<ValueProperty>,
	#[cfg(any(
		any(
			feature = "value-reference-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#value_reference: Vec<ValueReferenceProperty>,
	#[cfg(any(
		any(
			feature = "variable-measured-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#variable_measured: Vec<VariableMeasuredProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Observation {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#additional_property) as usize
				} else {
					0
				},
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
						feature = "margin-of-error-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#margin_of_error) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "max-value-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#max_value) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "measured-property-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#measured_property) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "measurement-denominator-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#measurement_denominator) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "measurement-method-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#measurement_method) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "measurement-qualifier-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#measurement_qualifier) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "measurement-technique-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#measurement_technique) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "min-value-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#min_value) as usize
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
						feature = "observation-about-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#observation_about) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "observation-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#observation_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "observation-period-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#observation_period) as usize
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
						feature = "unit-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#unit_code) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "unit-text-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#unit_text) as usize
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
				if cfg!(any(
					any(
						feature = "value-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#value) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "value-reference-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#value_reference) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "variable-measured-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#variable_measured) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "Observation", len)?;
			#[cfg(any(
				any(
					feature = "additional-property-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#additional_property) {
				serialize_struct.serialize_field("additionalProperty", {
					struct SerializeWith<'a>(&'a Vec<AdditionalPropertyProperty>);
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
					&SerializeWith(&self.r#additional_property)
				})?;
			} else {
				serialize_struct.skip_field("additionalProperty")?;
			}
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
				any(
					feature = "margin-of-error-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#margin_of_error) {
				serialize_struct.serialize_field("marginOfError", {
					struct SerializeWith<'a>(&'a Vec<MarginOfErrorProperty>);
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
					&SerializeWith(&self.r#margin_of_error)
				})?;
			} else {
				serialize_struct.skip_field("marginOfError")?;
			}
			#[cfg(any(
				any(
					feature = "max-value-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#max_value) {
				serialize_struct.serialize_field("maxValue", {
					struct SerializeWith<'a>(&'a Vec<MaxValueProperty>);
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
					&SerializeWith(&self.r#max_value)
				})?;
			} else {
				serialize_struct.skip_field("maxValue")?;
			}
			#[cfg(any(
				any(
					feature = "measured-property-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#measured_property) {
				serialize_struct.serialize_field("measuredProperty", {
					struct SerializeWith<'a>(&'a Vec<MeasuredPropertyProperty>);
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
					&SerializeWith(&self.r#measured_property)
				})?;
			} else {
				serialize_struct.skip_field("measuredProperty")?;
			}
			#[cfg(any(
				any(
					feature = "measurement-denominator-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#measurement_denominator) {
				serialize_struct.serialize_field("measurementDenominator", {
					struct SerializeWith<'a>(&'a Vec<MeasurementDenominatorProperty>);
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
					&SerializeWith(&self.r#measurement_denominator)
				})?;
			} else {
				serialize_struct.skip_field("measurementDenominator")?;
			}
			#[cfg(any(
				any(
					feature = "measurement-method-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#measurement_method) {
				serialize_struct.serialize_field("measurementMethod", {
					struct SerializeWith<'a>(&'a Vec<MeasurementMethodProperty>);
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
					&SerializeWith(&self.r#measurement_method)
				})?;
			} else {
				serialize_struct.skip_field("measurementMethod")?;
			}
			#[cfg(any(
				any(
					feature = "measurement-qualifier-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#measurement_qualifier) {
				serialize_struct.serialize_field("measurementQualifier", {
					struct SerializeWith<'a>(&'a Vec<MeasurementQualifierProperty>);
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
					&SerializeWith(&self.r#measurement_qualifier)
				})?;
			} else {
				serialize_struct.skip_field("measurementQualifier")?;
			}
			#[cfg(any(
				any(
					feature = "measurement-technique-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#measurement_technique) {
				serialize_struct.serialize_field("measurementTechnique", {
					struct SerializeWith<'a>(&'a Vec<MeasurementTechniqueProperty>);
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
					&SerializeWith(&self.r#measurement_technique)
				})?;
			} else {
				serialize_struct.skip_field("measurementTechnique")?;
			}
			#[cfg(any(
				any(
					feature = "min-value-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#min_value) {
				serialize_struct.serialize_field("minValue", {
					struct SerializeWith<'a>(&'a Vec<MinValueProperty>);
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
					&SerializeWith(&self.r#min_value)
				})?;
			} else {
				serialize_struct.skip_field("minValue")?;
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
					feature = "observation-about-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#observation_about) {
				serialize_struct.serialize_field("observationAbout", {
					struct SerializeWith<'a>(&'a Vec<ObservationAboutProperty>);
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
					&SerializeWith(&self.r#observation_about)
				})?;
			} else {
				serialize_struct.skip_field("observationAbout")?;
			}
			#[cfg(any(
				any(
					feature = "observation-date-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#observation_date) {
				serialize_struct.serialize_field("observationDate", {
					struct SerializeWith<'a>(&'a Vec<ObservationDateProperty>);
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
					&SerializeWith(&self.r#observation_date)
				})?;
			} else {
				serialize_struct.skip_field("observationDate")?;
			}
			#[cfg(any(
				any(
					feature = "observation-period-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#observation_period) {
				serialize_struct.serialize_field("observationPeriod", {
					struct SerializeWith<'a>(&'a Vec<ObservationPeriodProperty>);
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
					&SerializeWith(&self.r#observation_period)
				})?;
			} else {
				serialize_struct.skip_field("observationPeriod")?;
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
				any(
					feature = "unit-code-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#unit_code) {
				serialize_struct.serialize_field("unitCode", {
					struct SerializeWith<'a>(&'a Vec<UnitCodeProperty>);
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
					&SerializeWith(&self.r#unit_code)
				})?;
			} else {
				serialize_struct.skip_field("unitCode")?;
			}
			#[cfg(any(
				any(
					feature = "unit-text-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#unit_text) {
				serialize_struct.serialize_field("unitText", {
					struct SerializeWith<'a>(&'a Vec<UnitTextProperty>);
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
					&SerializeWith(&self.r#unit_text)
				})?;
			} else {
				serialize_struct.skip_field("unitText")?;
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
			#[cfg(any(
				any(feature = "value-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#value) {
				serialize_struct.serialize_field("value", {
					struct SerializeWith<'a>(&'a Vec<ValueProperty>);
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
					&SerializeWith(&self.r#value)
				})?;
			} else {
				serialize_struct.skip_field("value")?;
			}
			#[cfg(any(
				any(
					feature = "value-reference-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#value_reference) {
				serialize_struct.serialize_field("valueReference", {
					struct SerializeWith<'a>(&'a Vec<ValueReferenceProperty>);
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
					&SerializeWith(&self.r#value_reference)
				})?;
			} else {
				serialize_struct.skip_field("valueReference")?;
			}
			#[cfg(any(
				any(
					feature = "variable-measured-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#variable_measured) {
				serialize_struct.serialize_field("variableMeasured", {
					struct SerializeWith<'a>(&'a Vec<VariableMeasuredProperty>);
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
					&SerializeWith(&self.r#variable_measured)
				})?;
			} else {
				serialize_struct.skip_field("variableMeasured")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Observation {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdditionalProperty,
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
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MainEntityOfPage,
				#[cfg(any(
					any(
						feature = "margin-of-error-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MarginOfError,
				#[cfg(any(
					any(
						feature = "max-value-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MaxValue,
				#[cfg(any(
					any(
						feature = "measured-property-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MeasuredProperty,
				#[cfg(any(
					any(
						feature = "measurement-denominator-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MeasurementDenominator,
				#[cfg(any(
					any(
						feature = "measurement-method-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MeasurementMethod,
				#[cfg(any(
					any(
						feature = "measurement-qualifier-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MeasurementQualifier,
				#[cfg(any(
					any(
						feature = "measurement-technique-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MeasurementTechnique,
				#[cfg(any(
					any(
						feature = "min-value-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MinValue,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(
						feature = "observation-about-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ObservationAbout,
				#[cfg(any(
					any(
						feature = "observation-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ObservationDate,
				#[cfg(any(
					any(
						feature = "observation-period-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ObservationPeriod,
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
					any(
						feature = "unit-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				UnitCode,
				#[cfg(any(
					any(
						feature = "unit-text-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				UnitText,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
				#[cfg(any(
					any(feature = "value-property-schema", feature = "general-schema-section"),
					doc
				))]
				Value,
				#[cfg(any(
					any(
						feature = "value-reference-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ValueReference,
				#[cfg(any(
					any(
						feature = "variable-measured-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				VariableMeasured,
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
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalProperty" => Ok(Field::AdditionalProperty),
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
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "margin-of-error-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"marginOfError" => Ok(Field::MarginOfError),
						#[cfg(any(
							any(
								feature = "max-value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"maxValue" => Ok(Field::MaxValue),
						#[cfg(any(
							any(
								feature = "measured-property-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"measuredProperty" => Ok(Field::MeasuredProperty),
						#[cfg(any(
							any(
								feature = "measurement-denominator-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"measurementDenominator" => Ok(Field::MeasurementDenominator),
						#[cfg(any(
							any(
								feature = "measurement-method-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"measurementMethod" => Ok(Field::MeasurementMethod),
						#[cfg(any(
							any(
								feature = "measurement-qualifier-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"measurementQualifier" => Ok(Field::MeasurementQualifier),
						#[cfg(any(
							any(
								feature = "measurement-technique-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"measurementTechnique" => Ok(Field::MeasurementTechnique),
						#[cfg(any(
							any(
								feature = "min-value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"minValue" => Ok(Field::MinValue),
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
								feature = "observation-about-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"observationAbout" => Ok(Field::ObservationAbout),
						#[cfg(any(
							any(
								feature = "observation-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"observationDate" => Ok(Field::ObservationDate),
						#[cfg(any(
							any(
								feature = "observation-period-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"observationPeriod" => Ok(Field::ObservationPeriod),
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
								feature = "unit-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"unitCode" => Ok(Field::UnitCode),
						#[cfg(any(
							any(
								feature = "unit-text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"unitText" => Ok(Field::UnitText),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"url" => Ok(Field::Url),
						#[cfg(any(
							any(
								feature = "value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"value" => Ok(Field::Value),
						#[cfg(any(
							any(
								feature = "value-reference-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"valueReference" => Ok(Field::ValueReference),
						#[cfg(any(
							any(
								feature = "variable-measured-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"variableMeasured" => Ok(Field::VariableMeasured),
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
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalProperty" => Ok(Field::AdditionalProperty),
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
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "margin-of-error-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"marginOfError" => Ok(Field::MarginOfError),
						#[cfg(any(
							any(
								feature = "max-value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"maxValue" => Ok(Field::MaxValue),
						#[cfg(any(
							any(
								feature = "measured-property-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"measuredProperty" => Ok(Field::MeasuredProperty),
						#[cfg(any(
							any(
								feature = "measurement-denominator-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"measurementDenominator" => Ok(Field::MeasurementDenominator),
						#[cfg(any(
							any(
								feature = "measurement-method-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"measurementMethod" => Ok(Field::MeasurementMethod),
						#[cfg(any(
							any(
								feature = "measurement-qualifier-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"measurementQualifier" => Ok(Field::MeasurementQualifier),
						#[cfg(any(
							any(
								feature = "measurement-technique-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"measurementTechnique" => Ok(Field::MeasurementTechnique),
						#[cfg(any(
							any(
								feature = "min-value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"minValue" => Ok(Field::MinValue),
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
								feature = "observation-about-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"observationAbout" => Ok(Field::ObservationAbout),
						#[cfg(any(
							any(
								feature = "observation-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"observationDate" => Ok(Field::ObservationDate),
						#[cfg(any(
							any(
								feature = "observation-period-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"observationPeriod" => Ok(Field::ObservationPeriod),
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
								feature = "unit-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"unitCode" => Ok(Field::UnitCode),
						#[cfg(any(
							any(
								feature = "unit-text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"unitText" => Ok(Field::UnitText),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"url" => Ok(Field::Url),
						#[cfg(any(
							any(
								feature = "value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"value" => Ok(Field::Value),
						#[cfg(any(
							any(
								feature = "value-reference-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"valueReference" => Ok(Field::ValueReference),
						#[cfg(any(
							any(
								feature = "variable-measured-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"variableMeasured" => Ok(Field::VariableMeasured),
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
				type Value = Observation;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Observation")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "additional-property-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#additional_property_property = None;
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
							feature = "main-entity-of-page-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#main_entity_of_page_property = None;
					#[cfg(any(
						any(
							feature = "margin-of-error-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#margin_of_error_property = None;
					#[cfg(any(
						any(
							feature = "max-value-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#max_value_property = None;
					#[cfg(any(
						any(
							feature = "measured-property-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#measured_property_property = None;
					#[cfg(any(
						any(
							feature = "measurement-denominator-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#measurement_denominator_property = None;
					#[cfg(any(
						any(
							feature = "measurement-method-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#measurement_method_property = None;
					#[cfg(any(
						any(
							feature = "measurement-qualifier-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#measurement_qualifier_property = None;
					#[cfg(any(
						any(
							feature = "measurement-technique-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#measurement_technique_property = None;
					#[cfg(any(
						any(
							feature = "min-value-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#min_value_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "observation-about-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#observation_about_property = None;
					#[cfg(any(
						any(
							feature = "observation-date-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#observation_date_property = None;
					#[cfg(any(
						any(
							feature = "observation-period-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#observation_period_property = None;
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
						any(
							feature = "unit-code-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#unit_code_property = None;
					#[cfg(any(
						any(
							feature = "unit-text-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#unit_text_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					#[cfg(any(
						any(feature = "value-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#value_property = None;
					#[cfg(any(
						any(
							feature = "value-reference-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#value_reference_property = None;
					#[cfg(any(
						any(
							feature = "variable-measured-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#variable_measured_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "additional-property-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AdditionalProperty => {
								if r#additional_property_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalProperty",
									));
								}
								r#additional_property_property = Some({
									struct DeserializeWith(Vec<AdditionalPropertyProperty>);
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
									feature = "margin-of-error-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MarginOfError => {
								if r#margin_of_error_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"marginOfError",
									));
								}
								r#margin_of_error_property = Some({
									struct DeserializeWith(Vec<MarginOfErrorProperty>);
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
									feature = "max-value-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MaxValue => {
								if r#max_value_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maxValue",
									));
								}
								r#max_value_property = Some({
									struct DeserializeWith(Vec<MaxValueProperty>);
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
									feature = "measured-property-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MeasuredProperty => {
								if r#measured_property_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"measuredProperty",
									));
								}
								r#measured_property_property = Some({
									struct DeserializeWith(Vec<MeasuredPropertyProperty>);
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
									feature = "measurement-denominator-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MeasurementDenominator => {
								if r#measurement_denominator_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"measurementDenominator",
									));
								}
								r#measurement_denominator_property = Some({
									struct DeserializeWith(Vec<MeasurementDenominatorProperty>);
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
									feature = "measurement-method-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MeasurementMethod => {
								if r#measurement_method_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"measurementMethod",
									));
								}
								r#measurement_method_property = Some({
									struct DeserializeWith(Vec<MeasurementMethodProperty>);
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
									feature = "measurement-qualifier-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MeasurementQualifier => {
								if r#measurement_qualifier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"measurementQualifier",
									));
								}
								r#measurement_qualifier_property = Some({
									struct DeserializeWith(Vec<MeasurementQualifierProperty>);
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
									feature = "measurement-technique-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MeasurementTechnique => {
								if r#measurement_technique_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"measurementTechnique",
									));
								}
								r#measurement_technique_property = Some({
									struct DeserializeWith(Vec<MeasurementTechniqueProperty>);
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
									feature = "min-value-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MinValue => {
								if r#min_value_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"minValue",
									));
								}
								r#min_value_property = Some({
									struct DeserializeWith(Vec<MinValueProperty>);
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
									feature = "observation-about-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ObservationAbout => {
								if r#observation_about_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"observationAbout",
									));
								}
								r#observation_about_property = Some({
									struct DeserializeWith(Vec<ObservationAboutProperty>);
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
									feature = "observation-date-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ObservationDate => {
								if r#observation_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"observationDate",
									));
								}
								r#observation_date_property = Some({
									struct DeserializeWith(Vec<ObservationDateProperty>);
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
									feature = "observation-period-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ObservationPeriod => {
								if r#observation_period_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"observationPeriod",
									));
								}
								r#observation_period_property = Some({
									struct DeserializeWith(Vec<ObservationPeriodProperty>);
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
									feature = "unit-code-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::UnitCode => {
								if r#unit_code_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"unitCode",
									));
								}
								r#unit_code_property = Some({
									struct DeserializeWith(Vec<UnitCodeProperty>);
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
									feature = "unit-text-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::UnitText => {
								if r#unit_text_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"unitText",
									));
								}
								r#unit_text_property = Some({
									struct DeserializeWith(Vec<UnitTextProperty>);
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
							#[cfg(any(
								any(
									feature = "value-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Value => {
								if r#value_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("value"));
								}
								r#value_property = Some({
									struct DeserializeWith(Vec<ValueProperty>);
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
									feature = "value-reference-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ValueReference => {
								if r#value_reference_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"valueReference",
									));
								}
								r#value_reference_property = Some({
									struct DeserializeWith(Vec<ValueReferenceProperty>);
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
									feature = "variable-measured-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::VariableMeasured => {
								if r#variable_measured_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"variableMeasured",
									));
								}
								r#variable_measured_property = Some({
									struct DeserializeWith(Vec<VariableMeasuredProperty>);
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
					Ok(Observation {
						#[cfg(any(
							any(
								feature = "additional-property-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#additional_property: r#additional_property_property.unwrap_or_default(),
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
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "margin-of-error-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#margin_of_error: r#margin_of_error_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "max-value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#max_value: r#max_value_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "measured-property-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#measured_property: r#measured_property_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "measurement-denominator-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#measurement_denominator: r#measurement_denominator_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "measurement-method-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#measurement_method: r#measurement_method_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "measurement-qualifier-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#measurement_qualifier: r#measurement_qualifier_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "measurement-technique-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#measurement_technique: r#measurement_technique_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "min-value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#min_value: r#min_value_property.unwrap_or_default(),
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
								feature = "observation-about-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#observation_about: r#observation_about_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "observation-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#observation_date: r#observation_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "observation-period-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#observation_period: r#observation_period_property.unwrap_or_default(),
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
								feature = "unit-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#unit_code: r#unit_code_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "unit-text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#unit_text: r#unit_text_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#url: r#url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "value-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#value: r#value_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "value-reference-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#value_reference: r#value_reference_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "variable-measured-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#variable_measured: r#variable_measured_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "additional-property-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalProperty",
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
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mainEntityOfPage",
				#[cfg(any(
					any(
						feature = "margin-of-error-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"marginOfError",
				#[cfg(any(
					any(
						feature = "max-value-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"maxValue",
				#[cfg(any(
					any(
						feature = "measured-property-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"measuredProperty",
				#[cfg(any(
					any(
						feature = "measurement-denominator-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"measurementDenominator",
				#[cfg(any(
					any(
						feature = "measurement-method-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"measurementMethod",
				#[cfg(any(
					any(
						feature = "measurement-qualifier-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"measurementQualifier",
				#[cfg(any(
					any(
						feature = "measurement-technique-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"measurementTechnique",
				#[cfg(any(
					any(
						feature = "min-value-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"minValue",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(
						feature = "observation-about-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"observationAbout",
				#[cfg(any(
					any(
						feature = "observation-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"observationDate",
				#[cfg(any(
					any(
						feature = "observation-period-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"observationPeriod",
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
					any(
						feature = "unit-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"unitCode",
				#[cfg(any(
					any(
						feature = "unit-text-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"unitText",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
				#[cfg(any(
					any(feature = "value-property-schema", feature = "general-schema-section"),
					doc
				))]
				"value",
				#[cfg(any(
					any(
						feature = "value-reference-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"valueReference",
				#[cfg(any(
					any(
						feature = "variable-measured-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"variableMeasured",
			];
			deserializer.deserialize_struct("Observation", FIELDS, ClassVisitor)
		}
	}
}
