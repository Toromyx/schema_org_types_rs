use super::*;
/// <https://schema.org/Patient>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Patient {
	#[cfg(any(
		any(
			feature = "additional-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "additionalName"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#additional_name: Vec<AdditionalNameProperty>,
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
			feature = "address-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "address"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#address: Vec<AddressProperty>,
	#[cfg(any(
		any(
			feature = "affiliation-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "affiliation"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#affiliation: Vec<AffiliationProperty>,
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
			feature = "alumni-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "alumniOf"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#alumni_of: Vec<AlumniOfProperty>,
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
		any(feature = "award-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "award"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#award: Vec<AwardProperty>,
	#[cfg(any(
		any(feature = "awards-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "awards"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#awards: Vec<AwardsProperty>,
	#[cfg(any(
		any(
			feature = "birth-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "birthDate"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#birth_date: Vec<BirthDateProperty>,
	#[cfg(any(
		any(
			feature = "birth-place-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "birthPlace"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#birth_place: Vec<BirthPlaceProperty>,
	#[cfg(any(
		any(feature = "brand-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "brand"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#brand: Vec<BrandProperty>,
	#[cfg(any(
		any(
			feature = "call-sign-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "callSign"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#call_sign: Vec<CallSignProperty>,
	#[cfg(any(
		any(
			feature = "children-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "children"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#children: Vec<ChildrenProperty>,
	#[cfg(any(
		any(
			feature = "colleague-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "colleague"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#colleague: Vec<ColleagueProperty>,
	#[cfg(any(
		any(
			feature = "colleagues-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "colleagues"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#colleagues: Vec<ColleaguesProperty>,
	#[cfg(any(
		any(
			feature = "contact-point-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "contactPoint"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#contact_point: Vec<ContactPointProperty>,
	#[cfg(any(
		any(
			feature = "contact-points-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "contactPoints"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#contact_points: Vec<ContactPointsProperty>,
	#[cfg(any(
		any(
			feature = "death-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "deathDate"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#death_date: Vec<DeathDateProperty>,
	#[cfg(any(
		any(
			feature = "death-place-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "deathPlace"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#death_place: Vec<DeathPlaceProperty>,
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
			feature = "diagnosis-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "diagnosis"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#diagnosis: Vec<DiagnosisProperty>,
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
			feature = "drug-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "drug"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#drug: Vec<DrugProperty>,
	#[cfg(any(
		any(feature = "duns-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "duns"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#duns: Vec<DunsProperty>,
	#[cfg(any(
		any(feature = "email-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "email"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#email: Vec<EmailProperty>,
	#[cfg(any(
		any(
			feature = "family-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "familyName"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#family_name: Vec<FamilyNameProperty>,
	#[cfg(any(
		any(
			feature = "fax-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "faxNumber"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#fax_number: Vec<FaxNumberProperty>,
	#[cfg(any(
		any(
			feature = "follows-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "follows"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#follows: Vec<FollowsProperty>,
	#[cfg(any(
		any(feature = "funder-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "funder"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#funder: Vec<FunderProperty>,
	#[cfg(any(
		any(
			feature = "funding-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "funding"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#funding: Vec<FundingProperty>,
	#[cfg(any(
		any(feature = "gender-property-schema", feature = "pending-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "gender"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#gender: Vec<GenderProperty>,
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
			feature = "given-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "givenName"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#given_name: Vec<GivenNameProperty>,
	#[cfg(any(
		any(
			feature = "global-location-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "globalLocationNumber"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#global_location_number: Vec<GlobalLocationNumberProperty>,
	#[cfg(any(
		any(
			feature = "has-credential-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "hasCredential"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#has_credential: Vec<HasCredentialProperty>,
	#[cfg(any(
		any(
			feature = "has-occupation-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "hasOccupation"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#has_occupation: Vec<HasOccupationProperty>,
	#[cfg(any(
		any(
			feature = "has-offer-catalog-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "hasOfferCatalog"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#has_offer_catalog: Vec<HasOfferCatalogProperty>,
	#[cfg(any(
		any(
			feature = "has-pos-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "hasPOS"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#has_pos: Vec<HasPosProperty>,
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
		any(feature = "height-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "height"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#height: Vec<HeightProperty>,
	#[cfg(any(
		any(
			feature = "home-location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "homeLocation"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#home_location: Vec<HomeLocationProperty>,
	#[cfg(any(
		any(
			feature = "honorific-prefix-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "honorificPrefix"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#honorific_prefix: Vec<HonorificPrefixProperty>,
	#[cfg(any(
		any(
			feature = "honorific-suffix-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "honorificSuffix"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#honorific_suffix: Vec<HonorificSuffixProperty>,
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
			feature = "interaction-statistic-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "interactionStatistic"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
	#[cfg(any(
		any(
			feature = "isic-v-4-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "isicV4"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#isic_v_4: Vec<IsicV4Property>,
	#[cfg(any(
		any(
			feature = "job-title-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "jobTitle"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#job_title: Vec<JobTitleProperty>,
	#[cfg(any(
		any(feature = "knows-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "knows"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#knows: Vec<KnowsProperty>,
	#[cfg(any(
		any(
			feature = "knows-about-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "knowsAbout"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#knows_about: Vec<KnowsAboutProperty>,
	#[cfg(any(
		any(
			feature = "knows-language-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "knowsLanguage"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#knows_language: Vec<KnowsLanguageProperty>,
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
		any(
			feature = "makes-offer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "makesOffer"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#makes_offer: Vec<MakesOfferProperty>,
	#[cfg(any(
		any(
			feature = "member-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "memberOf"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#member_of: Vec<MemberOfProperty>,
	#[cfg(any(
		any(feature = "naics-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "naics"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#naics: Vec<NaicsProperty>,
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
			feature = "nationality-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "nationality"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#nationality: Vec<NationalityProperty>,
	#[cfg(any(
		any(
			feature = "net-worth-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "netWorth"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#net_worth: Vec<NetWorthProperty>,
	#[cfg(any(
		any(feature = "owns-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "owns"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#owns: Vec<OwnsProperty>,
	#[cfg(any(
		any(feature = "parent-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "parent"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#parent: Vec<ParentProperty>,
	#[cfg(any(
		any(
			feature = "parents-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "parents"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#parents: Vec<ParentsProperty>,
	#[cfg(any(
		any(
			feature = "performer-in-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "performerIn"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#performer_in: Vec<PerformerInProperty>,
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
			feature = "publishing-principles-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "publishingPrinciples"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
	#[cfg(any(
		any(
			feature = "related-to-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "relatedTo"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#related_to: Vec<RelatedToProperty>,
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
		any(feature = "seeks-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "seeks"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#seeks: Vec<SeeksProperty>,
	#[cfg(any(
		any(
			feature = "sibling-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "sibling"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#sibling: Vec<SiblingProperty>,
	#[cfg(any(
		any(
			feature = "siblings-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "siblings"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#siblings: Vec<SiblingsProperty>,
	#[cfg(any(
		any(
			feature = "sponsor-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "sponsor"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#sponsor: Vec<SponsorProperty>,
	#[cfg(any(
		any(feature = "spouse-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "spouse"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#spouse: Vec<SpouseProperty>,
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
		any(feature = "tax-id-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "taxID"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#tax_id: Vec<TaxIdProperty>,
	#[cfg(any(
		any(
			feature = "telephone-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "telephone"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#telephone: Vec<TelephoneProperty>,
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
	#[cfg(any(
		any(feature = "vat-id-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "vatID"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#vat_id: Vec<VatIdProperty>,
	#[cfg(any(
		any(feature = "weight-property-schema", feature = "general-schema-section"),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "weight"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#weight: Vec<WeightProperty>,
	#[cfg(any(
		any(
			feature = "work-location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "workLocation"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#work_location: Vec<WorkLocationProperty>,
	#[cfg(any(
		any(
			feature = "works-for-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	#[cfg_attr(feature = "serde", serde(rename = "worksFor"))]
	#[cfg_attr(feature = "serde", serde(default))]
	#[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
	#[cfg_attr(
		feature = "serde",
		serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
	)]
	pub r#works_for: Vec<WorksForProperty>,
}
