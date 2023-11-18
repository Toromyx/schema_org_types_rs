use super::*;
/// <https://schema.org/Observation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Observation {
	pub r#margin_of_error: Vec<MarginOfErrorProperty>,
	pub r#measured_property: Vec<MeasuredPropertyProperty>,
	pub r#measurement_denominator: Vec<MeasurementDenominatorProperty>,
	pub r#measurement_method: Vec<MeasurementMethodProperty>,
	pub r#measurement_qualifier: Vec<MeasurementQualifierProperty>,
	pub r#measurement_technique: Vec<MeasurementTechniqueProperty>,
	pub r#observation_about: Vec<ObservationAboutProperty>,
	pub r#observation_date: Vec<ObservationDateProperty>,
	pub r#observation_period: Vec<ObservationPeriodProperty>,
	pub r#variable_measured: Vec<VariableMeasuredProperty>,
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	pub r#max_value: Vec<MaxValueProperty>,
	pub r#min_value: Vec<MinValueProperty>,
	pub r#unit_code: Vec<UnitCodeProperty>,
	pub r#unit_text: Vec<UnitTextProperty>,
	pub r#value: Vec<ValueProperty>,
	pub r#value_reference: Vec<ValueReferenceProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#url: Vec<UrlProperty>,
}
pub trait ObservationTrait {
	fn get_margin_of_error(&self) -> &[MarginOfErrorProperty];
	fn take_margin_of_error(&mut self) -> Vec<MarginOfErrorProperty>;
	fn get_measured_property(&self) -> &[MeasuredPropertyProperty];
	fn take_measured_property(&mut self) -> Vec<MeasuredPropertyProperty>;
	fn get_measurement_denominator(&self) -> &[MeasurementDenominatorProperty];
	fn take_measurement_denominator(&mut self) -> Vec<MeasurementDenominatorProperty>;
	fn get_measurement_method(&self) -> &[MeasurementMethodProperty];
	fn take_measurement_method(&mut self) -> Vec<MeasurementMethodProperty>;
	fn get_measurement_qualifier(&self) -> &[MeasurementQualifierProperty];
	fn take_measurement_qualifier(&mut self) -> Vec<MeasurementQualifierProperty>;
	fn get_measurement_technique(&self) -> &[MeasurementTechniqueProperty];
	fn take_measurement_technique(&mut self) -> Vec<MeasurementTechniqueProperty>;
	fn get_observation_about(&self) -> &[ObservationAboutProperty];
	fn take_observation_about(&mut self) -> Vec<ObservationAboutProperty>;
	fn get_observation_date(&self) -> &[ObservationDateProperty];
	fn take_observation_date(&mut self) -> Vec<ObservationDateProperty>;
	fn get_observation_period(&self) -> &[ObservationPeriodProperty];
	fn take_observation_period(&mut self) -> Vec<ObservationPeriodProperty>;
	fn get_variable_measured(&self) -> &[VariableMeasuredProperty];
	fn take_variable_measured(&mut self) -> Vec<VariableMeasuredProperty>;
}
impl ObservationTrait for Observation {
	fn get_margin_of_error(&self) -> &[MarginOfErrorProperty] {
		self.r#margin_of_error.as_slice()
	}
	fn take_margin_of_error(&mut self) -> Vec<MarginOfErrorProperty> {
		std::mem::take(&mut self.r#margin_of_error)
	}
	fn get_measured_property(&self) -> &[MeasuredPropertyProperty] {
		self.r#measured_property.as_slice()
	}
	fn take_measured_property(&mut self) -> Vec<MeasuredPropertyProperty> {
		std::mem::take(&mut self.r#measured_property)
	}
	fn get_measurement_denominator(&self) -> &[MeasurementDenominatorProperty] {
		self.r#measurement_denominator.as_slice()
	}
	fn take_measurement_denominator(&mut self) -> Vec<MeasurementDenominatorProperty> {
		std::mem::take(&mut self.r#measurement_denominator)
	}
	fn get_measurement_method(&self) -> &[MeasurementMethodProperty] {
		self.r#measurement_method.as_slice()
	}
	fn take_measurement_method(&mut self) -> Vec<MeasurementMethodProperty> {
		std::mem::take(&mut self.r#measurement_method)
	}
	fn get_measurement_qualifier(&self) -> &[MeasurementQualifierProperty] {
		self.r#measurement_qualifier.as_slice()
	}
	fn take_measurement_qualifier(&mut self) -> Vec<MeasurementQualifierProperty> {
		std::mem::take(&mut self.r#measurement_qualifier)
	}
	fn get_measurement_technique(&self) -> &[MeasurementTechniqueProperty] {
		self.r#measurement_technique.as_slice()
	}
	fn take_measurement_technique(&mut self) -> Vec<MeasurementTechniqueProperty> {
		std::mem::take(&mut self.r#measurement_technique)
	}
	fn get_observation_about(&self) -> &[ObservationAboutProperty] {
		self.r#observation_about.as_slice()
	}
	fn take_observation_about(&mut self) -> Vec<ObservationAboutProperty> {
		std::mem::take(&mut self.r#observation_about)
	}
	fn get_observation_date(&self) -> &[ObservationDateProperty] {
		self.r#observation_date.as_slice()
	}
	fn take_observation_date(&mut self) -> Vec<ObservationDateProperty> {
		std::mem::take(&mut self.r#observation_date)
	}
	fn get_observation_period(&self) -> &[ObservationPeriodProperty] {
		self.r#observation_period.as_slice()
	}
	fn take_observation_period(&mut self) -> Vec<ObservationPeriodProperty> {
		std::mem::take(&mut self.r#observation_period)
	}
	fn get_variable_measured(&self) -> &[VariableMeasuredProperty] {
		self.r#variable_measured.as_slice()
	}
	fn take_variable_measured(&mut self) -> Vec<VariableMeasuredProperty> {
		std::mem::take(&mut self.r#variable_measured)
	}
}
impl QuantitativeValueTrait for Observation {
	fn get_additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn take_additional_property(&mut self) -> Vec<AdditionalPropertyProperty> {
		std::mem::take(&mut self.r#additional_property)
	}
	fn get_max_value(&self) -> &[MaxValueProperty] {
		self.r#max_value.as_slice()
	}
	fn take_max_value(&mut self) -> Vec<MaxValueProperty> {
		std::mem::take(&mut self.r#max_value)
	}
	fn get_min_value(&self) -> &[MinValueProperty] {
		self.r#min_value.as_slice()
	}
	fn take_min_value(&mut self) -> Vec<MinValueProperty> {
		std::mem::take(&mut self.r#min_value)
	}
	fn get_unit_code(&self) -> &[UnitCodeProperty] {
		self.r#unit_code.as_slice()
	}
	fn take_unit_code(&mut self) -> Vec<UnitCodeProperty> {
		std::mem::take(&mut self.r#unit_code)
	}
	fn get_unit_text(&self) -> &[UnitTextProperty] {
		self.r#unit_text.as_slice()
	}
	fn take_unit_text(&mut self) -> Vec<UnitTextProperty> {
		std::mem::take(&mut self.r#unit_text)
	}
	fn get_value(&self) -> &[ValueProperty] {
		self.r#value.as_slice()
	}
	fn take_value(&mut self) -> Vec<ValueProperty> {
		std::mem::take(&mut self.r#value)
	}
	fn get_value_reference(&self) -> &[ValueReferenceProperty] {
		self.r#value_reference.as_slice()
	}
	fn take_value_reference(&mut self) -> Vec<ValueReferenceProperty> {
		std::mem::take(&mut self.r#value_reference)
	}
}
impl StructuredValueTrait for Observation {}
impl ThingTrait for Observation {
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
	impl Serialize for Observation {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#margin_of_error) as usize,
				!Vec::is_empty(&self.r#measured_property) as usize,
				!Vec::is_empty(&self.r#measurement_denominator) as usize,
				!Vec::is_empty(&self.r#measurement_method) as usize,
				!Vec::is_empty(&self.r#measurement_qualifier) as usize,
				!Vec::is_empty(&self.r#measurement_technique) as usize,
				!Vec::is_empty(&self.r#observation_about) as usize,
				!Vec::is_empty(&self.r#observation_date) as usize,
				!Vec::is_empty(&self.r#observation_period) as usize,
				!Vec::is_empty(&self.r#variable_measured) as usize,
				!Vec::is_empty(&self.r#additional_property) as usize,
				!Vec::is_empty(&self.r#max_value) as usize,
				!Vec::is_empty(&self.r#min_value) as usize,
				!Vec::is_empty(&self.r#unit_code) as usize,
				!Vec::is_empty(&self.r#unit_text) as usize,
				!Vec::is_empty(&self.r#value) as usize,
				!Vec::is_empty(&self.r#value_reference) as usize,
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
				Serializer::serialize_struct(serializer, "Observation", len)?;
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
	impl<'de> Deserialize<'de> for Observation {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				MarginOfError,
				MeasuredProperty,
				MeasurementDenominator,
				MeasurementMethod,
				MeasurementQualifier,
				MeasurementTechnique,
				ObservationAbout,
				ObservationDate,
				ObservationPeriod,
				VariableMeasured,
				AdditionalProperty,
				MaxValue,
				MinValue,
				UnitCode,
				UnitText,
				Value,
				ValueReference,
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
						"marginOfError" => Ok(Field::MarginOfError),
						"measuredProperty" => Ok(Field::MeasuredProperty),
						"measurementDenominator" => Ok(Field::MeasurementDenominator),
						"measurementMethod" => Ok(Field::MeasurementMethod),
						"measurementQualifier" => Ok(Field::MeasurementQualifier),
						"measurementTechnique" => Ok(Field::MeasurementTechnique),
						"observationAbout" => Ok(Field::ObservationAbout),
						"observationDate" => Ok(Field::ObservationDate),
						"observationPeriod" => Ok(Field::ObservationPeriod),
						"variableMeasured" => Ok(Field::VariableMeasured),
						"additionalProperty" => Ok(Field::AdditionalProperty),
						"maxValue" => Ok(Field::MaxValue),
						"minValue" => Ok(Field::MinValue),
						"unitCode" => Ok(Field::UnitCode),
						"unitText" => Ok(Field::UnitText),
						"value" => Ok(Field::Value),
						"valueReference" => Ok(Field::ValueReference),
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
						b"marginOfError" => Ok(Field::MarginOfError),
						b"measuredProperty" => Ok(Field::MeasuredProperty),
						b"measurementDenominator" => Ok(Field::MeasurementDenominator),
						b"measurementMethod" => Ok(Field::MeasurementMethod),
						b"measurementQualifier" => Ok(Field::MeasurementQualifier),
						b"measurementTechnique" => Ok(Field::MeasurementTechnique),
						b"observationAbout" => Ok(Field::ObservationAbout),
						b"observationDate" => Ok(Field::ObservationDate),
						b"observationPeriod" => Ok(Field::ObservationPeriod),
						b"variableMeasured" => Ok(Field::VariableMeasured),
						b"additionalProperty" => Ok(Field::AdditionalProperty),
						b"maxValue" => Ok(Field::MaxValue),
						b"minValue" => Ok(Field::MinValue),
						b"unitCode" => Ok(Field::UnitCode),
						b"unitText" => Ok(Field::UnitText),
						b"value" => Ok(Field::Value),
						b"valueReference" => Ok(Field::ValueReference),
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
				type Value = Observation;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Observation")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#margin_of_error_property = None;
					let mut r#measured_property_property = None;
					let mut r#measurement_denominator_property = None;
					let mut r#measurement_method_property = None;
					let mut r#measurement_qualifier_property = None;
					let mut r#measurement_technique_property = None;
					let mut r#observation_about_property = None;
					let mut r#observation_date_property = None;
					let mut r#observation_period_property = None;
					let mut r#variable_measured_property = None;
					let mut r#additional_property_property = None;
					let mut r#max_value_property = None;
					let mut r#min_value_property = None;
					let mut r#unit_code_property = None;
					let mut r#unit_text_property = None;
					let mut r#value_property = None;
					let mut r#value_reference_property = None;
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
					Ok(Observation {
						r#margin_of_error: r#margin_of_error_property.unwrap_or_default(),
						r#measured_property: r#measured_property_property.unwrap_or_default(),
						r#measurement_denominator: r#measurement_denominator_property
							.unwrap_or_default(),
						r#measurement_method: r#measurement_method_property.unwrap_or_default(),
						r#measurement_qualifier: r#measurement_qualifier_property
							.unwrap_or_default(),
						r#measurement_technique: r#measurement_technique_property
							.unwrap_or_default(),
						r#observation_about: r#observation_about_property.unwrap_or_default(),
						r#observation_date: r#observation_date_property.unwrap_or_default(),
						r#observation_period: r#observation_period_property.unwrap_or_default(),
						r#variable_measured: r#variable_measured_property.unwrap_or_default(),
						r#additional_property: r#additional_property_property.unwrap_or_default(),
						r#max_value: r#max_value_property.unwrap_or_default(),
						r#min_value: r#min_value_property.unwrap_or_default(),
						r#unit_code: r#unit_code_property.unwrap_or_default(),
						r#unit_text: r#unit_text_property.unwrap_or_default(),
						r#value: r#value_property.unwrap_or_default(),
						r#value_reference: r#value_reference_property.unwrap_or_default(),
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
				"marginOfError",
				"measuredProperty",
				"measurementDenominator",
				"measurementMethod",
				"measurementQualifier",
				"measurementTechnique",
				"observationAbout",
				"observationDate",
				"observationPeriod",
				"variableMeasured",
				"additionalProperty",
				"maxValue",
				"minValue",
				"unitCode",
				"unitText",
				"value",
				"valueReference",
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
			deserializer.deserialize_struct("Observation", FIELDS, ClassVisitor)
		}
	}
}
