use super::*;
/// <https://schema.org/MedicalAudience>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MedicalAudience {
	#[cfg(any(
		any(
			feature = "additional-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	#[cfg(any(
		any(
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(
			feature = "audience-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "audienceType"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#audience_type: Vec<AudienceTypeProperty>,
	#[cfg(any(
		any(
			feature = "description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "description"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#description: Vec<DescriptionProperty>,
	#[cfg(any(
		any(
			feature = "disambiguating-description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	#[cfg(any(
		any(
			feature = "geographic-area-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "geographicArea"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#geographic_area: Vec<GeographicAreaProperty>,
	#[cfg(any(
		any(
			feature = "health-condition-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "healthCondition"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#health_condition: Vec<HealthConditionProperty>,
	#[cfg(any(
		any(
			feature = "identifier-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "identifier"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#identifier: Vec<IdentifierProperty>,
	#[cfg(any(
		any(feature = "image-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "image"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#image: Vec<ImageProperty>,
	#[cfg(any(
		any(
			feature = "main-entity-of-page-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "name"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "potential-action-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#potential_action: Vec<PotentialActionProperty>,
	#[cfg(any(
		any(
			feature = "required-gender-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "requiredGender"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#required_gender: Vec<RequiredGenderProperty>,
	#[cfg(any(
		any(
			feature = "required-max-age-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "requiredMaxAge"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#required_max_age: Vec<RequiredMaxAgeProperty>,
	#[cfg(any(
		any(
			feature = "required-min-age-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "requiredMinAge"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#required_min_age: Vec<RequiredMinAgeProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(
			feature = "subject-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#subject_of: Vec<SubjectOfProperty>,
	#[cfg(any(
		any(
			feature = "suggested-age-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "suggestedAge"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#suggested_age: Vec<SuggestedAgeProperty>,
	#[cfg(any(
		any(
			feature = "suggested-gender-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "suggestedGender"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#suggested_gender: Vec<SuggestedGenderProperty>,
	#[cfg(any(
		any(
			feature = "suggested-max-age-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "suggestedMaxAge"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#suggested_max_age: Vec<SuggestedMaxAgeProperty>,
	#[cfg(any(
		any(
			feature = "suggested-measurement-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "suggestedMeasurement"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#suggested_measurement: Vec<SuggestedMeasurementProperty>,
	#[cfg(any(
		any(
			feature = "suggested-min-age-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "suggestedMinAge"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#suggested_min_age: Vec<SuggestedMinAgeProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "url"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#url: Vec<UrlProperty>,
}
