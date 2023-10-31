use super::*;
/// <https://schema.org/Patient>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Patient {
	#[cfg(any(
		any(
			feature = "additional-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#additional_name: Vec<AdditionalNameProperty>,
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
			feature = "address-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#address: Vec<AddressProperty>,
	#[cfg(any(
		any(
			feature = "affiliation-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#affiliation: Vec<AffiliationProperty>,
	#[cfg(any(
		any(
			feature = "agent-interaction-statistic-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#agent_interaction_statistic: Vec<AgentInteractionStatisticProperty>,
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
			feature = "alumni-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alumni_of: Vec<AlumniOfProperty>,
	#[cfg(any(
		any(
			feature = "audience-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#audience_type: Vec<AudienceTypeProperty>,
	#[cfg(any(
		any(feature = "award-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#award: Vec<AwardProperty>,
	#[cfg(any(
		any(feature = "awards-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#awards: Vec<AwardsProperty>,
	#[cfg(any(
		any(
			feature = "birth-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#birth_date: Vec<BirthDateProperty>,
	#[cfg(any(
		any(
			feature = "birth-place-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#birth_place: Vec<BirthPlaceProperty>,
	#[cfg(any(
		any(feature = "brand-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#brand: Vec<BrandProperty>,
	#[cfg(any(
		any(
			feature = "call-sign-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#call_sign: Vec<CallSignProperty>,
	#[cfg(any(
		any(
			feature = "children-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#children: Vec<ChildrenProperty>,
	#[cfg(any(
		any(
			feature = "colleague-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#colleague: Vec<ColleagueProperty>,
	#[cfg(any(
		any(
			feature = "colleagues-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#colleagues: Vec<ColleaguesProperty>,
	#[cfg(any(
		any(
			feature = "contact-point-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#contact_point: Vec<ContactPointProperty>,
	#[cfg(any(
		any(
			feature = "contact-points-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#contact_points: Vec<ContactPointsProperty>,
	#[cfg(any(
		any(
			feature = "death-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#death_date: Vec<DeathDateProperty>,
	#[cfg(any(
		any(
			feature = "death-place-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#death_place: Vec<DeathPlaceProperty>,
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
			feature = "diagnosis-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#diagnosis: Vec<DiagnosisProperty>,
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
			feature = "drug-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#drug: Vec<DrugProperty>,
	#[cfg(any(
		any(feature = "duns-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#duns: Vec<DunsProperty>,
	#[cfg(any(
		any(feature = "email-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#email: Vec<EmailProperty>,
	#[cfg(any(
		any(
			feature = "family-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#family_name: Vec<FamilyNameProperty>,
	#[cfg(any(
		any(
			feature = "fax-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#fax_number: Vec<FaxNumberProperty>,
	#[cfg(any(
		any(
			feature = "follows-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#follows: Vec<FollowsProperty>,
	#[cfg(any(
		any(feature = "funder-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#funder: Vec<FunderProperty>,
	#[cfg(any(
		any(
			feature = "funding-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#funding: Vec<FundingProperty>,
	#[cfg(any(
		any(feature = "gender-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#gender: Vec<GenderProperty>,
	#[cfg(any(
		any(
			feature = "geographic-area-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#geographic_area: Vec<GeographicAreaProperty>,
	#[cfg(any(
		any(
			feature = "given-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#given_name: Vec<GivenNameProperty>,
	#[cfg(any(
		any(
			feature = "global-location-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#global_location_number: Vec<GlobalLocationNumberProperty>,
	#[cfg(any(
		any(
			feature = "has-credential-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_credential: Vec<HasCredentialProperty>,
	#[cfg(any(
		any(
			feature = "has-occupation-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#has_occupation: Vec<HasOccupationProperty>,
	#[cfg(any(
		any(
			feature = "has-offer-catalog-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#has_offer_catalog: Vec<HasOfferCatalogProperty>,
	#[cfg(any(
		any(
			feature = "has-pos-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#has_pos: Vec<HasPosProperty>,
	#[cfg(any(
		any(
			feature = "health-condition-property-schema",
			feature = "health-lifesci-schema-section"
		),
		doc
	))]
	pub r#health_condition: Vec<HealthConditionProperty>,
	#[cfg(any(
		any(feature = "height-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#height: Vec<HeightProperty>,
	#[cfg(any(
		any(
			feature = "home-location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#home_location: Vec<HomeLocationProperty>,
	#[cfg(any(
		any(
			feature = "honorific-prefix-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#honorific_prefix: Vec<HonorificPrefixProperty>,
	#[cfg(any(
		any(
			feature = "honorific-suffix-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#honorific_suffix: Vec<HonorificSuffixProperty>,
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
			feature = "interaction-statistic-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
	#[cfg(any(
		any(
			feature = "isic-v-4-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#isic_v_4: Vec<IsicV4Property>,
	#[cfg(any(
		any(
			feature = "job-title-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#job_title: Vec<JobTitleProperty>,
	#[cfg(any(
		any(feature = "knows-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#knows: Vec<KnowsProperty>,
	#[cfg(any(
		any(
			feature = "knows-about-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#knows_about: Vec<KnowsAboutProperty>,
	#[cfg(any(
		any(
			feature = "knows-language-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#knows_language: Vec<KnowsLanguageProperty>,
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
			feature = "makes-offer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#makes_offer: Vec<MakesOfferProperty>,
	#[cfg(any(
		any(
			feature = "member-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#member_of: Vec<MemberOfProperty>,
	#[cfg(any(
		any(feature = "naics-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#naics: Vec<NaicsProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "nationality-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#nationality: Vec<NationalityProperty>,
	#[cfg(any(
		any(
			feature = "net-worth-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#net_worth: Vec<NetWorthProperty>,
	#[cfg(any(
		any(feature = "owns-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#owns: Vec<OwnsProperty>,
	#[cfg(any(
		any(feature = "parent-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#parent: Vec<ParentProperty>,
	#[cfg(any(
		any(
			feature = "parents-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#parents: Vec<ParentsProperty>,
	#[cfg(any(
		any(
			feature = "performer-in-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#performer_in: Vec<PerformerInProperty>,
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
			feature = "publishing-principles-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
	#[cfg(any(
		any(
			feature = "related-to-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#related_to: Vec<RelatedToProperty>,
	#[cfg(any(
		any(
			feature = "required-gender-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#required_gender: Vec<RequiredGenderProperty>,
	#[cfg(any(
		any(
			feature = "required-max-age-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#required_max_age: Vec<RequiredMaxAgeProperty>,
	#[cfg(any(
		any(
			feature = "required-min-age-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#required_min_age: Vec<RequiredMinAgeProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(feature = "seeks-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#seeks: Vec<SeeksProperty>,
	#[cfg(any(
		any(
			feature = "sibling-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#sibling: Vec<SiblingProperty>,
	#[cfg(any(
		any(
			feature = "siblings-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#siblings: Vec<SiblingsProperty>,
	#[cfg(any(
		any(
			feature = "sponsor-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#sponsor: Vec<SponsorProperty>,
	#[cfg(any(
		any(feature = "spouse-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#spouse: Vec<SpouseProperty>,
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
			feature = "suggested-age-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#suggested_age: Vec<SuggestedAgeProperty>,
	#[cfg(any(
		any(
			feature = "suggested-gender-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#suggested_gender: Vec<SuggestedGenderProperty>,
	#[cfg(any(
		any(
			feature = "suggested-max-age-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#suggested_max_age: Vec<SuggestedMaxAgeProperty>,
	#[cfg(any(
		any(
			feature = "suggested-measurement-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#suggested_measurement: Vec<SuggestedMeasurementProperty>,
	#[cfg(any(
		any(
			feature = "suggested-min-age-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#suggested_min_age: Vec<SuggestedMinAgeProperty>,
	#[cfg(any(
		any(feature = "tax-id-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#tax_id: Vec<TaxIdProperty>,
	#[cfg(any(
		any(
			feature = "telephone-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#telephone: Vec<TelephoneProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
	#[cfg(any(
		any(feature = "vat-id-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#vat_id: Vec<VatIdProperty>,
	#[cfg(any(
		any(feature = "weight-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#weight: Vec<WeightProperty>,
	#[cfg(any(
		any(
			feature = "work-location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#work_location: Vec<WorkLocationProperty>,
	#[cfg(any(
		any(
			feature = "works-for-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#works_for: Vec<WorksForProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Patient {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "additional-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#additional_name) as usize
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
						feature = "address-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#address) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "affiliation-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#affiliation) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "agent-interaction-statistic-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#agent_interaction_statistic) as usize
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
						feature = "alumni-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#alumni_of) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "audience-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#audience_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "award-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#award) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "awards-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#awards) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "birth-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#birth_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "birth-place-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#birth_place) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "brand-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#brand) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "call-sign-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#call_sign) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "children-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#children) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "colleague-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#colleague) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "colleagues-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#colleagues) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "contact-point-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#contact_point) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "contact-points-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#contact_points) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "death-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#death_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "death-place-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#death_place) as usize
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
						feature = "diagnosis-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#diagnosis) as usize
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
						feature = "drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#drug) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "duns-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#duns) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "email-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#email) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "family-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#family_name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "fax-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#fax_number) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "follows-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#follows) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "funder-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#funder) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#funding) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "gender-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#gender) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "geographic-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#geographic_area) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "given-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#given_name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "global-location-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#global_location_number) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-credential-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_credential) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-occupation-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_occupation) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-offer-catalog-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_offer_catalog) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-pos-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_pos) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "health-condition-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#health_condition) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "height-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#height) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "home-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#home_location) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "honorific-prefix-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#honorific_prefix) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "honorific-suffix-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#honorific_suffix) as usize
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
						feature = "interaction-statistic-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#interaction_statistic) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "isic-v-4-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#isic_v_4) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "job-title-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#job_title) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "knows-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#knows) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "knows-about-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#knows_about) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "knows-language-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#knows_language) as usize
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
						feature = "makes-offer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#makes_offer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "member-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#member_of) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "naics-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#naics) as usize
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
						feature = "nationality-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#nationality) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "net-worth-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#net_worth) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "owns-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#owns) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "parent-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#parent) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "parents-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#parents) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "performer-in-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#performer_in) as usize
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
						feature = "publishing-principles-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#publishing_principles) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#related_to) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "required-gender-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#required_gender) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "required-max-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#required_max_age) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "required-min-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#required_min_age) as usize
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
						feature = "seeks-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#seeks) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sibling-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sibling) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "siblings-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#siblings) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sponsor-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sponsor) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "spouse-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#spouse) as usize
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
						feature = "suggested-age-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#suggested_age) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "suggested-gender-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#suggested_gender) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "suggested-max-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#suggested_max_age) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "suggested-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#suggested_measurement) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "suggested-min-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#suggested_min_age) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "tax-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#tax_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "telephone-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#telephone) as usize
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
						feature = "vat-id-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#vat_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "weight-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#weight) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "work-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#work_location) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "works-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#works_for) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Patient", len)?;
			#[cfg(any(
				any(
					feature = "additional-name-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#additional_name) {
				serialize_struct.serialize_field("additionalName", {
					struct SerializeWith<'a>(&'a Vec<AdditionalNameProperty>);
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
					&SerializeWith(&self.r#additional_name)
				})?;
			} else {
				serialize_struct.skip_field("additionalName")?;
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
					feature = "address-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#address) {
				serialize_struct.serialize_field("address", {
					struct SerializeWith<'a>(&'a Vec<AddressProperty>);
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
					&SerializeWith(&self.r#address)
				})?;
			} else {
				serialize_struct.skip_field("address")?;
			}
			#[cfg(any(
				any(
					feature = "affiliation-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#affiliation) {
				serialize_struct.serialize_field("affiliation", {
					struct SerializeWith<'a>(&'a Vec<AffiliationProperty>);
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
					&SerializeWith(&self.r#affiliation)
				})?;
			} else {
				serialize_struct.skip_field("affiliation")?;
			}
			#[cfg(any(
				any(
					feature = "agent-interaction-statistic-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#agent_interaction_statistic) {
				serialize_struct.serialize_field("agentInteractionStatistic", {
					struct SerializeWith<'a>(&'a Vec<AgentInteractionStatisticProperty>);
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
					&SerializeWith(&self.r#agent_interaction_statistic)
				})?;
			} else {
				serialize_struct.skip_field("agentInteractionStatistic")?;
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
					feature = "alumni-of-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#alumni_of) {
				serialize_struct.serialize_field("alumniOf", {
					struct SerializeWith<'a>(&'a Vec<AlumniOfProperty>);
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
					&SerializeWith(&self.r#alumni_of)
				})?;
			} else {
				serialize_struct.skip_field("alumniOf")?;
			}
			#[cfg(any(
				any(
					feature = "audience-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#audience_type) {
				serialize_struct.serialize_field("audienceType", {
					struct SerializeWith<'a>(&'a Vec<AudienceTypeProperty>);
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
					&SerializeWith(&self.r#audience_type)
				})?;
			} else {
				serialize_struct.skip_field("audienceType")?;
			}
			#[cfg(any(
				any(feature = "award-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#award) {
				serialize_struct.serialize_field("award", {
					struct SerializeWith<'a>(&'a Vec<AwardProperty>);
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
					&SerializeWith(&self.r#award)
				})?;
			} else {
				serialize_struct.skip_field("award")?;
			}
			#[cfg(any(
				any(feature = "awards-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#awards) {
				serialize_struct.serialize_field("awards", {
					struct SerializeWith<'a>(&'a Vec<AwardsProperty>);
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
					&SerializeWith(&self.r#awards)
				})?;
			} else {
				serialize_struct.skip_field("awards")?;
			}
			#[cfg(any(
				any(
					feature = "birth-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#birth_date) {
				serialize_struct.serialize_field("birthDate", {
					struct SerializeWith<'a>(&'a Vec<BirthDateProperty>);
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
					&SerializeWith(&self.r#birth_date)
				})?;
			} else {
				serialize_struct.skip_field("birthDate")?;
			}
			#[cfg(any(
				any(
					feature = "birth-place-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#birth_place) {
				serialize_struct.serialize_field("birthPlace", {
					struct SerializeWith<'a>(&'a Vec<BirthPlaceProperty>);
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
					&SerializeWith(&self.r#birth_place)
				})?;
			} else {
				serialize_struct.skip_field("birthPlace")?;
			}
			#[cfg(any(
				any(feature = "brand-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#brand) {
				serialize_struct.serialize_field("brand", {
					struct SerializeWith<'a>(&'a Vec<BrandProperty>);
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
					&SerializeWith(&self.r#brand)
				})?;
			} else {
				serialize_struct.skip_field("brand")?;
			}
			#[cfg(any(
				any(
					feature = "call-sign-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#call_sign) {
				serialize_struct.serialize_field("callSign", {
					struct SerializeWith<'a>(&'a Vec<CallSignProperty>);
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
					&SerializeWith(&self.r#call_sign)
				})?;
			} else {
				serialize_struct.skip_field("callSign")?;
			}
			#[cfg(any(
				any(
					feature = "children-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#children) {
				serialize_struct.serialize_field("children", {
					struct SerializeWith<'a>(&'a Vec<ChildrenProperty>);
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
					&SerializeWith(&self.r#children)
				})?;
			} else {
				serialize_struct.skip_field("children")?;
			}
			#[cfg(any(
				any(
					feature = "colleague-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#colleague) {
				serialize_struct.serialize_field("colleague", {
					struct SerializeWith<'a>(&'a Vec<ColleagueProperty>);
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
					&SerializeWith(&self.r#colleague)
				})?;
			} else {
				serialize_struct.skip_field("colleague")?;
			}
			#[cfg(any(
				any(
					feature = "colleagues-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#colleagues) {
				serialize_struct.serialize_field("colleagues", {
					struct SerializeWith<'a>(&'a Vec<ColleaguesProperty>);
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
					&SerializeWith(&self.r#colleagues)
				})?;
			} else {
				serialize_struct.skip_field("colleagues")?;
			}
			#[cfg(any(
				any(
					feature = "contact-point-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#contact_point) {
				serialize_struct.serialize_field("contactPoint", {
					struct SerializeWith<'a>(&'a Vec<ContactPointProperty>);
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
					&SerializeWith(&self.r#contact_point)
				})?;
			} else {
				serialize_struct.skip_field("contactPoint")?;
			}
			#[cfg(any(
				any(
					feature = "contact-points-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#contact_points) {
				serialize_struct.serialize_field("contactPoints", {
					struct SerializeWith<'a>(&'a Vec<ContactPointsProperty>);
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
					&SerializeWith(&self.r#contact_points)
				})?;
			} else {
				serialize_struct.skip_field("contactPoints")?;
			}
			#[cfg(any(
				any(
					feature = "death-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#death_date) {
				serialize_struct.serialize_field("deathDate", {
					struct SerializeWith<'a>(&'a Vec<DeathDateProperty>);
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
					&SerializeWith(&self.r#death_date)
				})?;
			} else {
				serialize_struct.skip_field("deathDate")?;
			}
			#[cfg(any(
				any(
					feature = "death-place-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#death_place) {
				serialize_struct.serialize_field("deathPlace", {
					struct SerializeWith<'a>(&'a Vec<DeathPlaceProperty>);
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
					&SerializeWith(&self.r#death_place)
				})?;
			} else {
				serialize_struct.skip_field("deathPlace")?;
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
					feature = "diagnosis-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#diagnosis) {
				serialize_struct.serialize_field("diagnosis", {
					struct SerializeWith<'a>(&'a Vec<DiagnosisProperty>);
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
					&SerializeWith(&self.r#diagnosis)
				})?;
			} else {
				serialize_struct.skip_field("diagnosis")?;
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
					feature = "drug-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#drug) {
				serialize_struct.serialize_field("drug", {
					struct SerializeWith<'a>(&'a Vec<DrugProperty>);
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
					&SerializeWith(&self.r#drug)
				})?;
			} else {
				serialize_struct.skip_field("drug")?;
			}
			#[cfg(any(
				any(feature = "duns-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#duns) {
				serialize_struct.serialize_field("duns", {
					struct SerializeWith<'a>(&'a Vec<DunsProperty>);
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
					&SerializeWith(&self.r#duns)
				})?;
			} else {
				serialize_struct.skip_field("duns")?;
			}
			#[cfg(any(
				any(feature = "email-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#email) {
				serialize_struct.serialize_field("email", {
					struct SerializeWith<'a>(&'a Vec<EmailProperty>);
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
					&SerializeWith(&self.r#email)
				})?;
			} else {
				serialize_struct.skip_field("email")?;
			}
			#[cfg(any(
				any(
					feature = "family-name-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#family_name) {
				serialize_struct.serialize_field("familyName", {
					struct SerializeWith<'a>(&'a Vec<FamilyNameProperty>);
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
					&SerializeWith(&self.r#family_name)
				})?;
			} else {
				serialize_struct.skip_field("familyName")?;
			}
			#[cfg(any(
				any(
					feature = "fax-number-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#fax_number) {
				serialize_struct.serialize_field("faxNumber", {
					struct SerializeWith<'a>(&'a Vec<FaxNumberProperty>);
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
					&SerializeWith(&self.r#fax_number)
				})?;
			} else {
				serialize_struct.skip_field("faxNumber")?;
			}
			#[cfg(any(
				any(
					feature = "follows-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#follows) {
				serialize_struct.serialize_field("follows", {
					struct SerializeWith<'a>(&'a Vec<FollowsProperty>);
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
					&SerializeWith(&self.r#follows)
				})?;
			} else {
				serialize_struct.skip_field("follows")?;
			}
			#[cfg(any(
				any(feature = "funder-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#funder) {
				serialize_struct.serialize_field("funder", {
					struct SerializeWith<'a>(&'a Vec<FunderProperty>);
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
					&SerializeWith(&self.r#funder)
				})?;
			} else {
				serialize_struct.skip_field("funder")?;
			}
			#[cfg(any(
				any(
					feature = "funding-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#funding) {
				serialize_struct.serialize_field("funding", {
					struct SerializeWith<'a>(&'a Vec<FundingProperty>);
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
					&SerializeWith(&self.r#funding)
				})?;
			} else {
				serialize_struct.skip_field("funding")?;
			}
			#[cfg(any(
				any(feature = "gender-property-schema", feature = "pending-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#gender) {
				serialize_struct.serialize_field("gender", {
					struct SerializeWith<'a>(&'a Vec<GenderProperty>);
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
					&SerializeWith(&self.r#gender)
				})?;
			} else {
				serialize_struct.skip_field("gender")?;
			}
			#[cfg(any(
				any(
					feature = "geographic-area-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#geographic_area) {
				serialize_struct.serialize_field("geographicArea", {
					struct SerializeWith<'a>(&'a Vec<GeographicAreaProperty>);
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
					&SerializeWith(&self.r#geographic_area)
				})?;
			} else {
				serialize_struct.skip_field("geographicArea")?;
			}
			#[cfg(any(
				any(
					feature = "given-name-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#given_name) {
				serialize_struct.serialize_field("givenName", {
					struct SerializeWith<'a>(&'a Vec<GivenNameProperty>);
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
					&SerializeWith(&self.r#given_name)
				})?;
			} else {
				serialize_struct.skip_field("givenName")?;
			}
			#[cfg(any(
				any(
					feature = "global-location-number-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#global_location_number) {
				serialize_struct.serialize_field("globalLocationNumber", {
					struct SerializeWith<'a>(&'a Vec<GlobalLocationNumberProperty>);
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
					&SerializeWith(&self.r#global_location_number)
				})?;
			} else {
				serialize_struct.skip_field("globalLocationNumber")?;
			}
			#[cfg(any(
				any(
					feature = "has-credential-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_credential) {
				serialize_struct.serialize_field("hasCredential", {
					struct SerializeWith<'a>(&'a Vec<HasCredentialProperty>);
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
					&SerializeWith(&self.r#has_credential)
				})?;
			} else {
				serialize_struct.skip_field("hasCredential")?;
			}
			#[cfg(any(
				any(
					feature = "has-occupation-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_occupation) {
				serialize_struct.serialize_field("hasOccupation", {
					struct SerializeWith<'a>(&'a Vec<HasOccupationProperty>);
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
					&SerializeWith(&self.r#has_occupation)
				})?;
			} else {
				serialize_struct.skip_field("hasOccupation")?;
			}
			#[cfg(any(
				any(
					feature = "has-offer-catalog-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_offer_catalog) {
				serialize_struct.serialize_field("hasOfferCatalog", {
					struct SerializeWith<'a>(&'a Vec<HasOfferCatalogProperty>);
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
					&SerializeWith(&self.r#has_offer_catalog)
				})?;
			} else {
				serialize_struct.skip_field("hasOfferCatalog")?;
			}
			#[cfg(any(
				any(
					feature = "has-pos-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_pos) {
				serialize_struct.serialize_field("hasPOS", {
					struct SerializeWith<'a>(&'a Vec<HasPosProperty>);
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
					&SerializeWith(&self.r#has_pos)
				})?;
			} else {
				serialize_struct.skip_field("hasPOS")?;
			}
			#[cfg(any(
				any(
					feature = "health-condition-property-schema",
					feature = "health-lifesci-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#health_condition) {
				serialize_struct.serialize_field("healthCondition", {
					struct SerializeWith<'a>(&'a Vec<HealthConditionProperty>);
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
					&SerializeWith(&self.r#health_condition)
				})?;
			} else {
				serialize_struct.skip_field("healthCondition")?;
			}
			#[cfg(any(
				any(feature = "height-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#height) {
				serialize_struct.serialize_field("height", {
					struct SerializeWith<'a>(&'a Vec<HeightProperty>);
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
					&SerializeWith(&self.r#height)
				})?;
			} else {
				serialize_struct.skip_field("height")?;
			}
			#[cfg(any(
				any(
					feature = "home-location-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#home_location) {
				serialize_struct.serialize_field("homeLocation", {
					struct SerializeWith<'a>(&'a Vec<HomeLocationProperty>);
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
					&SerializeWith(&self.r#home_location)
				})?;
			} else {
				serialize_struct.skip_field("homeLocation")?;
			}
			#[cfg(any(
				any(
					feature = "honorific-prefix-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#honorific_prefix) {
				serialize_struct.serialize_field("honorificPrefix", {
					struct SerializeWith<'a>(&'a Vec<HonorificPrefixProperty>);
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
					&SerializeWith(&self.r#honorific_prefix)
				})?;
			} else {
				serialize_struct.skip_field("honorificPrefix")?;
			}
			#[cfg(any(
				any(
					feature = "honorific-suffix-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#honorific_suffix) {
				serialize_struct.serialize_field("honorificSuffix", {
					struct SerializeWith<'a>(&'a Vec<HonorificSuffixProperty>);
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
					&SerializeWith(&self.r#honorific_suffix)
				})?;
			} else {
				serialize_struct.skip_field("honorificSuffix")?;
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
					feature = "interaction-statistic-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#interaction_statistic) {
				serialize_struct.serialize_field("interactionStatistic", {
					struct SerializeWith<'a>(&'a Vec<InteractionStatisticProperty>);
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
					&SerializeWith(&self.r#interaction_statistic)
				})?;
			} else {
				serialize_struct.skip_field("interactionStatistic")?;
			}
			#[cfg(any(
				any(
					feature = "isic-v-4-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#isic_v_4) {
				serialize_struct.serialize_field("isicV4", {
					struct SerializeWith<'a>(&'a Vec<IsicV4Property>);
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
					&SerializeWith(&self.r#isic_v_4)
				})?;
			} else {
				serialize_struct.skip_field("isicV4")?;
			}
			#[cfg(any(
				any(
					feature = "job-title-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#job_title) {
				serialize_struct.serialize_field("jobTitle", {
					struct SerializeWith<'a>(&'a Vec<JobTitleProperty>);
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
					&SerializeWith(&self.r#job_title)
				})?;
			} else {
				serialize_struct.skip_field("jobTitle")?;
			}
			#[cfg(any(
				any(feature = "knows-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#knows) {
				serialize_struct.serialize_field("knows", {
					struct SerializeWith<'a>(&'a Vec<KnowsProperty>);
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
					&SerializeWith(&self.r#knows)
				})?;
			} else {
				serialize_struct.skip_field("knows")?;
			}
			#[cfg(any(
				any(
					feature = "knows-about-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#knows_about) {
				serialize_struct.serialize_field("knowsAbout", {
					struct SerializeWith<'a>(&'a Vec<KnowsAboutProperty>);
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
					&SerializeWith(&self.r#knows_about)
				})?;
			} else {
				serialize_struct.skip_field("knowsAbout")?;
			}
			#[cfg(any(
				any(
					feature = "knows-language-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#knows_language) {
				serialize_struct.serialize_field("knowsLanguage", {
					struct SerializeWith<'a>(&'a Vec<KnowsLanguageProperty>);
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
					&SerializeWith(&self.r#knows_language)
				})?;
			} else {
				serialize_struct.skip_field("knowsLanguage")?;
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
					feature = "makes-offer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#makes_offer) {
				serialize_struct.serialize_field("makesOffer", {
					struct SerializeWith<'a>(&'a Vec<MakesOfferProperty>);
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
					&SerializeWith(&self.r#makes_offer)
				})?;
			} else {
				serialize_struct.skip_field("makesOffer")?;
			}
			#[cfg(any(
				any(
					feature = "member-of-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#member_of) {
				serialize_struct.serialize_field("memberOf", {
					struct SerializeWith<'a>(&'a Vec<MemberOfProperty>);
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
					&SerializeWith(&self.r#member_of)
				})?;
			} else {
				serialize_struct.skip_field("memberOf")?;
			}
			#[cfg(any(
				any(feature = "naics-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#naics) {
				serialize_struct.serialize_field("naics", {
					struct SerializeWith<'a>(&'a Vec<NaicsProperty>);
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
					&SerializeWith(&self.r#naics)
				})?;
			} else {
				serialize_struct.skip_field("naics")?;
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
					feature = "nationality-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#nationality) {
				serialize_struct.serialize_field("nationality", {
					struct SerializeWith<'a>(&'a Vec<NationalityProperty>);
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
					&SerializeWith(&self.r#nationality)
				})?;
			} else {
				serialize_struct.skip_field("nationality")?;
			}
			#[cfg(any(
				any(
					feature = "net-worth-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#net_worth) {
				serialize_struct.serialize_field("netWorth", {
					struct SerializeWith<'a>(&'a Vec<NetWorthProperty>);
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
					&SerializeWith(&self.r#net_worth)
				})?;
			} else {
				serialize_struct.skip_field("netWorth")?;
			}
			#[cfg(any(
				any(feature = "owns-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#owns) {
				serialize_struct.serialize_field("owns", {
					struct SerializeWith<'a>(&'a Vec<OwnsProperty>);
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
					&SerializeWith(&self.r#owns)
				})?;
			} else {
				serialize_struct.skip_field("owns")?;
			}
			#[cfg(any(
				any(feature = "parent-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#parent) {
				serialize_struct.serialize_field("parent", {
					struct SerializeWith<'a>(&'a Vec<ParentProperty>);
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
					&SerializeWith(&self.r#parent)
				})?;
			} else {
				serialize_struct.skip_field("parent")?;
			}
			#[cfg(any(
				any(
					feature = "parents-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#parents) {
				serialize_struct.serialize_field("parents", {
					struct SerializeWith<'a>(&'a Vec<ParentsProperty>);
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
					&SerializeWith(&self.r#parents)
				})?;
			} else {
				serialize_struct.skip_field("parents")?;
			}
			#[cfg(any(
				any(
					feature = "performer-in-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#performer_in) {
				serialize_struct.serialize_field("performerIn", {
					struct SerializeWith<'a>(&'a Vec<PerformerInProperty>);
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
					&SerializeWith(&self.r#performer_in)
				})?;
			} else {
				serialize_struct.skip_field("performerIn")?;
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
					feature = "publishing-principles-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#publishing_principles) {
				serialize_struct.serialize_field("publishingPrinciples", {
					struct SerializeWith<'a>(&'a Vec<PublishingPrinciplesProperty>);
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
					&SerializeWith(&self.r#publishing_principles)
				})?;
			} else {
				serialize_struct.skip_field("publishingPrinciples")?;
			}
			#[cfg(any(
				any(
					feature = "related-to-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#related_to) {
				serialize_struct.serialize_field("relatedTo", {
					struct SerializeWith<'a>(&'a Vec<RelatedToProperty>);
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
					&SerializeWith(&self.r#related_to)
				})?;
			} else {
				serialize_struct.skip_field("relatedTo")?;
			}
			#[cfg(any(
				any(
					feature = "required-gender-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#required_gender) {
				serialize_struct.serialize_field("requiredGender", {
					struct SerializeWith<'a>(&'a Vec<RequiredGenderProperty>);
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
					&SerializeWith(&self.r#required_gender)
				})?;
			} else {
				serialize_struct.skip_field("requiredGender")?;
			}
			#[cfg(any(
				any(
					feature = "required-max-age-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#required_max_age) {
				serialize_struct.serialize_field("requiredMaxAge", {
					struct SerializeWith<'a>(&'a Vec<RequiredMaxAgeProperty>);
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
					&SerializeWith(&self.r#required_max_age)
				})?;
			} else {
				serialize_struct.skip_field("requiredMaxAge")?;
			}
			#[cfg(any(
				any(
					feature = "required-min-age-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#required_min_age) {
				serialize_struct.serialize_field("requiredMinAge", {
					struct SerializeWith<'a>(&'a Vec<RequiredMinAgeProperty>);
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
					&SerializeWith(&self.r#required_min_age)
				})?;
			} else {
				serialize_struct.skip_field("requiredMinAge")?;
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
				any(feature = "seeks-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#seeks) {
				serialize_struct.serialize_field("seeks", {
					struct SerializeWith<'a>(&'a Vec<SeeksProperty>);
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
					&SerializeWith(&self.r#seeks)
				})?;
			} else {
				serialize_struct.skip_field("seeks")?;
			}
			#[cfg(any(
				any(
					feature = "sibling-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#sibling) {
				serialize_struct.serialize_field("sibling", {
					struct SerializeWith<'a>(&'a Vec<SiblingProperty>);
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
					&SerializeWith(&self.r#sibling)
				})?;
			} else {
				serialize_struct.skip_field("sibling")?;
			}
			#[cfg(any(
				any(
					feature = "siblings-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#siblings) {
				serialize_struct.serialize_field("siblings", {
					struct SerializeWith<'a>(&'a Vec<SiblingsProperty>);
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
					&SerializeWith(&self.r#siblings)
				})?;
			} else {
				serialize_struct.skip_field("siblings")?;
			}
			#[cfg(any(
				any(
					feature = "sponsor-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#sponsor) {
				serialize_struct.serialize_field("sponsor", {
					struct SerializeWith<'a>(&'a Vec<SponsorProperty>);
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
					&SerializeWith(&self.r#sponsor)
				})?;
			} else {
				serialize_struct.skip_field("sponsor")?;
			}
			#[cfg(any(
				any(feature = "spouse-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#spouse) {
				serialize_struct.serialize_field("spouse", {
					struct SerializeWith<'a>(&'a Vec<SpouseProperty>);
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
					&SerializeWith(&self.r#spouse)
				})?;
			} else {
				serialize_struct.skip_field("spouse")?;
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
					feature = "suggested-age-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#suggested_age) {
				serialize_struct.serialize_field("suggestedAge", {
					struct SerializeWith<'a>(&'a Vec<SuggestedAgeProperty>);
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
					&SerializeWith(&self.r#suggested_age)
				})?;
			} else {
				serialize_struct.skip_field("suggestedAge")?;
			}
			#[cfg(any(
				any(
					feature = "suggested-gender-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#suggested_gender) {
				serialize_struct.serialize_field("suggestedGender", {
					struct SerializeWith<'a>(&'a Vec<SuggestedGenderProperty>);
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
					&SerializeWith(&self.r#suggested_gender)
				})?;
			} else {
				serialize_struct.skip_field("suggestedGender")?;
			}
			#[cfg(any(
				any(
					feature = "suggested-max-age-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#suggested_max_age) {
				serialize_struct.serialize_field("suggestedMaxAge", {
					struct SerializeWith<'a>(&'a Vec<SuggestedMaxAgeProperty>);
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
					&SerializeWith(&self.r#suggested_max_age)
				})?;
			} else {
				serialize_struct.skip_field("suggestedMaxAge")?;
			}
			#[cfg(any(
				any(
					feature = "suggested-measurement-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#suggested_measurement) {
				serialize_struct.serialize_field("suggestedMeasurement", {
					struct SerializeWith<'a>(&'a Vec<SuggestedMeasurementProperty>);
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
					&SerializeWith(&self.r#suggested_measurement)
				})?;
			} else {
				serialize_struct.skip_field("suggestedMeasurement")?;
			}
			#[cfg(any(
				any(
					feature = "suggested-min-age-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#suggested_min_age) {
				serialize_struct.serialize_field("suggestedMinAge", {
					struct SerializeWith<'a>(&'a Vec<SuggestedMinAgeProperty>);
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
					&SerializeWith(&self.r#suggested_min_age)
				})?;
			} else {
				serialize_struct.skip_field("suggestedMinAge")?;
			}
			#[cfg(any(
				any(feature = "tax-id-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#tax_id) {
				serialize_struct.serialize_field("taxID", {
					struct SerializeWith<'a>(&'a Vec<TaxIdProperty>);
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
					&SerializeWith(&self.r#tax_id)
				})?;
			} else {
				serialize_struct.skip_field("taxID")?;
			}
			#[cfg(any(
				any(
					feature = "telephone-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#telephone) {
				serialize_struct.serialize_field("telephone", {
					struct SerializeWith<'a>(&'a Vec<TelephoneProperty>);
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
					&SerializeWith(&self.r#telephone)
				})?;
			} else {
				serialize_struct.skip_field("telephone")?;
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
				any(feature = "vat-id-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#vat_id) {
				serialize_struct.serialize_field("vatID", {
					struct SerializeWith<'a>(&'a Vec<VatIdProperty>);
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
					&SerializeWith(&self.r#vat_id)
				})?;
			} else {
				serialize_struct.skip_field("vatID")?;
			}
			#[cfg(any(
				any(feature = "weight-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#weight) {
				serialize_struct.serialize_field("weight", {
					struct SerializeWith<'a>(&'a Vec<WeightProperty>);
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
					&SerializeWith(&self.r#weight)
				})?;
			} else {
				serialize_struct.skip_field("weight")?;
			}
			#[cfg(any(
				any(
					feature = "work-location-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#work_location) {
				serialize_struct.serialize_field("workLocation", {
					struct SerializeWith<'a>(&'a Vec<WorkLocationProperty>);
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
					&SerializeWith(&self.r#work_location)
				})?;
			} else {
				serialize_struct.skip_field("workLocation")?;
			}
			#[cfg(any(
				any(
					feature = "works-for-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#works_for) {
				serialize_struct.serialize_field("worksFor", {
					struct SerializeWith<'a>(&'a Vec<WorksForProperty>);
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
					&SerializeWith(&self.r#works_for)
				})?;
			} else {
				serialize_struct.skip_field("worksFor")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Patient {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "additional-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AdditionalName,
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
						feature = "address-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Address,
				#[cfg(any(
					any(
						feature = "affiliation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Affiliation,
				#[cfg(any(
					any(
						feature = "agent-interaction-statistic-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				AgentInteractionStatistic,
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
						feature = "alumni-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlumniOf,
				#[cfg(any(
					any(
						feature = "audience-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AudienceType,
				#[cfg(any(
					any(feature = "award-property-schema", feature = "general-schema-section"),
					doc
				))]
				Award,
				#[cfg(any(
					any(feature = "awards-property-schema", feature = "general-schema-section"),
					doc
				))]
				Awards,
				#[cfg(any(
					any(
						feature = "birth-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				BirthDate,
				#[cfg(any(
					any(
						feature = "birth-place-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				BirthPlace,
				#[cfg(any(
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				Brand,
				#[cfg(any(
					any(
						feature = "call-sign-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CallSign,
				#[cfg(any(
					any(
						feature = "children-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Children,
				#[cfg(any(
					any(
						feature = "colleague-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Colleague,
				#[cfg(any(
					any(
						feature = "colleagues-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Colleagues,
				#[cfg(any(
					any(
						feature = "contact-point-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ContactPoint,
				#[cfg(any(
					any(
						feature = "contact-points-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ContactPoints,
				#[cfg(any(
					any(
						feature = "death-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DeathDate,
				#[cfg(any(
					any(
						feature = "death-place-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DeathPlace,
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
						feature = "diagnosis-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Diagnosis,
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
						feature = "drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				Drug,
				#[cfg(any(
					any(feature = "duns-property-schema", feature = "general-schema-section"),
					doc
				))]
				Duns,
				#[cfg(any(
					any(feature = "email-property-schema", feature = "general-schema-section"),
					doc
				))]
				Email,
				#[cfg(any(
					any(
						feature = "family-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FamilyName,
				#[cfg(any(
					any(
						feature = "fax-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FaxNumber,
				#[cfg(any(
					any(
						feature = "follows-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Follows,
				#[cfg(any(
					any(feature = "funder-property-schema", feature = "general-schema-section"),
					doc
				))]
				Funder,
				#[cfg(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Funding,
				#[cfg(any(
					any(feature = "gender-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Gender,
				#[cfg(any(
					any(
						feature = "geographic-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				GeographicArea,
				#[cfg(any(
					any(
						feature = "given-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				GivenName,
				#[cfg(any(
					any(
						feature = "global-location-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				GlobalLocationNumber,
				#[cfg(any(
					any(
						feature = "has-credential-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasCredential,
				#[cfg(any(
					any(
						feature = "has-occupation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HasOccupation,
				#[cfg(any(
					any(
						feature = "has-offer-catalog-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HasOfferCatalog,
				#[cfg(any(
					any(
						feature = "has-pos-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HasPos,
				#[cfg(any(
					any(
						feature = "health-condition-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				HealthCondition,
				#[cfg(any(
					any(feature = "height-property-schema", feature = "general-schema-section"),
					doc
				))]
				Height,
				#[cfg(any(
					any(
						feature = "home-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HomeLocation,
				#[cfg(any(
					any(
						feature = "honorific-prefix-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HonorificPrefix,
				#[cfg(any(
					any(
						feature = "honorific-suffix-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HonorificSuffix,
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
						feature = "interaction-statistic-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				InteractionStatistic,
				#[cfg(any(
					any(
						feature = "isic-v-4-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsicV4,
				#[cfg(any(
					any(
						feature = "job-title-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				JobTitle,
				#[cfg(any(
					any(feature = "knows-property-schema", feature = "general-schema-section"),
					doc
				))]
				Knows,
				#[cfg(any(
					any(
						feature = "knows-about-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				KnowsAbout,
				#[cfg(any(
					any(
						feature = "knows-language-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				KnowsLanguage,
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
						feature = "makes-offer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MakesOffer,
				#[cfg(any(
					any(
						feature = "member-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MemberOf,
				#[cfg(any(
					any(feature = "naics-property-schema", feature = "general-schema-section"),
					doc
				))]
				Naics,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(
						feature = "nationality-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Nationality,
				#[cfg(any(
					any(
						feature = "net-worth-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NetWorth,
				#[cfg(any(
					any(feature = "owns-property-schema", feature = "general-schema-section"),
					doc
				))]
				Owns,
				#[cfg(any(
					any(feature = "parent-property-schema", feature = "general-schema-section"),
					doc
				))]
				Parent,
				#[cfg(any(
					any(
						feature = "parents-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Parents,
				#[cfg(any(
					any(
						feature = "performer-in-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PerformerIn,
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
						feature = "publishing-principles-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PublishingPrinciples,
				#[cfg(any(
					any(
						feature = "related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RelatedTo,
				#[cfg(any(
					any(
						feature = "required-gender-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RequiredGender,
				#[cfg(any(
					any(
						feature = "required-max-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RequiredMaxAge,
				#[cfg(any(
					any(
						feature = "required-min-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RequiredMinAge,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(feature = "seeks-property-schema", feature = "general-schema-section"),
					doc
				))]
				Seeks,
				#[cfg(any(
					any(
						feature = "sibling-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Sibling,
				#[cfg(any(
					any(
						feature = "siblings-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Siblings,
				#[cfg(any(
					any(
						feature = "sponsor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Sponsor,
				#[cfg(any(
					any(feature = "spouse-property-schema", feature = "general-schema-section"),
					doc
				))]
				Spouse,
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
						feature = "suggested-age-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SuggestedAge,
				#[cfg(any(
					any(
						feature = "suggested-gender-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SuggestedGender,
				#[cfg(any(
					any(
						feature = "suggested-max-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SuggestedMaxAge,
				#[cfg(any(
					any(
						feature = "suggested-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SuggestedMeasurement,
				#[cfg(any(
					any(
						feature = "suggested-min-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SuggestedMinAge,
				#[cfg(any(
					any(feature = "tax-id-property-schema", feature = "general-schema-section"),
					doc
				))]
				TaxId,
				#[cfg(any(
					any(
						feature = "telephone-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Telephone,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
				#[cfg(any(
					any(feature = "vat-id-property-schema", feature = "general-schema-section"),
					doc
				))]
				VatId,
				#[cfg(any(
					any(feature = "weight-property-schema", feature = "general-schema-section"),
					doc
				))]
				Weight,
				#[cfg(any(
					any(
						feature = "work-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				WorkLocation,
				#[cfg(any(
					any(
						feature = "works-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				WorksFor,
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
								feature = "additional-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalName" => Ok(Field::AdditionalName),
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
								feature = "address-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"address" => Ok(Field::Address),
						#[cfg(any(
							any(
								feature = "affiliation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"affiliation" => Ok(Field::Affiliation),
						#[cfg(any(
							any(
								feature = "agent-interaction-statistic-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"agentInteractionStatistic" => Ok(Field::AgentInteractionStatistic),
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
								feature = "alumni-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alumniOf" => Ok(Field::AlumniOf),
						#[cfg(any(
							any(
								feature = "audience-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"audienceType" => Ok(Field::AudienceType),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"award" => Ok(Field::Award),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"awards" => Ok(Field::Awards),
						#[cfg(any(
							any(
								feature = "birth-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"birthDate" => Ok(Field::BirthDate),
						#[cfg(any(
							any(
								feature = "birth-place-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"birthPlace" => Ok(Field::BirthPlace),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"brand" => Ok(Field::Brand),
						#[cfg(any(
							any(
								feature = "call-sign-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"callSign" => Ok(Field::CallSign),
						#[cfg(any(
							any(
								feature = "children-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"children" => Ok(Field::Children),
						#[cfg(any(
							any(
								feature = "colleague-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"colleague" => Ok(Field::Colleague),
						#[cfg(any(
							any(
								feature = "colleagues-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"colleagues" => Ok(Field::Colleagues),
						#[cfg(any(
							any(
								feature = "contact-point-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contactPoint" => Ok(Field::ContactPoint),
						#[cfg(any(
							any(
								feature = "contact-points-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contactPoints" => Ok(Field::ContactPoints),
						#[cfg(any(
							any(
								feature = "death-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"deathDate" => Ok(Field::DeathDate),
						#[cfg(any(
							any(
								feature = "death-place-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"deathPlace" => Ok(Field::DeathPlace),
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
								feature = "diagnosis-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"diagnosis" => Ok(Field::Diagnosis),
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
								feature = "drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"drug" => Ok(Field::Drug),
						#[cfg(any(
							any(
								feature = "duns-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"duns" => Ok(Field::Duns),
						#[cfg(any(
							any(
								feature = "email-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"email" => Ok(Field::Email),
						#[cfg(any(
							any(
								feature = "family-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"familyName" => Ok(Field::FamilyName),
						#[cfg(any(
							any(
								feature = "fax-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"faxNumber" => Ok(Field::FaxNumber),
						#[cfg(any(
							any(
								feature = "follows-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"follows" => Ok(Field::Follows),
						#[cfg(any(
							any(
								feature = "funder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"funder" => Ok(Field::Funder),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"funding" => Ok(Field::Funding),
						#[cfg(any(
							any(
								feature = "gender-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"gender" => Ok(Field::Gender),
						#[cfg(any(
							any(
								feature = "geographic-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"geographicArea" => Ok(Field::GeographicArea),
						#[cfg(any(
							any(
								feature = "given-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"givenName" => Ok(Field::GivenName),
						#[cfg(any(
							any(
								feature = "global-location-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"globalLocationNumber" => Ok(Field::GlobalLocationNumber),
						#[cfg(any(
							any(
								feature = "has-credential-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasCredential" => Ok(Field::HasCredential),
						#[cfg(any(
							any(
								feature = "has-occupation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"hasOccupation" => Ok(Field::HasOccupation),
						#[cfg(any(
							any(
								feature = "has-offer-catalog-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						#[cfg(any(
							any(
								feature = "has-pos-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"hasPOS" => Ok(Field::HasPos),
						#[cfg(any(
							any(
								feature = "health-condition-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						"healthCondition" => Ok(Field::HealthCondition),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"height" => Ok(Field::Height),
						#[cfg(any(
							any(
								feature = "home-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"homeLocation" => Ok(Field::HomeLocation),
						#[cfg(any(
							any(
								feature = "honorific-prefix-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"honorificPrefix" => Ok(Field::HonorificPrefix),
						#[cfg(any(
							any(
								feature = "honorific-suffix-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"honorificSuffix" => Ok(Field::HonorificSuffix),
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
								feature = "interaction-statistic-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"interactionStatistic" => Ok(Field::InteractionStatistic),
						#[cfg(any(
							any(
								feature = "isic-v-4-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isicV4" => Ok(Field::IsicV4),
						#[cfg(any(
							any(
								feature = "job-title-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"jobTitle" => Ok(Field::JobTitle),
						#[cfg(any(
							any(
								feature = "knows-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"knows" => Ok(Field::Knows),
						#[cfg(any(
							any(
								feature = "knows-about-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"knowsAbout" => Ok(Field::KnowsAbout),
						#[cfg(any(
							any(
								feature = "knows-language-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"knowsLanguage" => Ok(Field::KnowsLanguage),
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
								feature = "makes-offer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"makesOffer" => Ok(Field::MakesOffer),
						#[cfg(any(
							any(
								feature = "member-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"memberOf" => Ok(Field::MemberOf),
						#[cfg(any(
							any(
								feature = "naics-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"naics" => Ok(Field::Naics),
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
								feature = "nationality-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"nationality" => Ok(Field::Nationality),
						#[cfg(any(
							any(
								feature = "net-worth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"netWorth" => Ok(Field::NetWorth),
						#[cfg(any(
							any(
								feature = "owns-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"owns" => Ok(Field::Owns),
						#[cfg(any(
							any(
								feature = "parent-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"parent" => Ok(Field::Parent),
						#[cfg(any(
							any(
								feature = "parents-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"parents" => Ok(Field::Parents),
						#[cfg(any(
							any(
								feature = "performer-in-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"performerIn" => Ok(Field::PerformerIn),
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
								feature = "publishing-principles-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"publishingPrinciples" => Ok(Field::PublishingPrinciples),
						#[cfg(any(
							any(
								feature = "related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"relatedTo" => Ok(Field::RelatedTo),
						#[cfg(any(
							any(
								feature = "required-gender-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"requiredGender" => Ok(Field::RequiredGender),
						#[cfg(any(
							any(
								feature = "required-max-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"requiredMaxAge" => Ok(Field::RequiredMaxAge),
						#[cfg(any(
							any(
								feature = "required-min-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"requiredMinAge" => Ok(Field::RequiredMinAge),
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
								feature = "seeks-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"seeks" => Ok(Field::Seeks),
						#[cfg(any(
							any(
								feature = "sibling-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sibling" => Ok(Field::Sibling),
						#[cfg(any(
							any(
								feature = "siblings-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"siblings" => Ok(Field::Siblings),
						#[cfg(any(
							any(
								feature = "sponsor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sponsor" => Ok(Field::Sponsor),
						#[cfg(any(
							any(
								feature = "spouse-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"spouse" => Ok(Field::Spouse),
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
								feature = "suggested-age-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"suggestedAge" => Ok(Field::SuggestedAge),
						#[cfg(any(
							any(
								feature = "suggested-gender-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"suggestedGender" => Ok(Field::SuggestedGender),
						#[cfg(any(
							any(
								feature = "suggested-max-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"suggestedMaxAge" => Ok(Field::SuggestedMaxAge),
						#[cfg(any(
							any(
								feature = "suggested-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"suggestedMeasurement" => Ok(Field::SuggestedMeasurement),
						#[cfg(any(
							any(
								feature = "suggested-min-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"suggestedMinAge" => Ok(Field::SuggestedMinAge),
						#[cfg(any(
							any(
								feature = "tax-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"taxID" => Ok(Field::TaxId),
						#[cfg(any(
							any(
								feature = "telephone-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"telephone" => Ok(Field::Telephone),
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
								feature = "vat-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"vatID" => Ok(Field::VatId),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"weight" => Ok(Field::Weight),
						#[cfg(any(
							any(
								feature = "work-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"workLocation" => Ok(Field::WorkLocation),
						#[cfg(any(
							any(
								feature = "works-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"worksFor" => Ok(Field::WorksFor),
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
								feature = "additional-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalName" => Ok(Field::AdditionalName),
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
								feature = "address-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"address" => Ok(Field::Address),
						#[cfg(any(
							any(
								feature = "affiliation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"affiliation" => Ok(Field::Affiliation),
						#[cfg(any(
							any(
								feature = "agent-interaction-statistic-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"agentInteractionStatistic" => Ok(Field::AgentInteractionStatistic),
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
								feature = "alumni-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alumniOf" => Ok(Field::AlumniOf),
						#[cfg(any(
							any(
								feature = "audience-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"audienceType" => Ok(Field::AudienceType),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"award" => Ok(Field::Award),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"awards" => Ok(Field::Awards),
						#[cfg(any(
							any(
								feature = "birth-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"birthDate" => Ok(Field::BirthDate),
						#[cfg(any(
							any(
								feature = "birth-place-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"birthPlace" => Ok(Field::BirthPlace),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"brand" => Ok(Field::Brand),
						#[cfg(any(
							any(
								feature = "call-sign-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"callSign" => Ok(Field::CallSign),
						#[cfg(any(
							any(
								feature = "children-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"children" => Ok(Field::Children),
						#[cfg(any(
							any(
								feature = "colleague-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"colleague" => Ok(Field::Colleague),
						#[cfg(any(
							any(
								feature = "colleagues-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"colleagues" => Ok(Field::Colleagues),
						#[cfg(any(
							any(
								feature = "contact-point-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contactPoint" => Ok(Field::ContactPoint),
						#[cfg(any(
							any(
								feature = "contact-points-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contactPoints" => Ok(Field::ContactPoints),
						#[cfg(any(
							any(
								feature = "death-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"deathDate" => Ok(Field::DeathDate),
						#[cfg(any(
							any(
								feature = "death-place-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"deathPlace" => Ok(Field::DeathPlace),
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
								feature = "diagnosis-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"diagnosis" => Ok(Field::Diagnosis),
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
								feature = "drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"drug" => Ok(Field::Drug),
						#[cfg(any(
							any(
								feature = "duns-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"duns" => Ok(Field::Duns),
						#[cfg(any(
							any(
								feature = "email-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"email" => Ok(Field::Email),
						#[cfg(any(
							any(
								feature = "family-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"familyName" => Ok(Field::FamilyName),
						#[cfg(any(
							any(
								feature = "fax-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"faxNumber" => Ok(Field::FaxNumber),
						#[cfg(any(
							any(
								feature = "follows-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"follows" => Ok(Field::Follows),
						#[cfg(any(
							any(
								feature = "funder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"funder" => Ok(Field::Funder),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"funding" => Ok(Field::Funding),
						#[cfg(any(
							any(
								feature = "gender-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"gender" => Ok(Field::Gender),
						#[cfg(any(
							any(
								feature = "geographic-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"geographicArea" => Ok(Field::GeographicArea),
						#[cfg(any(
							any(
								feature = "given-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"givenName" => Ok(Field::GivenName),
						#[cfg(any(
							any(
								feature = "global-location-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"globalLocationNumber" => Ok(Field::GlobalLocationNumber),
						#[cfg(any(
							any(
								feature = "has-credential-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasCredential" => Ok(Field::HasCredential),
						#[cfg(any(
							any(
								feature = "has-occupation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"hasOccupation" => Ok(Field::HasOccupation),
						#[cfg(any(
							any(
								feature = "has-offer-catalog-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						#[cfg(any(
							any(
								feature = "has-pos-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"hasPOS" => Ok(Field::HasPos),
						#[cfg(any(
							any(
								feature = "health-condition-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						b"healthCondition" => Ok(Field::HealthCondition),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"height" => Ok(Field::Height),
						#[cfg(any(
							any(
								feature = "home-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"homeLocation" => Ok(Field::HomeLocation),
						#[cfg(any(
							any(
								feature = "honorific-prefix-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"honorificPrefix" => Ok(Field::HonorificPrefix),
						#[cfg(any(
							any(
								feature = "honorific-suffix-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"honorificSuffix" => Ok(Field::HonorificSuffix),
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
								feature = "interaction-statistic-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"interactionStatistic" => Ok(Field::InteractionStatistic),
						#[cfg(any(
							any(
								feature = "isic-v-4-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isicV4" => Ok(Field::IsicV4),
						#[cfg(any(
							any(
								feature = "job-title-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"jobTitle" => Ok(Field::JobTitle),
						#[cfg(any(
							any(
								feature = "knows-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"knows" => Ok(Field::Knows),
						#[cfg(any(
							any(
								feature = "knows-about-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"knowsAbout" => Ok(Field::KnowsAbout),
						#[cfg(any(
							any(
								feature = "knows-language-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"knowsLanguage" => Ok(Field::KnowsLanguage),
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
								feature = "makes-offer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"makesOffer" => Ok(Field::MakesOffer),
						#[cfg(any(
							any(
								feature = "member-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"memberOf" => Ok(Field::MemberOf),
						#[cfg(any(
							any(
								feature = "naics-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"naics" => Ok(Field::Naics),
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
								feature = "nationality-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"nationality" => Ok(Field::Nationality),
						#[cfg(any(
							any(
								feature = "net-worth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"netWorth" => Ok(Field::NetWorth),
						#[cfg(any(
							any(
								feature = "owns-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"owns" => Ok(Field::Owns),
						#[cfg(any(
							any(
								feature = "parent-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"parent" => Ok(Field::Parent),
						#[cfg(any(
							any(
								feature = "parents-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"parents" => Ok(Field::Parents),
						#[cfg(any(
							any(
								feature = "performer-in-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"performerIn" => Ok(Field::PerformerIn),
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
								feature = "publishing-principles-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"publishingPrinciples" => Ok(Field::PublishingPrinciples),
						#[cfg(any(
							any(
								feature = "related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"relatedTo" => Ok(Field::RelatedTo),
						#[cfg(any(
							any(
								feature = "required-gender-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"requiredGender" => Ok(Field::RequiredGender),
						#[cfg(any(
							any(
								feature = "required-max-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"requiredMaxAge" => Ok(Field::RequiredMaxAge),
						#[cfg(any(
							any(
								feature = "required-min-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"requiredMinAge" => Ok(Field::RequiredMinAge),
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
								feature = "seeks-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"seeks" => Ok(Field::Seeks),
						#[cfg(any(
							any(
								feature = "sibling-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sibling" => Ok(Field::Sibling),
						#[cfg(any(
							any(
								feature = "siblings-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"siblings" => Ok(Field::Siblings),
						#[cfg(any(
							any(
								feature = "sponsor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sponsor" => Ok(Field::Sponsor),
						#[cfg(any(
							any(
								feature = "spouse-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"spouse" => Ok(Field::Spouse),
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
								feature = "suggested-age-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"suggestedAge" => Ok(Field::SuggestedAge),
						#[cfg(any(
							any(
								feature = "suggested-gender-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"suggestedGender" => Ok(Field::SuggestedGender),
						#[cfg(any(
							any(
								feature = "suggested-max-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"suggestedMaxAge" => Ok(Field::SuggestedMaxAge),
						#[cfg(any(
							any(
								feature = "suggested-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"suggestedMeasurement" => Ok(Field::SuggestedMeasurement),
						#[cfg(any(
							any(
								feature = "suggested-min-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"suggestedMinAge" => Ok(Field::SuggestedMinAge),
						#[cfg(any(
							any(
								feature = "tax-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"taxID" => Ok(Field::TaxId),
						#[cfg(any(
							any(
								feature = "telephone-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"telephone" => Ok(Field::Telephone),
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
								feature = "vat-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"vatID" => Ok(Field::VatId),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"weight" => Ok(Field::Weight),
						#[cfg(any(
							any(
								feature = "work-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"workLocation" => Ok(Field::WorkLocation),
						#[cfg(any(
							any(
								feature = "works-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"worksFor" => Ok(Field::WorksFor),
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
				type Value = Patient;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Patient")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "additional-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#additional_name_property = None;
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
							feature = "address-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#address_property = None;
					#[cfg(any(
						any(
							feature = "affiliation-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#affiliation_property = None;
					#[cfg(any(
						any(
							feature = "agent-interaction-statistic-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#agent_interaction_statistic_property = None;
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
							feature = "alumni-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alumni_of_property = None;
					#[cfg(any(
						any(
							feature = "audience-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#audience_type_property = None;
					#[cfg(any(
						any(feature = "award-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#award_property = None;
					#[cfg(any(
						any(
							feature = "awards-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#awards_property = None;
					#[cfg(any(
						any(
							feature = "birth-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#birth_date_property = None;
					#[cfg(any(
						any(
							feature = "birth-place-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#birth_place_property = None;
					#[cfg(any(
						any(feature = "brand-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#brand_property = None;
					#[cfg(any(
						any(
							feature = "call-sign-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#call_sign_property = None;
					#[cfg(any(
						any(
							feature = "children-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#children_property = None;
					#[cfg(any(
						any(
							feature = "colleague-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#colleague_property = None;
					#[cfg(any(
						any(
							feature = "colleagues-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#colleagues_property = None;
					#[cfg(any(
						any(
							feature = "contact-point-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#contact_point_property = None;
					#[cfg(any(
						any(
							feature = "contact-points-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#contact_points_property = None;
					#[cfg(any(
						any(
							feature = "death-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#death_date_property = None;
					#[cfg(any(
						any(
							feature = "death-place-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#death_place_property = None;
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
							feature = "diagnosis-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#diagnosis_property = None;
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
							feature = "drug-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#drug_property = None;
					#[cfg(any(
						any(feature = "duns-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#duns_property = None;
					#[cfg(any(
						any(feature = "email-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#email_property = None;
					#[cfg(any(
						any(
							feature = "family-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#family_name_property = None;
					#[cfg(any(
						any(
							feature = "fax-number-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#fax_number_property = None;
					#[cfg(any(
						any(
							feature = "follows-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#follows_property = None;
					#[cfg(any(
						any(
							feature = "funder-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#funder_property = None;
					#[cfg(any(
						any(
							feature = "funding-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#funding_property = None;
					#[cfg(any(
						any(
							feature = "gender-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#gender_property = None;
					#[cfg(any(
						any(
							feature = "geographic-area-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#geographic_area_property = None;
					#[cfg(any(
						any(
							feature = "given-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#given_name_property = None;
					#[cfg(any(
						any(
							feature = "global-location-number-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#global_location_number_property = None;
					#[cfg(any(
						any(
							feature = "has-credential-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_credential_property = None;
					#[cfg(any(
						any(
							feature = "has-occupation-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#has_occupation_property = None;
					#[cfg(any(
						any(
							feature = "has-offer-catalog-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#has_offer_catalog_property = None;
					#[cfg(any(
						any(
							feature = "has-pos-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#has_pos_property = None;
					#[cfg(any(
						any(
							feature = "health-condition-property-schema",
							feature = "health-lifesci-schema-section"
						),
						doc
					))]
					let mut r#health_condition_property = None;
					#[cfg(any(
						any(
							feature = "height-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#height_property = None;
					#[cfg(any(
						any(
							feature = "home-location-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#home_location_property = None;
					#[cfg(any(
						any(
							feature = "honorific-prefix-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#honorific_prefix_property = None;
					#[cfg(any(
						any(
							feature = "honorific-suffix-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#honorific_suffix_property = None;
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
							feature = "interaction-statistic-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#interaction_statistic_property = None;
					#[cfg(any(
						any(
							feature = "isic-v-4-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#isic_v_4_property = None;
					#[cfg(any(
						any(
							feature = "job-title-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#job_title_property = None;
					#[cfg(any(
						any(feature = "knows-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#knows_property = None;
					#[cfg(any(
						any(
							feature = "knows-about-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#knows_about_property = None;
					#[cfg(any(
						any(
							feature = "knows-language-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#knows_language_property = None;
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
							feature = "makes-offer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#makes_offer_property = None;
					#[cfg(any(
						any(
							feature = "member-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#member_of_property = None;
					#[cfg(any(
						any(feature = "naics-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#naics_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "nationality-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#nationality_property = None;
					#[cfg(any(
						any(
							feature = "net-worth-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#net_worth_property = None;
					#[cfg(any(
						any(feature = "owns-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#owns_property = None;
					#[cfg(any(
						any(
							feature = "parent-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#parent_property = None;
					#[cfg(any(
						any(
							feature = "parents-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#parents_property = None;
					#[cfg(any(
						any(
							feature = "performer-in-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#performer_in_property = None;
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
							feature = "publishing-principles-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#publishing_principles_property = None;
					#[cfg(any(
						any(
							feature = "related-to-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#related_to_property = None;
					#[cfg(any(
						any(
							feature = "required-gender-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#required_gender_property = None;
					#[cfg(any(
						any(
							feature = "required-max-age-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#required_max_age_property = None;
					#[cfg(any(
						any(
							feature = "required-min-age-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#required_min_age_property = None;
					#[cfg(any(
						any(
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(feature = "seeks-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#seeks_property = None;
					#[cfg(any(
						any(
							feature = "sibling-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#sibling_property = None;
					#[cfg(any(
						any(
							feature = "siblings-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#siblings_property = None;
					#[cfg(any(
						any(
							feature = "sponsor-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#sponsor_property = None;
					#[cfg(any(
						any(
							feature = "spouse-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#spouse_property = None;
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
							feature = "suggested-age-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#suggested_age_property = None;
					#[cfg(any(
						any(
							feature = "suggested-gender-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#suggested_gender_property = None;
					#[cfg(any(
						any(
							feature = "suggested-max-age-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#suggested_max_age_property = None;
					#[cfg(any(
						any(
							feature = "suggested-measurement-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#suggested_measurement_property = None;
					#[cfg(any(
						any(
							feature = "suggested-min-age-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#suggested_min_age_property = None;
					#[cfg(any(
						any(
							feature = "tax-id-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#tax_id_property = None;
					#[cfg(any(
						any(
							feature = "telephone-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#telephone_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					#[cfg(any(
						any(
							feature = "vat-id-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#vat_id_property = None;
					#[cfg(any(
						any(
							feature = "weight-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#weight_property = None;
					#[cfg(any(
						any(
							feature = "work-location-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#work_location_property = None;
					#[cfg(any(
						any(
							feature = "works-for-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#works_for_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "additional-name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AdditionalName => {
								if r#additional_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalName",
									));
								}
								r#additional_name_property = Some({
									struct DeserializeWith(Vec<AdditionalNameProperty>);
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
									feature = "address-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Address => {
								if r#address_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"address",
									));
								}
								r#address_property = Some({
									struct DeserializeWith(Vec<AddressProperty>);
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
									feature = "affiliation-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Affiliation => {
								if r#affiliation_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"affiliation",
									));
								}
								r#affiliation_property = Some({
									struct DeserializeWith(Vec<AffiliationProperty>);
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
									feature = "agent-interaction-statistic-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::AgentInteractionStatistic => {
								if r#agent_interaction_statistic_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"agentInteractionStatistic",
									));
								}
								r#agent_interaction_statistic_property = Some({
									struct DeserializeWith(Vec<AgentInteractionStatisticProperty>);
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
									feature = "alumni-of-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AlumniOf => {
								if r#alumni_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"alumniOf",
									));
								}
								r#alumni_of_property = Some({
									struct DeserializeWith(Vec<AlumniOfProperty>);
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
									feature = "audience-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AudienceType => {
								if r#audience_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"audienceType",
									));
								}
								r#audience_type_property = Some({
									struct DeserializeWith(Vec<AudienceTypeProperty>);
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
									feature = "award-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Award => {
								if r#award_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("award"));
								}
								r#award_property = Some({
									struct DeserializeWith(Vec<AwardProperty>);
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
									feature = "awards-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Awards => {
								if r#awards_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("awards"));
								}
								r#awards_property = Some({
									struct DeserializeWith(Vec<AwardsProperty>);
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
									feature = "birth-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::BirthDate => {
								if r#birth_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"birthDate",
									));
								}
								r#birth_date_property = Some({
									struct DeserializeWith(Vec<BirthDateProperty>);
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
									feature = "birth-place-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::BirthPlace => {
								if r#birth_place_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"birthPlace",
									));
								}
								r#birth_place_property = Some({
									struct DeserializeWith(Vec<BirthPlaceProperty>);
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
									feature = "brand-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Brand => {
								if r#brand_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("brand"));
								}
								r#brand_property = Some({
									struct DeserializeWith(Vec<BrandProperty>);
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
									feature = "call-sign-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::CallSign => {
								if r#call_sign_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"callSign",
									));
								}
								r#call_sign_property = Some({
									struct DeserializeWith(Vec<CallSignProperty>);
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
									feature = "children-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Children => {
								if r#children_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"children",
									));
								}
								r#children_property = Some({
									struct DeserializeWith(Vec<ChildrenProperty>);
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
									feature = "colleague-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Colleague => {
								if r#colleague_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"colleague",
									));
								}
								r#colleague_property = Some({
									struct DeserializeWith(Vec<ColleagueProperty>);
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
									feature = "colleagues-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Colleagues => {
								if r#colleagues_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"colleagues",
									));
								}
								r#colleagues_property = Some({
									struct DeserializeWith(Vec<ColleaguesProperty>);
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
									feature = "contact-point-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ContactPoint => {
								if r#contact_point_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contactPoint",
									));
								}
								r#contact_point_property = Some({
									struct DeserializeWith(Vec<ContactPointProperty>);
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
									feature = "contact-points-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ContactPoints => {
								if r#contact_points_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contactPoints",
									));
								}
								r#contact_points_property = Some({
									struct DeserializeWith(Vec<ContactPointsProperty>);
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
									feature = "death-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DeathDate => {
								if r#death_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"deathDate",
									));
								}
								r#death_date_property = Some({
									struct DeserializeWith(Vec<DeathDateProperty>);
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
									feature = "death-place-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DeathPlace => {
								if r#death_place_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"deathPlace",
									));
								}
								r#death_place_property = Some({
									struct DeserializeWith(Vec<DeathPlaceProperty>);
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
									feature = "diagnosis-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::Diagnosis => {
								if r#diagnosis_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"diagnosis",
									));
								}
								r#diagnosis_property = Some({
									struct DeserializeWith(Vec<DiagnosisProperty>);
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
									feature = "drug-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::Drug => {
								if r#drug_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("drug"));
								}
								r#drug_property = Some({
									struct DeserializeWith(Vec<DrugProperty>);
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
									feature = "duns-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Duns => {
								if r#duns_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("duns"));
								}
								r#duns_property = Some({
									struct DeserializeWith(Vec<DunsProperty>);
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
									feature = "email-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Email => {
								if r#email_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("email"));
								}
								r#email_property = Some({
									struct DeserializeWith(Vec<EmailProperty>);
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
									feature = "family-name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FamilyName => {
								if r#family_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"familyName",
									));
								}
								r#family_name_property = Some({
									struct DeserializeWith(Vec<FamilyNameProperty>);
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
									feature = "fax-number-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FaxNumber => {
								if r#fax_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"faxNumber",
									));
								}
								r#fax_number_property = Some({
									struct DeserializeWith(Vec<FaxNumberProperty>);
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
									feature = "follows-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Follows => {
								if r#follows_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"follows",
									));
								}
								r#follows_property = Some({
									struct DeserializeWith(Vec<FollowsProperty>);
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
									feature = "funder-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Funder => {
								if r#funder_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("funder"));
								}
								r#funder_property = Some({
									struct DeserializeWith(Vec<FunderProperty>);
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
									feature = "funding-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Funding => {
								if r#funding_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"funding",
									));
								}
								r#funding_property = Some({
									struct DeserializeWith(Vec<FundingProperty>);
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
									feature = "gender-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Gender => {
								if r#gender_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("gender"));
								}
								r#gender_property = Some({
									struct DeserializeWith(Vec<GenderProperty>);
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
									feature = "geographic-area-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::GeographicArea => {
								if r#geographic_area_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geographicArea",
									));
								}
								r#geographic_area_property = Some({
									struct DeserializeWith(Vec<GeographicAreaProperty>);
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
									feature = "given-name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::GivenName => {
								if r#given_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"givenName",
									));
								}
								r#given_name_property = Some({
									struct DeserializeWith(Vec<GivenNameProperty>);
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
									feature = "global-location-number-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::GlobalLocationNumber => {
								if r#global_location_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"globalLocationNumber",
									));
								}
								r#global_location_number_property = Some({
									struct DeserializeWith(Vec<GlobalLocationNumberProperty>);
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
									feature = "has-credential-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasCredential => {
								if r#has_credential_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasCredential",
									));
								}
								r#has_credential_property = Some({
									struct DeserializeWith(Vec<HasCredentialProperty>);
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
									feature = "has-occupation-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::HasOccupation => {
								if r#has_occupation_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasOccupation",
									));
								}
								r#has_occupation_property = Some({
									struct DeserializeWith(Vec<HasOccupationProperty>);
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
									feature = "has-offer-catalog-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::HasOfferCatalog => {
								if r#has_offer_catalog_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasOfferCatalog",
									));
								}
								r#has_offer_catalog_property = Some({
									struct DeserializeWith(Vec<HasOfferCatalogProperty>);
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
									feature = "has-pos-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::HasPos => {
								if r#has_pos_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("hasPOS"));
								}
								r#has_pos_property = Some({
									struct DeserializeWith(Vec<HasPosProperty>);
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
									feature = "health-condition-property-schema",
									feature = "health-lifesci-schema-section"
								),
								doc
							))]
							Field::HealthCondition => {
								if r#health_condition_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"healthCondition",
									));
								}
								r#health_condition_property = Some({
									struct DeserializeWith(Vec<HealthConditionProperty>);
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
									feature = "height-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Height => {
								if r#height_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("height"));
								}
								r#height_property = Some({
									struct DeserializeWith(Vec<HeightProperty>);
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
									feature = "home-location-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::HomeLocation => {
								if r#home_location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"homeLocation",
									));
								}
								r#home_location_property = Some({
									struct DeserializeWith(Vec<HomeLocationProperty>);
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
									feature = "honorific-prefix-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::HonorificPrefix => {
								if r#honorific_prefix_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"honorificPrefix",
									));
								}
								r#honorific_prefix_property = Some({
									struct DeserializeWith(Vec<HonorificPrefixProperty>);
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
									feature = "honorific-suffix-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::HonorificSuffix => {
								if r#honorific_suffix_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"honorificSuffix",
									));
								}
								r#honorific_suffix_property = Some({
									struct DeserializeWith(Vec<HonorificSuffixProperty>);
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
									feature = "interaction-statistic-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::InteractionStatistic => {
								if r#interaction_statistic_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"interactionStatistic",
									));
								}
								r#interaction_statistic_property = Some({
									struct DeserializeWith(Vec<InteractionStatisticProperty>);
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
									feature = "isic-v-4-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::IsicV4 => {
								if r#isic_v_4_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("isicV4"));
								}
								r#isic_v_4_property = Some({
									struct DeserializeWith(Vec<IsicV4Property>);
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
									feature = "job-title-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::JobTitle => {
								if r#job_title_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"jobTitle",
									));
								}
								r#job_title_property = Some({
									struct DeserializeWith(Vec<JobTitleProperty>);
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
									feature = "knows-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Knows => {
								if r#knows_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("knows"));
								}
								r#knows_property = Some({
									struct DeserializeWith(Vec<KnowsProperty>);
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
									feature = "knows-about-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::KnowsAbout => {
								if r#knows_about_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"knowsAbout",
									));
								}
								r#knows_about_property = Some({
									struct DeserializeWith(Vec<KnowsAboutProperty>);
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
									feature = "knows-language-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::KnowsLanguage => {
								if r#knows_language_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"knowsLanguage",
									));
								}
								r#knows_language_property = Some({
									struct DeserializeWith(Vec<KnowsLanguageProperty>);
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
									feature = "makes-offer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MakesOffer => {
								if r#makes_offer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"makesOffer",
									));
								}
								r#makes_offer_property = Some({
									struct DeserializeWith(Vec<MakesOfferProperty>);
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
									feature = "member-of-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MemberOf => {
								if r#member_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"memberOf",
									));
								}
								r#member_of_property = Some({
									struct DeserializeWith(Vec<MemberOfProperty>);
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
									feature = "naics-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Naics => {
								if r#naics_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("naics"));
								}
								r#naics_property = Some({
									struct DeserializeWith(Vec<NaicsProperty>);
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
									feature = "nationality-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Nationality => {
								if r#nationality_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"nationality",
									));
								}
								r#nationality_property = Some({
									struct DeserializeWith(Vec<NationalityProperty>);
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
									feature = "net-worth-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::NetWorth => {
								if r#net_worth_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"netWorth",
									));
								}
								r#net_worth_property = Some({
									struct DeserializeWith(Vec<NetWorthProperty>);
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
									feature = "owns-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Owns => {
								if r#owns_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("owns"));
								}
								r#owns_property = Some({
									struct DeserializeWith(Vec<OwnsProperty>);
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
									feature = "parent-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Parent => {
								if r#parent_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("parent"));
								}
								r#parent_property = Some({
									struct DeserializeWith(Vec<ParentProperty>);
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
									feature = "parents-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Parents => {
								if r#parents_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"parents",
									));
								}
								r#parents_property = Some({
									struct DeserializeWith(Vec<ParentsProperty>);
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
									feature = "performer-in-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PerformerIn => {
								if r#performer_in_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"performerIn",
									));
								}
								r#performer_in_property = Some({
									struct DeserializeWith(Vec<PerformerInProperty>);
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
									feature = "publishing-principles-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PublishingPrinciples => {
								if r#publishing_principles_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"publishingPrinciples",
									));
								}
								r#publishing_principles_property = Some({
									struct DeserializeWith(Vec<PublishingPrinciplesProperty>);
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
									feature = "related-to-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::RelatedTo => {
								if r#related_to_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"relatedTo",
									));
								}
								r#related_to_property = Some({
									struct DeserializeWith(Vec<RelatedToProperty>);
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
									feature = "required-gender-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::RequiredGender => {
								if r#required_gender_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"requiredGender",
									));
								}
								r#required_gender_property = Some({
									struct DeserializeWith(Vec<RequiredGenderProperty>);
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
									feature = "required-max-age-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::RequiredMaxAge => {
								if r#required_max_age_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"requiredMaxAge",
									));
								}
								r#required_max_age_property = Some({
									struct DeserializeWith(Vec<RequiredMaxAgeProperty>);
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
									feature = "required-min-age-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::RequiredMinAge => {
								if r#required_min_age_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"requiredMinAge",
									));
								}
								r#required_min_age_property = Some({
									struct DeserializeWith(Vec<RequiredMinAgeProperty>);
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
									feature = "seeks-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Seeks => {
								if r#seeks_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("seeks"));
								}
								r#seeks_property = Some({
									struct DeserializeWith(Vec<SeeksProperty>);
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
									feature = "sibling-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Sibling => {
								if r#sibling_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sibling",
									));
								}
								r#sibling_property = Some({
									struct DeserializeWith(Vec<SiblingProperty>);
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
									feature = "siblings-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Siblings => {
								if r#siblings_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"siblings",
									));
								}
								r#siblings_property = Some({
									struct DeserializeWith(Vec<SiblingsProperty>);
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
									feature = "sponsor-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Sponsor => {
								if r#sponsor_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sponsor",
									));
								}
								r#sponsor_property = Some({
									struct DeserializeWith(Vec<SponsorProperty>);
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
									feature = "spouse-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Spouse => {
								if r#spouse_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("spouse"));
								}
								r#spouse_property = Some({
									struct DeserializeWith(Vec<SpouseProperty>);
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
									feature = "suggested-age-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::SuggestedAge => {
								if r#suggested_age_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"suggestedAge",
									));
								}
								r#suggested_age_property = Some({
									struct DeserializeWith(Vec<SuggestedAgeProperty>);
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
									feature = "suggested-gender-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SuggestedGender => {
								if r#suggested_gender_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"suggestedGender",
									));
								}
								r#suggested_gender_property = Some({
									struct DeserializeWith(Vec<SuggestedGenderProperty>);
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
									feature = "suggested-max-age-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SuggestedMaxAge => {
								if r#suggested_max_age_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"suggestedMaxAge",
									));
								}
								r#suggested_max_age_property = Some({
									struct DeserializeWith(Vec<SuggestedMaxAgeProperty>);
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
									feature = "suggested-measurement-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::SuggestedMeasurement => {
								if r#suggested_measurement_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"suggestedMeasurement",
									));
								}
								r#suggested_measurement_property = Some({
									struct DeserializeWith(Vec<SuggestedMeasurementProperty>);
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
									feature = "suggested-min-age-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SuggestedMinAge => {
								if r#suggested_min_age_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"suggestedMinAge",
									));
								}
								r#suggested_min_age_property = Some({
									struct DeserializeWith(Vec<SuggestedMinAgeProperty>);
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
									feature = "tax-id-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::TaxId => {
								if r#tax_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("taxID"));
								}
								r#tax_id_property = Some({
									struct DeserializeWith(Vec<TaxIdProperty>);
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
									feature = "telephone-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Telephone => {
								if r#telephone_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"telephone",
									));
								}
								r#telephone_property = Some({
									struct DeserializeWith(Vec<TelephoneProperty>);
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
									feature = "vat-id-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VatId => {
								if r#vat_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("vatID"));
								}
								r#vat_id_property = Some({
									struct DeserializeWith(Vec<VatIdProperty>);
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
									feature = "weight-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Weight => {
								if r#weight_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("weight"));
								}
								r#weight_property = Some({
									struct DeserializeWith(Vec<WeightProperty>);
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
									feature = "work-location-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::WorkLocation => {
								if r#work_location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"workLocation",
									));
								}
								r#work_location_property = Some({
									struct DeserializeWith(Vec<WorkLocationProperty>);
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
									feature = "works-for-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::WorksFor => {
								if r#works_for_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"worksFor",
									));
								}
								r#works_for_property = Some({
									struct DeserializeWith(Vec<WorksForProperty>);
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
					Ok(Patient {
						#[cfg(any(
							any(
								feature = "additional-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#additional_name: r#additional_name_property.unwrap_or_default(),
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
								feature = "address-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#address: r#address_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "affiliation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#affiliation: r#affiliation_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "agent-interaction-statistic-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#agent_interaction_statistic: r#agent_interaction_statistic_property
							.unwrap_or_default(),
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
								feature = "alumni-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#alumni_of: r#alumni_of_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "audience-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#audience_type: r#audience_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "award-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#award: r#award_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "awards-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#awards: r#awards_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "birth-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#birth_date: r#birth_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "birth-place-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#birth_place: r#birth_place_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#brand: r#brand_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "call-sign-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#call_sign: r#call_sign_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "children-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#children: r#children_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "colleague-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#colleague: r#colleague_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "colleagues-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#colleagues: r#colleagues_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "contact-point-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#contact_point: r#contact_point_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "contact-points-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#contact_points: r#contact_points_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "death-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#death_date: r#death_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "death-place-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#death_place: r#death_place_property.unwrap_or_default(),
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
								feature = "diagnosis-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#diagnosis: r#diagnosis_property.unwrap_or_default(),
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
								feature = "drug-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#drug: r#drug_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "duns-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#duns: r#duns_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "email-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#email: r#email_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "family-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#family_name: r#family_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "fax-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#fax_number: r#fax_number_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "follows-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#follows: r#follows_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "funder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#funder: r#funder_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#funding: r#funding_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "gender-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#gender: r#gender_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "geographic-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#geographic_area: r#geographic_area_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "given-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#given_name: r#given_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "global-location-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#global_location_number: r#global_location_number_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-credential-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_credential: r#has_credential_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-occupation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#has_occupation: r#has_occupation_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-offer-catalog-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#has_offer_catalog: r#has_offer_catalog_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-pos-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#has_pos: r#has_pos_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "health-condition-property-schema",
								feature = "health-lifesci-schema-section"
							),
							doc
						))]
						r#health_condition: r#health_condition_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "height-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#height: r#height_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "home-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#home_location: r#home_location_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "honorific-prefix-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#honorific_prefix: r#honorific_prefix_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "honorific-suffix-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#honorific_suffix: r#honorific_suffix_property.unwrap_or_default(),
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
								feature = "interaction-statistic-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#interaction_statistic: r#interaction_statistic_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "isic-v-4-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#isic_v_4: r#isic_v_4_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "job-title-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#job_title: r#job_title_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "knows-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#knows: r#knows_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "knows-about-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#knows_about: r#knows_about_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "knows-language-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#knows_language: r#knows_language_property.unwrap_or_default(),
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
								feature = "makes-offer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#makes_offer: r#makes_offer_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "member-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#member_of: r#member_of_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "naics-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#naics: r#naics_property.unwrap_or_default(),
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
								feature = "nationality-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#nationality: r#nationality_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "net-worth-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#net_worth: r#net_worth_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "owns-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#owns: r#owns_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "parent-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#parent: r#parent_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "parents-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#parents: r#parents_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "performer-in-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#performer_in: r#performer_in_property.unwrap_or_default(),
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
								feature = "publishing-principles-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#publishing_principles: r#publishing_principles_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "related-to-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#related_to: r#related_to_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "required-gender-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#required_gender: r#required_gender_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "required-max-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#required_max_age: r#required_max_age_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "required-min-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#required_min_age: r#required_min_age_property.unwrap_or_default(),
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
								feature = "seeks-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#seeks: r#seeks_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sibling-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#sibling: r#sibling_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "siblings-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#siblings: r#siblings_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sponsor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#sponsor: r#sponsor_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "spouse-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#spouse: r#spouse_property.unwrap_or_default(),
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
								feature = "suggested-age-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#suggested_age: r#suggested_age_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "suggested-gender-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#suggested_gender: r#suggested_gender_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "suggested-max-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#suggested_max_age: r#suggested_max_age_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "suggested-measurement-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#suggested_measurement: r#suggested_measurement_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "suggested-min-age-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#suggested_min_age: r#suggested_min_age_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "tax-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#tax_id: r#tax_id_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "telephone-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#telephone: r#telephone_property.unwrap_or_default(),
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
								feature = "vat-id-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#vat_id: r#vat_id_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "weight-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#weight: r#weight_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "work-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#work_location: r#work_location_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "works-for-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#works_for: r#works_for_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "additional-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"additionalName",
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
						feature = "address-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"address",
				#[cfg(any(
					any(
						feature = "affiliation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"affiliation",
				#[cfg(any(
					any(
						feature = "agent-interaction-statistic-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"agentInteractionStatistic",
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
						feature = "alumni-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alumniOf",
				#[cfg(any(
					any(
						feature = "audience-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"audienceType",
				#[cfg(any(
					any(feature = "award-property-schema", feature = "general-schema-section"),
					doc
				))]
				"award",
				#[cfg(any(
					any(feature = "awards-property-schema", feature = "general-schema-section"),
					doc
				))]
				"awards",
				#[cfg(any(
					any(
						feature = "birth-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"birthDate",
				#[cfg(any(
					any(
						feature = "birth-place-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"birthPlace",
				#[cfg(any(
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				"brand",
				#[cfg(any(
					any(
						feature = "call-sign-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"callSign",
				#[cfg(any(
					any(
						feature = "children-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"children",
				#[cfg(any(
					any(
						feature = "colleague-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"colleague",
				#[cfg(any(
					any(
						feature = "colleagues-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"colleagues",
				#[cfg(any(
					any(
						feature = "contact-point-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contactPoint",
				#[cfg(any(
					any(
						feature = "contact-points-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contactPoints",
				#[cfg(any(
					any(
						feature = "death-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"deathDate",
				#[cfg(any(
					any(
						feature = "death-place-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"deathPlace",
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
						feature = "diagnosis-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"diagnosis",
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
						feature = "drug-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"drug",
				#[cfg(any(
					any(feature = "duns-property-schema", feature = "general-schema-section"),
					doc
				))]
				"duns",
				#[cfg(any(
					any(feature = "email-property-schema", feature = "general-schema-section"),
					doc
				))]
				"email",
				#[cfg(any(
					any(
						feature = "family-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"familyName",
				#[cfg(any(
					any(
						feature = "fax-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"faxNumber",
				#[cfg(any(
					any(
						feature = "follows-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"follows",
				#[cfg(any(
					any(feature = "funder-property-schema", feature = "general-schema-section"),
					doc
				))]
				"funder",
				#[cfg(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"funding",
				#[cfg(any(
					any(feature = "gender-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"gender",
				#[cfg(any(
					any(
						feature = "geographic-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"geographicArea",
				#[cfg(any(
					any(
						feature = "given-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"givenName",
				#[cfg(any(
					any(
						feature = "global-location-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"globalLocationNumber",
				#[cfg(any(
					any(
						feature = "has-credential-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasCredential",
				#[cfg(any(
					any(
						feature = "has-occupation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"hasOccupation",
				#[cfg(any(
					any(
						feature = "has-offer-catalog-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"hasOfferCatalog",
				#[cfg(any(
					any(
						feature = "has-pos-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"hasPOS",
				#[cfg(any(
					any(
						feature = "health-condition-property-schema",
						feature = "health-lifesci-schema-section"
					),
					doc
				))]
				"healthCondition",
				#[cfg(any(
					any(feature = "height-property-schema", feature = "general-schema-section"),
					doc
				))]
				"height",
				#[cfg(any(
					any(
						feature = "home-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"homeLocation",
				#[cfg(any(
					any(
						feature = "honorific-prefix-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"honorificPrefix",
				#[cfg(any(
					any(
						feature = "honorific-suffix-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"honorificSuffix",
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
						feature = "interaction-statistic-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"interactionStatistic",
				#[cfg(any(
					any(
						feature = "isic-v-4-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isicV4",
				#[cfg(any(
					any(
						feature = "job-title-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"jobTitle",
				#[cfg(any(
					any(feature = "knows-property-schema", feature = "general-schema-section"),
					doc
				))]
				"knows",
				#[cfg(any(
					any(
						feature = "knows-about-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"knowsAbout",
				#[cfg(any(
					any(
						feature = "knows-language-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"knowsLanguage",
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
						feature = "makes-offer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"makesOffer",
				#[cfg(any(
					any(
						feature = "member-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"memberOf",
				#[cfg(any(
					any(feature = "naics-property-schema", feature = "general-schema-section"),
					doc
				))]
				"naics",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(
						feature = "nationality-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"nationality",
				#[cfg(any(
					any(
						feature = "net-worth-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"netWorth",
				#[cfg(any(
					any(feature = "owns-property-schema", feature = "general-schema-section"),
					doc
				))]
				"owns",
				#[cfg(any(
					any(feature = "parent-property-schema", feature = "general-schema-section"),
					doc
				))]
				"parent",
				#[cfg(any(
					any(
						feature = "parents-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"parents",
				#[cfg(any(
					any(
						feature = "performer-in-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"performerIn",
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
						feature = "publishing-principles-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"publishingPrinciples",
				#[cfg(any(
					any(
						feature = "related-to-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"relatedTo",
				#[cfg(any(
					any(
						feature = "required-gender-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"requiredGender",
				#[cfg(any(
					any(
						feature = "required-max-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"requiredMaxAge",
				#[cfg(any(
					any(
						feature = "required-min-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"requiredMinAge",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(feature = "seeks-property-schema", feature = "general-schema-section"),
					doc
				))]
				"seeks",
				#[cfg(any(
					any(
						feature = "sibling-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sibling",
				#[cfg(any(
					any(
						feature = "siblings-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"siblings",
				#[cfg(any(
					any(
						feature = "sponsor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sponsor",
				#[cfg(any(
					any(feature = "spouse-property-schema", feature = "general-schema-section"),
					doc
				))]
				"spouse",
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
						feature = "suggested-age-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"suggestedAge",
				#[cfg(any(
					any(
						feature = "suggested-gender-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"suggestedGender",
				#[cfg(any(
					any(
						feature = "suggested-max-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"suggestedMaxAge",
				#[cfg(any(
					any(
						feature = "suggested-measurement-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"suggestedMeasurement",
				#[cfg(any(
					any(
						feature = "suggested-min-age-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"suggestedMinAge",
				#[cfg(any(
					any(feature = "tax-id-property-schema", feature = "general-schema-section"),
					doc
				))]
				"taxID",
				#[cfg(any(
					any(
						feature = "telephone-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"telephone",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
				#[cfg(any(
					any(feature = "vat-id-property-schema", feature = "general-schema-section"),
					doc
				))]
				"vatID",
				#[cfg(any(
					any(feature = "weight-property-schema", feature = "general-schema-section"),
					doc
				))]
				"weight",
				#[cfg(any(
					any(
						feature = "work-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"workLocation",
				#[cfg(any(
					any(
						feature = "works-for-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"worksFor",
			];
			deserializer.deserialize_struct("Patient", FIELDS, ClassVisitor)
		}
	}
}
