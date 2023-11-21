use super::*;
/// <https://schema.org/Patient>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Patient {
	/// <https://schema.org/diagnosis>
	pub r#diagnosis: Vec<DiagnosisProperty>,
	/// <https://schema.org/drug>
	pub r#drug: Vec<DrugProperty>,
	/// <https://schema.org/healthCondition>
	pub r#health_condition: Vec<HealthConditionProperty>,
	/// <https://schema.org/audienceType>
	pub r#audience_type: Vec<AudienceTypeProperty>,
	/// <https://schema.org/geographicArea>
	pub r#geographic_area: Vec<GeographicAreaProperty>,
	/// <https://schema.org/requiredGender>
	pub r#required_gender: Vec<RequiredGenderProperty>,
	/// <https://schema.org/requiredMaxAge>
	pub r#required_max_age: Vec<RequiredMaxAgeProperty>,
	/// <https://schema.org/requiredMinAge>
	pub r#required_min_age: Vec<RequiredMinAgeProperty>,
	/// <https://schema.org/suggestedAge>
	pub r#suggested_age: Vec<SuggestedAgeProperty>,
	/// <https://schema.org/suggestedGender>
	pub r#suggested_gender: Vec<SuggestedGenderProperty>,
	/// <https://schema.org/suggestedMaxAge>
	pub r#suggested_max_age: Vec<SuggestedMaxAgeProperty>,
	/// <https://schema.org/suggestedMeasurement>
	pub r#suggested_measurement: Vec<SuggestedMeasurementProperty>,
	/// <https://schema.org/suggestedMinAge>
	pub r#suggested_min_age: Vec<SuggestedMinAgeProperty>,
	/// <https://schema.org/additionalName>
	pub r#additional_name: Vec<AdditionalNameProperty>,
	/// <https://schema.org/address>
	pub r#address: Vec<AddressProperty>,
	/// <https://schema.org/affiliation>
	pub r#affiliation: Vec<AffiliationProperty>,
	/// <https://schema.org/agentInteractionStatistic>
	pub r#agent_interaction_statistic: Vec<AgentInteractionStatisticProperty>,
	/// <https://schema.org/alumniOf>
	pub r#alumni_of: Vec<AlumniOfProperty>,
	/// <https://schema.org/award>
	pub r#award: Vec<AwardProperty>,
	/// <https://schema.org/awards>
	#[deprecated = "This schema is superseded by <https://schema.org/award>."]
	pub r#awards: Vec<AwardsProperty>,
	/// <https://schema.org/birthDate>
	pub r#birth_date: Vec<BirthDateProperty>,
	/// <https://schema.org/birthPlace>
	pub r#birth_place: Vec<BirthPlaceProperty>,
	/// <https://schema.org/brand>
	pub r#brand: Vec<BrandProperty>,
	/// <https://schema.org/callSign>
	pub r#call_sign: Vec<CallSignProperty>,
	/// <https://schema.org/children>
	pub r#children: Vec<ChildrenProperty>,
	/// <https://schema.org/colleague>
	pub r#colleague: Vec<ColleagueProperty>,
	/// <https://schema.org/colleagues>
	#[deprecated = "This schema is superseded by <https://schema.org/colleague>."]
	pub r#colleagues: Vec<ColleaguesProperty>,
	/// <https://schema.org/contactPoint>
	pub r#contact_point: Vec<ContactPointProperty>,
	/// <https://schema.org/contactPoints>
	#[deprecated = "This schema is superseded by <https://schema.org/contactPoint>."]
	pub r#contact_points: Vec<ContactPointsProperty>,
	/// <https://schema.org/deathDate>
	pub r#death_date: Vec<DeathDateProperty>,
	/// <https://schema.org/deathPlace>
	pub r#death_place: Vec<DeathPlaceProperty>,
	/// <https://schema.org/duns>
	pub r#duns: Vec<DunsProperty>,
	/// <https://schema.org/email>
	pub r#email: Vec<EmailProperty>,
	/// <https://schema.org/familyName>
	pub r#family_name: Vec<FamilyNameProperty>,
	/// <https://schema.org/faxNumber>
	pub r#fax_number: Vec<FaxNumberProperty>,
	/// <https://schema.org/follows>
	pub r#follows: Vec<FollowsProperty>,
	/// <https://schema.org/funder>
	pub r#funder: Vec<FunderProperty>,
	/// <https://schema.org/funding>
	pub r#funding: Vec<FundingProperty>,
	/// <https://schema.org/gender>
	pub r#gender: Vec<GenderProperty>,
	/// <https://schema.org/givenName>
	pub r#given_name: Vec<GivenNameProperty>,
	/// <https://schema.org/globalLocationNumber>
	pub r#global_location_number: Vec<GlobalLocationNumberProperty>,
	/// <https://schema.org/hasCredential>
	pub r#has_credential: Vec<HasCredentialProperty>,
	/// <https://schema.org/hasOccupation>
	pub r#has_occupation: Vec<HasOccupationProperty>,
	/// <https://schema.org/hasOfferCatalog>
	pub r#has_offer_catalog: Vec<HasOfferCatalogProperty>,
	/// <https://schema.org/hasPOS>
	pub r#has_pos: Vec<HasPosProperty>,
	/// <https://schema.org/height>
	pub r#height: Vec<HeightProperty>,
	/// <https://schema.org/homeLocation>
	pub r#home_location: Vec<HomeLocationProperty>,
	/// <https://schema.org/honorificPrefix>
	pub r#honorific_prefix: Vec<HonorificPrefixProperty>,
	/// <https://schema.org/honorificSuffix>
	pub r#honorific_suffix: Vec<HonorificSuffixProperty>,
	/// <https://schema.org/interactionStatistic>
	pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
	/// <https://schema.org/isicV4>
	pub r#isic_v_4: Vec<IsicV4Property>,
	/// <https://schema.org/jobTitle>
	pub r#job_title: Vec<JobTitleProperty>,
	/// <https://schema.org/knows>
	pub r#knows: Vec<KnowsProperty>,
	/// <https://schema.org/knowsAbout>
	pub r#knows_about: Vec<KnowsAboutProperty>,
	/// <https://schema.org/knowsLanguage>
	pub r#knows_language: Vec<KnowsLanguageProperty>,
	/// <https://schema.org/makesOffer>
	pub r#makes_offer: Vec<MakesOfferProperty>,
	/// <https://schema.org/memberOf>
	pub r#member_of: Vec<MemberOfProperty>,
	/// <https://schema.org/naics>
	pub r#naics: Vec<NaicsProperty>,
	/// <https://schema.org/nationality>
	pub r#nationality: Vec<NationalityProperty>,
	/// <https://schema.org/netWorth>
	pub r#net_worth: Vec<NetWorthProperty>,
	/// <https://schema.org/owns>
	pub r#owns: Vec<OwnsProperty>,
	/// <https://schema.org/parent>
	pub r#parent: Vec<ParentProperty>,
	/// <https://schema.org/parents>
	#[deprecated = "This schema is superseded by <https://schema.org/parent>."]
	pub r#parents: Vec<ParentsProperty>,
	/// <https://schema.org/performerIn>
	pub r#performer_in: Vec<PerformerInProperty>,
	/// <https://schema.org/publishingPrinciples>
	pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
	/// <https://schema.org/relatedTo>
	pub r#related_to: Vec<RelatedToProperty>,
	/// <https://schema.org/seeks>
	pub r#seeks: Vec<SeeksProperty>,
	/// <https://schema.org/sibling>
	pub r#sibling: Vec<SiblingProperty>,
	/// <https://schema.org/siblings>
	#[deprecated = "This schema is superseded by <https://schema.org/sibling>."]
	pub r#siblings: Vec<SiblingsProperty>,
	/// <https://schema.org/sponsor>
	pub r#sponsor: Vec<SponsorProperty>,
	/// <https://schema.org/spouse>
	pub r#spouse: Vec<SpouseProperty>,
	/// <https://schema.org/taxID>
	pub r#tax_id: Vec<TaxIdProperty>,
	/// <https://schema.org/telephone>
	pub r#telephone: Vec<TelephoneProperty>,
	/// <https://schema.org/vatID>
	pub r#vat_id: Vec<VatIdProperty>,
	/// <https://schema.org/weight>
	pub r#weight: Vec<WeightProperty>,
	/// <https://schema.org/workLocation>
	pub r#work_location: Vec<WorkLocationProperty>,
	/// <https://schema.org/worksFor>
	pub r#works_for: Vec<WorksForProperty>,
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
/// This trait is for properties from <https://schema.org/Patient>.
pub trait PatientTrait {
	/// Get <https://schema.org/diagnosis> from [`Self`] as borrowed slice.
	fn get_diagnosis(&self) -> &[DiagnosisProperty];
	/// Take <https://schema.org/diagnosis> from [`Self`] as owned vector.
	fn take_diagnosis(&mut self) -> Vec<DiagnosisProperty>;
	/// Get <https://schema.org/drug> from [`Self`] as borrowed slice.
	fn get_drug(&self) -> &[DrugProperty];
	/// Take <https://schema.org/drug> from [`Self`] as owned vector.
	fn take_drug(&mut self) -> Vec<DrugProperty>;
	/// Get <https://schema.org/healthCondition> from [`Self`] as borrowed slice.
	fn get_health_condition(&self) -> &[HealthConditionProperty];
	/// Take <https://schema.org/healthCondition> from [`Self`] as owned vector.
	fn take_health_condition(&mut self) -> Vec<HealthConditionProperty>;
}
impl PatientTrait for Patient {
	fn get_diagnosis(&self) -> &[DiagnosisProperty] {
		self.r#diagnosis.as_slice()
	}
	fn take_diagnosis(&mut self) -> Vec<DiagnosisProperty> {
		std::mem::take(&mut self.r#diagnosis)
	}
	fn get_drug(&self) -> &[DrugProperty] {
		self.r#drug.as_slice()
	}
	fn take_drug(&mut self) -> Vec<DrugProperty> {
		std::mem::take(&mut self.r#drug)
	}
	fn get_health_condition(&self) -> &[HealthConditionProperty] {
		self.r#health_condition.as_slice()
	}
	fn take_health_condition(&mut self) -> Vec<HealthConditionProperty> {
		std::mem::take(&mut self.r#health_condition)
	}
}
impl AudienceTrait for Patient {
	fn get_audience_type(&self) -> &[AudienceTypeProperty] {
		self.r#audience_type.as_slice()
	}
	fn take_audience_type(&mut self) -> Vec<AudienceTypeProperty> {
		std::mem::take(&mut self.r#audience_type)
	}
	fn get_geographic_area(&self) -> &[GeographicAreaProperty] {
		self.r#geographic_area.as_slice()
	}
	fn take_geographic_area(&mut self) -> Vec<GeographicAreaProperty> {
		std::mem::take(&mut self.r#geographic_area)
	}
}
impl MedicalAudienceTrait for Patient {}
impl PeopleAudienceTrait for Patient {
	fn get_health_condition(&self) -> &[HealthConditionProperty] {
		self.r#health_condition.as_slice()
	}
	fn take_health_condition(&mut self) -> Vec<HealthConditionProperty> {
		std::mem::take(&mut self.r#health_condition)
	}
	fn get_required_gender(&self) -> &[RequiredGenderProperty] {
		self.r#required_gender.as_slice()
	}
	fn take_required_gender(&mut self) -> Vec<RequiredGenderProperty> {
		std::mem::take(&mut self.r#required_gender)
	}
	fn get_required_max_age(&self) -> &[RequiredMaxAgeProperty] {
		self.r#required_max_age.as_slice()
	}
	fn take_required_max_age(&mut self) -> Vec<RequiredMaxAgeProperty> {
		std::mem::take(&mut self.r#required_max_age)
	}
	fn get_required_min_age(&self) -> &[RequiredMinAgeProperty] {
		self.r#required_min_age.as_slice()
	}
	fn take_required_min_age(&mut self) -> Vec<RequiredMinAgeProperty> {
		std::mem::take(&mut self.r#required_min_age)
	}
	fn get_suggested_age(&self) -> &[SuggestedAgeProperty] {
		self.r#suggested_age.as_slice()
	}
	fn take_suggested_age(&mut self) -> Vec<SuggestedAgeProperty> {
		std::mem::take(&mut self.r#suggested_age)
	}
	fn get_suggested_gender(&self) -> &[SuggestedGenderProperty] {
		self.r#suggested_gender.as_slice()
	}
	fn take_suggested_gender(&mut self) -> Vec<SuggestedGenderProperty> {
		std::mem::take(&mut self.r#suggested_gender)
	}
	fn get_suggested_max_age(&self) -> &[SuggestedMaxAgeProperty] {
		self.r#suggested_max_age.as_slice()
	}
	fn take_suggested_max_age(&mut self) -> Vec<SuggestedMaxAgeProperty> {
		std::mem::take(&mut self.r#suggested_max_age)
	}
	fn get_suggested_measurement(&self) -> &[SuggestedMeasurementProperty] {
		self.r#suggested_measurement.as_slice()
	}
	fn take_suggested_measurement(&mut self) -> Vec<SuggestedMeasurementProperty> {
		std::mem::take(&mut self.r#suggested_measurement)
	}
	fn get_suggested_min_age(&self) -> &[SuggestedMinAgeProperty] {
		self.r#suggested_min_age.as_slice()
	}
	fn take_suggested_min_age(&mut self) -> Vec<SuggestedMinAgeProperty> {
		std::mem::take(&mut self.r#suggested_min_age)
	}
}
impl PersonTrait for Patient {
	fn get_additional_name(&self) -> &[AdditionalNameProperty] {
		self.r#additional_name.as_slice()
	}
	fn take_additional_name(&mut self) -> Vec<AdditionalNameProperty> {
		std::mem::take(&mut self.r#additional_name)
	}
	fn get_address(&self) -> &[AddressProperty] {
		self.r#address.as_slice()
	}
	fn take_address(&mut self) -> Vec<AddressProperty> {
		std::mem::take(&mut self.r#address)
	}
	fn get_affiliation(&self) -> &[AffiliationProperty] {
		self.r#affiliation.as_slice()
	}
	fn take_affiliation(&mut self) -> Vec<AffiliationProperty> {
		std::mem::take(&mut self.r#affiliation)
	}
	fn get_agent_interaction_statistic(&self) -> &[AgentInteractionStatisticProperty] {
		self.r#agent_interaction_statistic.as_slice()
	}
	fn take_agent_interaction_statistic(&mut self) -> Vec<AgentInteractionStatisticProperty> {
		std::mem::take(&mut self.r#agent_interaction_statistic)
	}
	fn get_alumni_of(&self) -> &[AlumniOfProperty] {
		self.r#alumni_of.as_slice()
	}
	fn take_alumni_of(&mut self) -> Vec<AlumniOfProperty> {
		std::mem::take(&mut self.r#alumni_of)
	}
	fn get_award(&self) -> &[AwardProperty] {
		self.r#award.as_slice()
	}
	fn take_award(&mut self) -> Vec<AwardProperty> {
		std::mem::take(&mut self.r#award)
	}
	fn get_awards(&self) -> &[AwardsProperty] {
		self.r#awards.as_slice()
	}
	fn take_awards(&mut self) -> Vec<AwardsProperty> {
		std::mem::take(&mut self.r#awards)
	}
	fn get_birth_date(&self) -> &[BirthDateProperty] {
		self.r#birth_date.as_slice()
	}
	fn take_birth_date(&mut self) -> Vec<BirthDateProperty> {
		std::mem::take(&mut self.r#birth_date)
	}
	fn get_birth_place(&self) -> &[BirthPlaceProperty] {
		self.r#birth_place.as_slice()
	}
	fn take_birth_place(&mut self) -> Vec<BirthPlaceProperty> {
		std::mem::take(&mut self.r#birth_place)
	}
	fn get_brand(&self) -> &[BrandProperty] {
		self.r#brand.as_slice()
	}
	fn take_brand(&mut self) -> Vec<BrandProperty> {
		std::mem::take(&mut self.r#brand)
	}
	fn get_call_sign(&self) -> &[CallSignProperty] {
		self.r#call_sign.as_slice()
	}
	fn take_call_sign(&mut self) -> Vec<CallSignProperty> {
		std::mem::take(&mut self.r#call_sign)
	}
	fn get_children(&self) -> &[ChildrenProperty] {
		self.r#children.as_slice()
	}
	fn take_children(&mut self) -> Vec<ChildrenProperty> {
		std::mem::take(&mut self.r#children)
	}
	fn get_colleague(&self) -> &[ColleagueProperty] {
		self.r#colleague.as_slice()
	}
	fn take_colleague(&mut self) -> Vec<ColleagueProperty> {
		std::mem::take(&mut self.r#colleague)
	}
	fn get_colleagues(&self) -> &[ColleaguesProperty] {
		self.r#colleagues.as_slice()
	}
	fn take_colleagues(&mut self) -> Vec<ColleaguesProperty> {
		std::mem::take(&mut self.r#colleagues)
	}
	fn get_contact_point(&self) -> &[ContactPointProperty] {
		self.r#contact_point.as_slice()
	}
	fn take_contact_point(&mut self) -> Vec<ContactPointProperty> {
		std::mem::take(&mut self.r#contact_point)
	}
	fn get_contact_points(&self) -> &[ContactPointsProperty] {
		self.r#contact_points.as_slice()
	}
	fn take_contact_points(&mut self) -> Vec<ContactPointsProperty> {
		std::mem::take(&mut self.r#contact_points)
	}
	fn get_death_date(&self) -> &[DeathDateProperty] {
		self.r#death_date.as_slice()
	}
	fn take_death_date(&mut self) -> Vec<DeathDateProperty> {
		std::mem::take(&mut self.r#death_date)
	}
	fn get_death_place(&self) -> &[DeathPlaceProperty] {
		self.r#death_place.as_slice()
	}
	fn take_death_place(&mut self) -> Vec<DeathPlaceProperty> {
		std::mem::take(&mut self.r#death_place)
	}
	fn get_duns(&self) -> &[DunsProperty] {
		self.r#duns.as_slice()
	}
	fn take_duns(&mut self) -> Vec<DunsProperty> {
		std::mem::take(&mut self.r#duns)
	}
	fn get_email(&self) -> &[EmailProperty] {
		self.r#email.as_slice()
	}
	fn take_email(&mut self) -> Vec<EmailProperty> {
		std::mem::take(&mut self.r#email)
	}
	fn get_family_name(&self) -> &[FamilyNameProperty] {
		self.r#family_name.as_slice()
	}
	fn take_family_name(&mut self) -> Vec<FamilyNameProperty> {
		std::mem::take(&mut self.r#family_name)
	}
	fn get_fax_number(&self) -> &[FaxNumberProperty] {
		self.r#fax_number.as_slice()
	}
	fn take_fax_number(&mut self) -> Vec<FaxNumberProperty> {
		std::mem::take(&mut self.r#fax_number)
	}
	fn get_follows(&self) -> &[FollowsProperty] {
		self.r#follows.as_slice()
	}
	fn take_follows(&mut self) -> Vec<FollowsProperty> {
		std::mem::take(&mut self.r#follows)
	}
	fn get_funder(&self) -> &[FunderProperty] {
		self.r#funder.as_slice()
	}
	fn take_funder(&mut self) -> Vec<FunderProperty> {
		std::mem::take(&mut self.r#funder)
	}
	fn get_funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn take_funding(&mut self) -> Vec<FundingProperty> {
		std::mem::take(&mut self.r#funding)
	}
	fn get_gender(&self) -> &[GenderProperty] {
		self.r#gender.as_slice()
	}
	fn take_gender(&mut self) -> Vec<GenderProperty> {
		std::mem::take(&mut self.r#gender)
	}
	fn get_given_name(&self) -> &[GivenNameProperty] {
		self.r#given_name.as_slice()
	}
	fn take_given_name(&mut self) -> Vec<GivenNameProperty> {
		std::mem::take(&mut self.r#given_name)
	}
	fn get_global_location_number(&self) -> &[GlobalLocationNumberProperty] {
		self.r#global_location_number.as_slice()
	}
	fn take_global_location_number(&mut self) -> Vec<GlobalLocationNumberProperty> {
		std::mem::take(&mut self.r#global_location_number)
	}
	fn get_has_credential(&self) -> &[HasCredentialProperty] {
		self.r#has_credential.as_slice()
	}
	fn take_has_credential(&mut self) -> Vec<HasCredentialProperty> {
		std::mem::take(&mut self.r#has_credential)
	}
	fn get_has_occupation(&self) -> &[HasOccupationProperty] {
		self.r#has_occupation.as_slice()
	}
	fn take_has_occupation(&mut self) -> Vec<HasOccupationProperty> {
		std::mem::take(&mut self.r#has_occupation)
	}
	fn get_has_offer_catalog(&self) -> &[HasOfferCatalogProperty] {
		self.r#has_offer_catalog.as_slice()
	}
	fn take_has_offer_catalog(&mut self) -> Vec<HasOfferCatalogProperty> {
		std::mem::take(&mut self.r#has_offer_catalog)
	}
	fn get_has_pos(&self) -> &[HasPosProperty] {
		self.r#has_pos.as_slice()
	}
	fn take_has_pos(&mut self) -> Vec<HasPosProperty> {
		std::mem::take(&mut self.r#has_pos)
	}
	fn get_height(&self) -> &[HeightProperty] {
		self.r#height.as_slice()
	}
	fn take_height(&mut self) -> Vec<HeightProperty> {
		std::mem::take(&mut self.r#height)
	}
	fn get_home_location(&self) -> &[HomeLocationProperty] {
		self.r#home_location.as_slice()
	}
	fn take_home_location(&mut self) -> Vec<HomeLocationProperty> {
		std::mem::take(&mut self.r#home_location)
	}
	fn get_honorific_prefix(&self) -> &[HonorificPrefixProperty] {
		self.r#honorific_prefix.as_slice()
	}
	fn take_honorific_prefix(&mut self) -> Vec<HonorificPrefixProperty> {
		std::mem::take(&mut self.r#honorific_prefix)
	}
	fn get_honorific_suffix(&self) -> &[HonorificSuffixProperty] {
		self.r#honorific_suffix.as_slice()
	}
	fn take_honorific_suffix(&mut self) -> Vec<HonorificSuffixProperty> {
		std::mem::take(&mut self.r#honorific_suffix)
	}
	fn get_interaction_statistic(&self) -> &[InteractionStatisticProperty] {
		self.r#interaction_statistic.as_slice()
	}
	fn take_interaction_statistic(&mut self) -> Vec<InteractionStatisticProperty> {
		std::mem::take(&mut self.r#interaction_statistic)
	}
	fn get_isic_v_4(&self) -> &[IsicV4Property] {
		self.r#isic_v_4.as_slice()
	}
	fn take_isic_v_4(&mut self) -> Vec<IsicV4Property> {
		std::mem::take(&mut self.r#isic_v_4)
	}
	fn get_job_title(&self) -> &[JobTitleProperty] {
		self.r#job_title.as_slice()
	}
	fn take_job_title(&mut self) -> Vec<JobTitleProperty> {
		std::mem::take(&mut self.r#job_title)
	}
	fn get_knows(&self) -> &[KnowsProperty] {
		self.r#knows.as_slice()
	}
	fn take_knows(&mut self) -> Vec<KnowsProperty> {
		std::mem::take(&mut self.r#knows)
	}
	fn get_knows_about(&self) -> &[KnowsAboutProperty] {
		self.r#knows_about.as_slice()
	}
	fn take_knows_about(&mut self) -> Vec<KnowsAboutProperty> {
		std::mem::take(&mut self.r#knows_about)
	}
	fn get_knows_language(&self) -> &[KnowsLanguageProperty] {
		self.r#knows_language.as_slice()
	}
	fn take_knows_language(&mut self) -> Vec<KnowsLanguageProperty> {
		std::mem::take(&mut self.r#knows_language)
	}
	fn get_makes_offer(&self) -> &[MakesOfferProperty] {
		self.r#makes_offer.as_slice()
	}
	fn take_makes_offer(&mut self) -> Vec<MakesOfferProperty> {
		std::mem::take(&mut self.r#makes_offer)
	}
	fn get_member_of(&self) -> &[MemberOfProperty] {
		self.r#member_of.as_slice()
	}
	fn take_member_of(&mut self) -> Vec<MemberOfProperty> {
		std::mem::take(&mut self.r#member_of)
	}
	fn get_naics(&self) -> &[NaicsProperty] {
		self.r#naics.as_slice()
	}
	fn take_naics(&mut self) -> Vec<NaicsProperty> {
		std::mem::take(&mut self.r#naics)
	}
	fn get_nationality(&self) -> &[NationalityProperty] {
		self.r#nationality.as_slice()
	}
	fn take_nationality(&mut self) -> Vec<NationalityProperty> {
		std::mem::take(&mut self.r#nationality)
	}
	fn get_net_worth(&self) -> &[NetWorthProperty] {
		self.r#net_worth.as_slice()
	}
	fn take_net_worth(&mut self) -> Vec<NetWorthProperty> {
		std::mem::take(&mut self.r#net_worth)
	}
	fn get_owns(&self) -> &[OwnsProperty] {
		self.r#owns.as_slice()
	}
	fn take_owns(&mut self) -> Vec<OwnsProperty> {
		std::mem::take(&mut self.r#owns)
	}
	fn get_parent(&self) -> &[ParentProperty] {
		self.r#parent.as_slice()
	}
	fn take_parent(&mut self) -> Vec<ParentProperty> {
		std::mem::take(&mut self.r#parent)
	}
	fn get_parents(&self) -> &[ParentsProperty] {
		self.r#parents.as_slice()
	}
	fn take_parents(&mut self) -> Vec<ParentsProperty> {
		std::mem::take(&mut self.r#parents)
	}
	fn get_performer_in(&self) -> &[PerformerInProperty] {
		self.r#performer_in.as_slice()
	}
	fn take_performer_in(&mut self) -> Vec<PerformerInProperty> {
		std::mem::take(&mut self.r#performer_in)
	}
	fn get_publishing_principles(&self) -> &[PublishingPrinciplesProperty] {
		self.r#publishing_principles.as_slice()
	}
	fn take_publishing_principles(&mut self) -> Vec<PublishingPrinciplesProperty> {
		std::mem::take(&mut self.r#publishing_principles)
	}
	fn get_related_to(&self) -> &[RelatedToProperty] {
		self.r#related_to.as_slice()
	}
	fn take_related_to(&mut self) -> Vec<RelatedToProperty> {
		std::mem::take(&mut self.r#related_to)
	}
	fn get_seeks(&self) -> &[SeeksProperty] {
		self.r#seeks.as_slice()
	}
	fn take_seeks(&mut self) -> Vec<SeeksProperty> {
		std::mem::take(&mut self.r#seeks)
	}
	fn get_sibling(&self) -> &[SiblingProperty] {
		self.r#sibling.as_slice()
	}
	fn take_sibling(&mut self) -> Vec<SiblingProperty> {
		std::mem::take(&mut self.r#sibling)
	}
	fn get_siblings(&self) -> &[SiblingsProperty] {
		self.r#siblings.as_slice()
	}
	fn take_siblings(&mut self) -> Vec<SiblingsProperty> {
		std::mem::take(&mut self.r#siblings)
	}
	fn get_sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn take_sponsor(&mut self) -> Vec<SponsorProperty> {
		std::mem::take(&mut self.r#sponsor)
	}
	fn get_spouse(&self) -> &[SpouseProperty] {
		self.r#spouse.as_slice()
	}
	fn take_spouse(&mut self) -> Vec<SpouseProperty> {
		std::mem::take(&mut self.r#spouse)
	}
	fn get_tax_id(&self) -> &[TaxIdProperty] {
		self.r#tax_id.as_slice()
	}
	fn take_tax_id(&mut self) -> Vec<TaxIdProperty> {
		std::mem::take(&mut self.r#tax_id)
	}
	fn get_telephone(&self) -> &[TelephoneProperty] {
		self.r#telephone.as_slice()
	}
	fn take_telephone(&mut self) -> Vec<TelephoneProperty> {
		std::mem::take(&mut self.r#telephone)
	}
	fn get_vat_id(&self) -> &[VatIdProperty] {
		self.r#vat_id.as_slice()
	}
	fn take_vat_id(&mut self) -> Vec<VatIdProperty> {
		std::mem::take(&mut self.r#vat_id)
	}
	fn get_weight(&self) -> &[WeightProperty] {
		self.r#weight.as_slice()
	}
	fn take_weight(&mut self) -> Vec<WeightProperty> {
		std::mem::take(&mut self.r#weight)
	}
	fn get_work_location(&self) -> &[WorkLocationProperty] {
		self.r#work_location.as_slice()
	}
	fn take_work_location(&mut self) -> Vec<WorkLocationProperty> {
		std::mem::take(&mut self.r#work_location)
	}
	fn get_works_for(&self) -> &[WorksForProperty] {
		self.r#works_for.as_slice()
	}
	fn take_works_for(&mut self) -> Vec<WorksForProperty> {
		std::mem::take(&mut self.r#works_for)
	}
}
impl ThingTrait for Patient {
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
	impl Serialize for Patient {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#diagnosis) as usize,
				!Vec::is_empty(&self.r#drug) as usize,
				!Vec::is_empty(&self.r#health_condition) as usize,
				!Vec::is_empty(&self.r#audience_type) as usize,
				!Vec::is_empty(&self.r#geographic_area) as usize,
				!Vec::is_empty(&self.r#required_gender) as usize,
				!Vec::is_empty(&self.r#required_max_age) as usize,
				!Vec::is_empty(&self.r#required_min_age) as usize,
				!Vec::is_empty(&self.r#suggested_age) as usize,
				!Vec::is_empty(&self.r#suggested_gender) as usize,
				!Vec::is_empty(&self.r#suggested_max_age) as usize,
				!Vec::is_empty(&self.r#suggested_measurement) as usize,
				!Vec::is_empty(&self.r#suggested_min_age) as usize,
				!Vec::is_empty(&self.r#additional_name) as usize,
				!Vec::is_empty(&self.r#address) as usize,
				!Vec::is_empty(&self.r#affiliation) as usize,
				!Vec::is_empty(&self.r#agent_interaction_statistic) as usize,
				!Vec::is_empty(&self.r#alumni_of) as usize,
				!Vec::is_empty(&self.r#award) as usize,
				!Vec::is_empty(&self.r#awards) as usize,
				!Vec::is_empty(&self.r#birth_date) as usize,
				!Vec::is_empty(&self.r#birth_place) as usize,
				!Vec::is_empty(&self.r#brand) as usize,
				!Vec::is_empty(&self.r#call_sign) as usize,
				!Vec::is_empty(&self.r#children) as usize,
				!Vec::is_empty(&self.r#colleague) as usize,
				!Vec::is_empty(&self.r#colleagues) as usize,
				!Vec::is_empty(&self.r#contact_point) as usize,
				!Vec::is_empty(&self.r#contact_points) as usize,
				!Vec::is_empty(&self.r#death_date) as usize,
				!Vec::is_empty(&self.r#death_place) as usize,
				!Vec::is_empty(&self.r#duns) as usize,
				!Vec::is_empty(&self.r#email) as usize,
				!Vec::is_empty(&self.r#family_name) as usize,
				!Vec::is_empty(&self.r#fax_number) as usize,
				!Vec::is_empty(&self.r#follows) as usize,
				!Vec::is_empty(&self.r#funder) as usize,
				!Vec::is_empty(&self.r#funding) as usize,
				!Vec::is_empty(&self.r#gender) as usize,
				!Vec::is_empty(&self.r#given_name) as usize,
				!Vec::is_empty(&self.r#global_location_number) as usize,
				!Vec::is_empty(&self.r#has_credential) as usize,
				!Vec::is_empty(&self.r#has_occupation) as usize,
				!Vec::is_empty(&self.r#has_offer_catalog) as usize,
				!Vec::is_empty(&self.r#has_pos) as usize,
				!Vec::is_empty(&self.r#height) as usize,
				!Vec::is_empty(&self.r#home_location) as usize,
				!Vec::is_empty(&self.r#honorific_prefix) as usize,
				!Vec::is_empty(&self.r#honorific_suffix) as usize,
				!Vec::is_empty(&self.r#interaction_statistic) as usize,
				!Vec::is_empty(&self.r#isic_v_4) as usize,
				!Vec::is_empty(&self.r#job_title) as usize,
				!Vec::is_empty(&self.r#knows) as usize,
				!Vec::is_empty(&self.r#knows_about) as usize,
				!Vec::is_empty(&self.r#knows_language) as usize,
				!Vec::is_empty(&self.r#makes_offer) as usize,
				!Vec::is_empty(&self.r#member_of) as usize,
				!Vec::is_empty(&self.r#naics) as usize,
				!Vec::is_empty(&self.r#nationality) as usize,
				!Vec::is_empty(&self.r#net_worth) as usize,
				!Vec::is_empty(&self.r#owns) as usize,
				!Vec::is_empty(&self.r#parent) as usize,
				!Vec::is_empty(&self.r#parents) as usize,
				!Vec::is_empty(&self.r#performer_in) as usize,
				!Vec::is_empty(&self.r#publishing_principles) as usize,
				!Vec::is_empty(&self.r#related_to) as usize,
				!Vec::is_empty(&self.r#seeks) as usize,
				!Vec::is_empty(&self.r#sibling) as usize,
				!Vec::is_empty(&self.r#siblings) as usize,
				!Vec::is_empty(&self.r#sponsor) as usize,
				!Vec::is_empty(&self.r#spouse) as usize,
				!Vec::is_empty(&self.r#tax_id) as usize,
				!Vec::is_empty(&self.r#telephone) as usize,
				!Vec::is_empty(&self.r#vat_id) as usize,
				!Vec::is_empty(&self.r#weight) as usize,
				!Vec::is_empty(&self.r#work_location) as usize,
				!Vec::is_empty(&self.r#works_for) as usize,
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
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Patient", len)?;
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
	impl<'de> Deserialize<'de> for Patient {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Diagnosis,
				Drug,
				HealthCondition,
				AudienceType,
				GeographicArea,
				RequiredGender,
				RequiredMaxAge,
				RequiredMinAge,
				SuggestedAge,
				SuggestedGender,
				SuggestedMaxAge,
				SuggestedMeasurement,
				SuggestedMinAge,
				AdditionalName,
				Address,
				Affiliation,
				AgentInteractionStatistic,
				AlumniOf,
				Award,
				Awards,
				BirthDate,
				BirthPlace,
				Brand,
				CallSign,
				Children,
				Colleague,
				Colleagues,
				ContactPoint,
				ContactPoints,
				DeathDate,
				DeathPlace,
				Duns,
				Email,
				FamilyName,
				FaxNumber,
				Follows,
				Funder,
				Funding,
				Gender,
				GivenName,
				GlobalLocationNumber,
				HasCredential,
				HasOccupation,
				HasOfferCatalog,
				HasPos,
				Height,
				HomeLocation,
				HonorificPrefix,
				HonorificSuffix,
				InteractionStatistic,
				IsicV4,
				JobTitle,
				Knows,
				KnowsAbout,
				KnowsLanguage,
				MakesOffer,
				MemberOf,
				Naics,
				Nationality,
				NetWorth,
				Owns,
				Parent,
				Parents,
				PerformerIn,
				PublishingPrinciples,
				RelatedTo,
				Seeks,
				Sibling,
				Siblings,
				Sponsor,
				Spouse,
				TaxId,
				Telephone,
				VatId,
				Weight,
				WorkLocation,
				WorksFor,
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
						"diagnosis" => Ok(Field::Diagnosis),
						"drug" => Ok(Field::Drug),
						"healthCondition" => Ok(Field::HealthCondition),
						"audienceType" => Ok(Field::AudienceType),
						"geographicArea" => Ok(Field::GeographicArea),
						"requiredGender" => Ok(Field::RequiredGender),
						"requiredMaxAge" => Ok(Field::RequiredMaxAge),
						"requiredMinAge" => Ok(Field::RequiredMinAge),
						"suggestedAge" => Ok(Field::SuggestedAge),
						"suggestedGender" => Ok(Field::SuggestedGender),
						"suggestedMaxAge" => Ok(Field::SuggestedMaxAge),
						"suggestedMeasurement" => Ok(Field::SuggestedMeasurement),
						"suggestedMinAge" => Ok(Field::SuggestedMinAge),
						"additionalName" => Ok(Field::AdditionalName),
						"address" => Ok(Field::Address),
						"affiliation" => Ok(Field::Affiliation),
						"agentInteractionStatistic" => Ok(Field::AgentInteractionStatistic),
						"alumniOf" => Ok(Field::AlumniOf),
						"award" => Ok(Field::Award),
						"awards" => Ok(Field::Awards),
						"birthDate" => Ok(Field::BirthDate),
						"birthPlace" => Ok(Field::BirthPlace),
						"brand" => Ok(Field::Brand),
						"callSign" => Ok(Field::CallSign),
						"children" => Ok(Field::Children),
						"colleague" => Ok(Field::Colleague),
						"colleagues" => Ok(Field::Colleagues),
						"contactPoint" => Ok(Field::ContactPoint),
						"contactPoints" => Ok(Field::ContactPoints),
						"deathDate" => Ok(Field::DeathDate),
						"deathPlace" => Ok(Field::DeathPlace),
						"duns" => Ok(Field::Duns),
						"email" => Ok(Field::Email),
						"familyName" => Ok(Field::FamilyName),
						"faxNumber" => Ok(Field::FaxNumber),
						"follows" => Ok(Field::Follows),
						"funder" => Ok(Field::Funder),
						"funding" => Ok(Field::Funding),
						"gender" => Ok(Field::Gender),
						"givenName" => Ok(Field::GivenName),
						"globalLocationNumber" => Ok(Field::GlobalLocationNumber),
						"hasCredential" => Ok(Field::HasCredential),
						"hasOccupation" => Ok(Field::HasOccupation),
						"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						"hasPOS" => Ok(Field::HasPos),
						"height" => Ok(Field::Height),
						"homeLocation" => Ok(Field::HomeLocation),
						"honorificPrefix" => Ok(Field::HonorificPrefix),
						"honorificSuffix" => Ok(Field::HonorificSuffix),
						"interactionStatistic" => Ok(Field::InteractionStatistic),
						"isicV4" => Ok(Field::IsicV4),
						"jobTitle" => Ok(Field::JobTitle),
						"knows" => Ok(Field::Knows),
						"knowsAbout" => Ok(Field::KnowsAbout),
						"knowsLanguage" => Ok(Field::KnowsLanguage),
						"makesOffer" => Ok(Field::MakesOffer),
						"memberOf" => Ok(Field::MemberOf),
						"naics" => Ok(Field::Naics),
						"nationality" => Ok(Field::Nationality),
						"netWorth" => Ok(Field::NetWorth),
						"owns" => Ok(Field::Owns),
						"parent" => Ok(Field::Parent),
						"parents" => Ok(Field::Parents),
						"performerIn" => Ok(Field::PerformerIn),
						"publishingPrinciples" => Ok(Field::PublishingPrinciples),
						"relatedTo" => Ok(Field::RelatedTo),
						"seeks" => Ok(Field::Seeks),
						"sibling" => Ok(Field::Sibling),
						"siblings" => Ok(Field::Siblings),
						"sponsor" => Ok(Field::Sponsor),
						"spouse" => Ok(Field::Spouse),
						"taxID" => Ok(Field::TaxId),
						"telephone" => Ok(Field::Telephone),
						"vatID" => Ok(Field::VatId),
						"weight" => Ok(Field::Weight),
						"workLocation" => Ok(Field::WorkLocation),
						"worksFor" => Ok(Field::WorksFor),
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
						_ => Err(de::Error::unknown_field(value, FIELDS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"diagnosis" => Ok(Field::Diagnosis),
						b"drug" => Ok(Field::Drug),
						b"healthCondition" => Ok(Field::HealthCondition),
						b"audienceType" => Ok(Field::AudienceType),
						b"geographicArea" => Ok(Field::GeographicArea),
						b"requiredGender" => Ok(Field::RequiredGender),
						b"requiredMaxAge" => Ok(Field::RequiredMaxAge),
						b"requiredMinAge" => Ok(Field::RequiredMinAge),
						b"suggestedAge" => Ok(Field::SuggestedAge),
						b"suggestedGender" => Ok(Field::SuggestedGender),
						b"suggestedMaxAge" => Ok(Field::SuggestedMaxAge),
						b"suggestedMeasurement" => Ok(Field::SuggestedMeasurement),
						b"suggestedMinAge" => Ok(Field::SuggestedMinAge),
						b"additionalName" => Ok(Field::AdditionalName),
						b"address" => Ok(Field::Address),
						b"affiliation" => Ok(Field::Affiliation),
						b"agentInteractionStatistic" => Ok(Field::AgentInteractionStatistic),
						b"alumniOf" => Ok(Field::AlumniOf),
						b"award" => Ok(Field::Award),
						b"awards" => Ok(Field::Awards),
						b"birthDate" => Ok(Field::BirthDate),
						b"birthPlace" => Ok(Field::BirthPlace),
						b"brand" => Ok(Field::Brand),
						b"callSign" => Ok(Field::CallSign),
						b"children" => Ok(Field::Children),
						b"colleague" => Ok(Field::Colleague),
						b"colleagues" => Ok(Field::Colleagues),
						b"contactPoint" => Ok(Field::ContactPoint),
						b"contactPoints" => Ok(Field::ContactPoints),
						b"deathDate" => Ok(Field::DeathDate),
						b"deathPlace" => Ok(Field::DeathPlace),
						b"duns" => Ok(Field::Duns),
						b"email" => Ok(Field::Email),
						b"familyName" => Ok(Field::FamilyName),
						b"faxNumber" => Ok(Field::FaxNumber),
						b"follows" => Ok(Field::Follows),
						b"funder" => Ok(Field::Funder),
						b"funding" => Ok(Field::Funding),
						b"gender" => Ok(Field::Gender),
						b"givenName" => Ok(Field::GivenName),
						b"globalLocationNumber" => Ok(Field::GlobalLocationNumber),
						b"hasCredential" => Ok(Field::HasCredential),
						b"hasOccupation" => Ok(Field::HasOccupation),
						b"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						b"hasPOS" => Ok(Field::HasPos),
						b"height" => Ok(Field::Height),
						b"homeLocation" => Ok(Field::HomeLocation),
						b"honorificPrefix" => Ok(Field::HonorificPrefix),
						b"honorificSuffix" => Ok(Field::HonorificSuffix),
						b"interactionStatistic" => Ok(Field::InteractionStatistic),
						b"isicV4" => Ok(Field::IsicV4),
						b"jobTitle" => Ok(Field::JobTitle),
						b"knows" => Ok(Field::Knows),
						b"knowsAbout" => Ok(Field::KnowsAbout),
						b"knowsLanguage" => Ok(Field::KnowsLanguage),
						b"makesOffer" => Ok(Field::MakesOffer),
						b"memberOf" => Ok(Field::MemberOf),
						b"naics" => Ok(Field::Naics),
						b"nationality" => Ok(Field::Nationality),
						b"netWorth" => Ok(Field::NetWorth),
						b"owns" => Ok(Field::Owns),
						b"parent" => Ok(Field::Parent),
						b"parents" => Ok(Field::Parents),
						b"performerIn" => Ok(Field::PerformerIn),
						b"publishingPrinciples" => Ok(Field::PublishingPrinciples),
						b"relatedTo" => Ok(Field::RelatedTo),
						b"seeks" => Ok(Field::Seeks),
						b"sibling" => Ok(Field::Sibling),
						b"siblings" => Ok(Field::Siblings),
						b"sponsor" => Ok(Field::Sponsor),
						b"spouse" => Ok(Field::Spouse),
						b"taxID" => Ok(Field::TaxId),
						b"telephone" => Ok(Field::Telephone),
						b"vatID" => Ok(Field::VatId),
						b"weight" => Ok(Field::Weight),
						b"workLocation" => Ok(Field::WorkLocation),
						b"worksFor" => Ok(Field::WorksFor),
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
				type Value = Patient;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Patient")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#diagnosis_property = None;
					let mut r#drug_property = None;
					let mut r#health_condition_property = None;
					let mut r#audience_type_property = None;
					let mut r#geographic_area_property = None;
					let mut r#required_gender_property = None;
					let mut r#required_max_age_property = None;
					let mut r#required_min_age_property = None;
					let mut r#suggested_age_property = None;
					let mut r#suggested_gender_property = None;
					let mut r#suggested_max_age_property = None;
					let mut r#suggested_measurement_property = None;
					let mut r#suggested_min_age_property = None;
					let mut r#additional_name_property = None;
					let mut r#address_property = None;
					let mut r#affiliation_property = None;
					let mut r#agent_interaction_statistic_property = None;
					let mut r#alumni_of_property = None;
					let mut r#award_property = None;
					let mut r#awards_property = None;
					let mut r#birth_date_property = None;
					let mut r#birth_place_property = None;
					let mut r#brand_property = None;
					let mut r#call_sign_property = None;
					let mut r#children_property = None;
					let mut r#colleague_property = None;
					let mut r#colleagues_property = None;
					let mut r#contact_point_property = None;
					let mut r#contact_points_property = None;
					let mut r#death_date_property = None;
					let mut r#death_place_property = None;
					let mut r#duns_property = None;
					let mut r#email_property = None;
					let mut r#family_name_property = None;
					let mut r#fax_number_property = None;
					let mut r#follows_property = None;
					let mut r#funder_property = None;
					let mut r#funding_property = None;
					let mut r#gender_property = None;
					let mut r#given_name_property = None;
					let mut r#global_location_number_property = None;
					let mut r#has_credential_property = None;
					let mut r#has_occupation_property = None;
					let mut r#has_offer_catalog_property = None;
					let mut r#has_pos_property = None;
					let mut r#height_property = None;
					let mut r#home_location_property = None;
					let mut r#honorific_prefix_property = None;
					let mut r#honorific_suffix_property = None;
					let mut r#interaction_statistic_property = None;
					let mut r#isic_v_4_property = None;
					let mut r#job_title_property = None;
					let mut r#knows_property = None;
					let mut r#knows_about_property = None;
					let mut r#knows_language_property = None;
					let mut r#makes_offer_property = None;
					let mut r#member_of_property = None;
					let mut r#naics_property = None;
					let mut r#nationality_property = None;
					let mut r#net_worth_property = None;
					let mut r#owns_property = None;
					let mut r#parent_property = None;
					let mut r#parents_property = None;
					let mut r#performer_in_property = None;
					let mut r#publishing_principles_property = None;
					let mut r#related_to_property = None;
					let mut r#seeks_property = None;
					let mut r#sibling_property = None;
					let mut r#siblings_property = None;
					let mut r#sponsor_property = None;
					let mut r#spouse_property = None;
					let mut r#tax_id_property = None;
					let mut r#telephone_property = None;
					let mut r#vat_id_property = None;
					let mut r#weight_property = None;
					let mut r#work_location_property = None;
					let mut r#works_for_property = None;
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
						}
					}
					Ok(Patient {
						r#diagnosis: r#diagnosis_property.unwrap_or_default(),
						r#drug: r#drug_property.unwrap_or_default(),
						r#health_condition: r#health_condition_property.unwrap_or_default(),
						r#audience_type: r#audience_type_property.unwrap_or_default(),
						r#geographic_area: r#geographic_area_property.unwrap_or_default(),
						r#required_gender: r#required_gender_property.unwrap_or_default(),
						r#required_max_age: r#required_max_age_property.unwrap_or_default(),
						r#required_min_age: r#required_min_age_property.unwrap_or_default(),
						r#suggested_age: r#suggested_age_property.unwrap_or_default(),
						r#suggested_gender: r#suggested_gender_property.unwrap_or_default(),
						r#suggested_max_age: r#suggested_max_age_property.unwrap_or_default(),
						r#suggested_measurement: r#suggested_measurement_property
							.unwrap_or_default(),
						r#suggested_min_age: r#suggested_min_age_property.unwrap_or_default(),
						r#additional_name: r#additional_name_property.unwrap_or_default(),
						r#address: r#address_property.unwrap_or_default(),
						r#affiliation: r#affiliation_property.unwrap_or_default(),
						r#agent_interaction_statistic: r#agent_interaction_statistic_property
							.unwrap_or_default(),
						r#alumni_of: r#alumni_of_property.unwrap_or_default(),
						r#award: r#award_property.unwrap_or_default(),
						r#awards: r#awards_property.unwrap_or_default(),
						r#birth_date: r#birth_date_property.unwrap_or_default(),
						r#birth_place: r#birth_place_property.unwrap_or_default(),
						r#brand: r#brand_property.unwrap_or_default(),
						r#call_sign: r#call_sign_property.unwrap_or_default(),
						r#children: r#children_property.unwrap_or_default(),
						r#colleague: r#colleague_property.unwrap_or_default(),
						r#colleagues: r#colleagues_property.unwrap_or_default(),
						r#contact_point: r#contact_point_property.unwrap_or_default(),
						r#contact_points: r#contact_points_property.unwrap_or_default(),
						r#death_date: r#death_date_property.unwrap_or_default(),
						r#death_place: r#death_place_property.unwrap_or_default(),
						r#duns: r#duns_property.unwrap_or_default(),
						r#email: r#email_property.unwrap_or_default(),
						r#family_name: r#family_name_property.unwrap_or_default(),
						r#fax_number: r#fax_number_property.unwrap_or_default(),
						r#follows: r#follows_property.unwrap_or_default(),
						r#funder: r#funder_property.unwrap_or_default(),
						r#funding: r#funding_property.unwrap_or_default(),
						r#gender: r#gender_property.unwrap_or_default(),
						r#given_name: r#given_name_property.unwrap_or_default(),
						r#global_location_number: r#global_location_number_property
							.unwrap_or_default(),
						r#has_credential: r#has_credential_property.unwrap_or_default(),
						r#has_occupation: r#has_occupation_property.unwrap_or_default(),
						r#has_offer_catalog: r#has_offer_catalog_property.unwrap_or_default(),
						r#has_pos: r#has_pos_property.unwrap_or_default(),
						r#height: r#height_property.unwrap_or_default(),
						r#home_location: r#home_location_property.unwrap_or_default(),
						r#honorific_prefix: r#honorific_prefix_property.unwrap_or_default(),
						r#honorific_suffix: r#honorific_suffix_property.unwrap_or_default(),
						r#interaction_statistic: r#interaction_statistic_property
							.unwrap_or_default(),
						r#isic_v_4: r#isic_v_4_property.unwrap_or_default(),
						r#job_title: r#job_title_property.unwrap_or_default(),
						r#knows: r#knows_property.unwrap_or_default(),
						r#knows_about: r#knows_about_property.unwrap_or_default(),
						r#knows_language: r#knows_language_property.unwrap_or_default(),
						r#makes_offer: r#makes_offer_property.unwrap_or_default(),
						r#member_of: r#member_of_property.unwrap_or_default(),
						r#naics: r#naics_property.unwrap_or_default(),
						r#nationality: r#nationality_property.unwrap_or_default(),
						r#net_worth: r#net_worth_property.unwrap_or_default(),
						r#owns: r#owns_property.unwrap_or_default(),
						r#parent: r#parent_property.unwrap_or_default(),
						r#parents: r#parents_property.unwrap_or_default(),
						r#performer_in: r#performer_in_property.unwrap_or_default(),
						r#publishing_principles: r#publishing_principles_property
							.unwrap_or_default(),
						r#related_to: r#related_to_property.unwrap_or_default(),
						r#seeks: r#seeks_property.unwrap_or_default(),
						r#sibling: r#sibling_property.unwrap_or_default(),
						r#siblings: r#siblings_property.unwrap_or_default(),
						r#sponsor: r#sponsor_property.unwrap_or_default(),
						r#spouse: r#spouse_property.unwrap_or_default(),
						r#tax_id: r#tax_id_property.unwrap_or_default(),
						r#telephone: r#telephone_property.unwrap_or_default(),
						r#vat_id: r#vat_id_property.unwrap_or_default(),
						r#weight: r#weight_property.unwrap_or_default(),
						r#work_location: r#work_location_property.unwrap_or_default(),
						r#works_for: r#works_for_property.unwrap_or_default(),
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
				"diagnosis",
				"drug",
				"healthCondition",
				"audienceType",
				"geographicArea",
				"requiredGender",
				"requiredMaxAge",
				"requiredMinAge",
				"suggestedAge",
				"suggestedGender",
				"suggestedMaxAge",
				"suggestedMeasurement",
				"suggestedMinAge",
				"additionalName",
				"address",
				"affiliation",
				"agentInteractionStatistic",
				"alumniOf",
				"award",
				"awards",
				"birthDate",
				"birthPlace",
				"brand",
				"callSign",
				"children",
				"colleague",
				"colleagues",
				"contactPoint",
				"contactPoints",
				"deathDate",
				"deathPlace",
				"duns",
				"email",
				"familyName",
				"faxNumber",
				"follows",
				"funder",
				"funding",
				"gender",
				"givenName",
				"globalLocationNumber",
				"hasCredential",
				"hasOccupation",
				"hasOfferCatalog",
				"hasPOS",
				"height",
				"homeLocation",
				"honorificPrefix",
				"honorificSuffix",
				"interactionStatistic",
				"isicV4",
				"jobTitle",
				"knows",
				"knowsAbout",
				"knowsLanguage",
				"makesOffer",
				"memberOf",
				"naics",
				"nationality",
				"netWorth",
				"owns",
				"parent",
				"parents",
				"performerIn",
				"publishingPrinciples",
				"relatedTo",
				"seeks",
				"sibling",
				"siblings",
				"sponsor",
				"spouse",
				"taxID",
				"telephone",
				"vatID",
				"weight",
				"workLocation",
				"worksFor",
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
			deserializer.deserialize_struct("Patient", FIELDS, ClassVisitor)
		}
	}
}
