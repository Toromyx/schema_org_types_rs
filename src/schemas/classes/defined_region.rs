use super::*;
/// <https://schema.org/DefinedRegion>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DefinedRegion {
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
			feature = "address-country-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "addressCountry"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#address_country: Vec<AddressCountryProperty>,
	#[cfg(any(
		any(
			feature = "address-region-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "addressRegion"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#address_region: Vec<AddressRegionProperty>,
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
			feature = "postal-code-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "postalCode"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#postal_code: Vec<PostalCodeProperty>,
	#[cfg(any(
		any(
			feature = "postal-code-prefix-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "postalCodePrefix"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#postal_code_prefix: Vec<PostalCodePrefixProperty>,
	#[cfg(any(
		any(
			feature = "postal-code-range-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "postalCodeRange"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#postal_code_range: Vec<PostalCodeRangeProperty>,
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
