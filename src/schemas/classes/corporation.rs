use super::*;
/// <https://schema.org/Corporation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Corporation {
	#[cfg(any(
		any(
			feature = "actionable-feedback-policy-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#actionable_feedback_policy: Vec<ActionableFeedbackPolicyProperty>,
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
			feature = "agent-interaction-statistic-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#agent_interaction_statistic: Vec<AgentInteractionStatisticProperty>,
	#[cfg(any(
		any(
			feature = "aggregate-rating-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	#[cfg(any(
		any(
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(feature = "alumni-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#alumni: Vec<AlumniProperty>,
	#[cfg(any(
		any(
			feature = "area-served-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#area_served: Vec<AreaServedProperty>,
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
		any(feature = "brand-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#brand: Vec<BrandProperty>,
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
			feature = "corrections-policy-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#corrections_policy: Vec<CorrectionsPolicyProperty>,
	#[cfg(any(
		any(
			feature = "department-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#department: Vec<DepartmentProperty>,
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
			feature = "dissolution-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#dissolution_date: Vec<DissolutionDateProperty>,
	#[cfg(any(
		any(
			feature = "diversity-policy-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#diversity_policy: Vec<DiversityPolicyProperty>,
	#[cfg(any(
		any(
			feature = "diversity-staffing-report-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#diversity_staffing_report: Vec<DiversityStaffingReportProperty>,
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
			feature = "employee-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#employee: Vec<EmployeeProperty>,
	#[cfg(any(
		any(
			feature = "employees-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#employees: Vec<EmployeesProperty>,
	#[cfg(any(
		any(
			feature = "ethics-policy-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#ethics_policy: Vec<EthicsPolicyProperty>,
	#[cfg(any(
		any(feature = "event-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#event: Vec<EventProperty>,
	#[cfg(any(
		any(feature = "events-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#events: Vec<EventsProperty>,
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
			feature = "founder-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#founder: Vec<FounderProperty>,
	#[cfg(any(
		any(
			feature = "founders-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#founders: Vec<FoundersProperty>,
	#[cfg(any(
		any(
			feature = "founding-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#founding_date: Vec<FoundingDateProperty>,
	#[cfg(any(
		any(
			feature = "founding-location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#founding_location: Vec<FoundingLocationProperty>,
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
			feature = "has-merchant-return-policy-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
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
			feature = "has-product-return-policy-property-schema",
			feature = "attic-schema-section"
		),
		doc
	))]
	pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
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
			feature = "iso-6523-code-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#iso_6523_code: Vec<Iso6523CodeProperty>,
	#[cfg(any(
		any(
			feature = "keywords-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#keywords: Vec<KeywordsProperty>,
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
			feature = "legal-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#legal_name: Vec<LegalNameProperty>,
	#[cfg(any(
		any(
			feature = "lei-code-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#lei_code: Vec<LeiCodeProperty>,
	#[cfg(any(
		any(
			feature = "location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#location: Vec<LocationProperty>,
	#[cfg(any(
		any(feature = "logo-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#logo: Vec<LogoProperty>,
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
		any(feature = "member-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#member: Vec<MemberProperty>,
	#[cfg(any(
		any(
			feature = "member-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#member_of: Vec<MemberOfProperty>,
	#[cfg(any(
		any(
			feature = "members-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#members: Vec<MembersProperty>,
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
			feature = "nonprofit-status-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#nonprofit_status: Vec<NonprofitStatusProperty>,
	#[cfg(any(
		any(
			feature = "number-of-employees-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#number_of_employees: Vec<NumberOfEmployeesProperty>,
	#[cfg(any(
		any(
			feature = "ownership-funding-info-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#ownership_funding_info: Vec<OwnershipFundingInfoProperty>,
	#[cfg(any(
		any(feature = "owns-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#owns: Vec<OwnsProperty>,
	#[cfg(any(
		any(
			feature = "parent-organization-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#parent_organization: Vec<ParentOrganizationProperty>,
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
		any(feature = "review-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#review: Vec<ReviewProperty>,
	#[cfg(any(
		any(
			feature = "reviews-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#reviews: Vec<ReviewsProperty>,
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
			feature = "service-area-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#service_area: Vec<ServiceAreaProperty>,
	#[cfg(any(
		any(feature = "slogan-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#slogan: Vec<SloganProperty>,
	#[cfg(any(
		any(
			feature = "sponsor-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#sponsor: Vec<SponsorProperty>,
	#[cfg(any(
		any(
			feature = "sub-organization-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#sub_organization: Vec<SubOrganizationProperty>,
	#[cfg(any(
		any(
			feature = "subject-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#subject_of: Vec<SubjectOfProperty>,
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
		any(
			feature = "ticker-symbol-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#ticker_symbol: Vec<TickerSymbolProperty>,
	#[cfg(any(
		any(
			feature = "unnamed-sources-policy-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#unnamed_sources_policy: Vec<UnnamedSourcesPolicyProperty>,
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
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Corporation {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "actionable-feedback-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#actionable_feedback_policy) as usize
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
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#aggregate_rating) as usize
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
						feature = "alumni-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#alumni) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "area-served-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#area_served) as usize
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
						feature = "corrections-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#corrections_policy) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "department-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#department) as usize
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
						feature = "dissolution-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#dissolution_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "diversity-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#diversity_policy) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "diversity-staffing-report-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#diversity_staffing_report) as usize
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
						feature = "employee-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#employee) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "employees-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#employees) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "ethics-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#ethics_policy) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "event-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#event) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "events-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#events) as usize
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
						feature = "founder-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#founder) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "founders-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#founders) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "founding-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#founding_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "founding-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#founding_location) as usize
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
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_merchant_return_policy) as usize
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
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_product_return_policy) as usize
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
						feature = "iso-6523-code-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#iso_6523_code) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#keywords) as usize
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
						feature = "legal-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#legal_name) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "lei-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#lei_code) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "location-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#location) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "logo-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#logo) as usize
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
						feature = "member-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#member) as usize
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
						feature = "members-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#members) as usize
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
						feature = "nonprofit-status-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#nonprofit_status) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "number-of-employees-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_employees) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "ownership-funding-info-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#ownership_funding_info) as usize
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
						feature = "parent-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#parent_organization) as usize
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
						feature = "review-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#review) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#reviews) as usize
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
						feature = "service-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#service_area) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "slogan-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#slogan) as usize
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
						feature = "sub-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sub_organization) as usize
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
						feature = "ticker-symbol-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#ticker_symbol) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "unnamed-sources-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#unnamed_sources_policy) as usize
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
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "Corporation", len)?;
			#[cfg(any(
				any(
					feature = "actionable-feedback-policy-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#actionable_feedback_policy) {
				serialize_struct.serialize_field("actionableFeedbackPolicy", {
					struct SerializeWith<'a>(&'a Vec<ActionableFeedbackPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#actionable_feedback_policy)
				})?;
			} else {
				serialize_struct.skip_field("actionableFeedbackPolicy")?;
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
					feature = "aggregate-rating-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#aggregate_rating) {
				serialize_struct.serialize_field("aggregateRating", {
					struct SerializeWith<'a>(&'a Vec<AggregateRatingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#aggregate_rating)
				})?;
			} else {
				serialize_struct.skip_field("aggregateRating")?;
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
				any(feature = "alumni-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#alumni) {
				serialize_struct.serialize_field("alumni", {
					struct SerializeWith<'a>(&'a Vec<AlumniProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#alumni)
				})?;
			} else {
				serialize_struct.skip_field("alumni")?;
			}
			#[cfg(any(
				any(
					feature = "area-served-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#area_served) {
				serialize_struct.serialize_field("areaServed", {
					struct SerializeWith<'a>(&'a Vec<AreaServedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#area_served)
				})?;
			} else {
				serialize_struct.skip_field("areaServed")?;
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
					feature = "corrections-policy-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#corrections_policy) {
				serialize_struct.serialize_field("correctionsPolicy", {
					struct SerializeWith<'a>(&'a Vec<CorrectionsPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#corrections_policy)
				})?;
			} else {
				serialize_struct.skip_field("correctionsPolicy")?;
			}
			#[cfg(any(
				any(
					feature = "department-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#department) {
				serialize_struct.serialize_field("department", {
					struct SerializeWith<'a>(&'a Vec<DepartmentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#department)
				})?;
			} else {
				serialize_struct.skip_field("department")?;
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
					feature = "dissolution-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#dissolution_date) {
				serialize_struct.serialize_field("dissolutionDate", {
					struct SerializeWith<'a>(&'a Vec<DissolutionDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#dissolution_date)
				})?;
			} else {
				serialize_struct.skip_field("dissolutionDate")?;
			}
			#[cfg(any(
				any(
					feature = "diversity-policy-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#diversity_policy) {
				serialize_struct.serialize_field("diversityPolicy", {
					struct SerializeWith<'a>(&'a Vec<DiversityPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#diversity_policy)
				})?;
			} else {
				serialize_struct.skip_field("diversityPolicy")?;
			}
			#[cfg(any(
				any(
					feature = "diversity-staffing-report-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#diversity_staffing_report) {
				serialize_struct.serialize_field("diversityStaffingReport", {
					struct SerializeWith<'a>(&'a Vec<DiversityStaffingReportProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#diversity_staffing_report)
				})?;
			} else {
				serialize_struct.skip_field("diversityStaffingReport")?;
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
					feature = "employee-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#employee) {
				serialize_struct.serialize_field("employee", {
					struct SerializeWith<'a>(&'a Vec<EmployeeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#employee)
				})?;
			} else {
				serialize_struct.skip_field("employee")?;
			}
			#[cfg(any(
				any(
					feature = "employees-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#employees) {
				serialize_struct.serialize_field("employees", {
					struct SerializeWith<'a>(&'a Vec<EmployeesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#employees)
				})?;
			} else {
				serialize_struct.skip_field("employees")?;
			}
			#[cfg(any(
				any(
					feature = "ethics-policy-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#ethics_policy) {
				serialize_struct.serialize_field("ethicsPolicy", {
					struct SerializeWith<'a>(&'a Vec<EthicsPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#ethics_policy)
				})?;
			} else {
				serialize_struct.skip_field("ethicsPolicy")?;
			}
			#[cfg(any(
				any(feature = "event-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#event) {
				serialize_struct.serialize_field("event", {
					struct SerializeWith<'a>(&'a Vec<EventProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#event)
				})?;
			} else {
				serialize_struct.skip_field("event")?;
			}
			#[cfg(any(
				any(feature = "events-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#events) {
				serialize_struct.serialize_field("events", {
					struct SerializeWith<'a>(&'a Vec<EventsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#events)
				})?;
			} else {
				serialize_struct.skip_field("events")?;
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
					feature = "founder-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#founder) {
				serialize_struct.serialize_field("founder", {
					struct SerializeWith<'a>(&'a Vec<FounderProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#founder)
				})?;
			} else {
				serialize_struct.skip_field("founder")?;
			}
			#[cfg(any(
				any(
					feature = "founders-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#founders) {
				serialize_struct.serialize_field("founders", {
					struct SerializeWith<'a>(&'a Vec<FoundersProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#founders)
				})?;
			} else {
				serialize_struct.skip_field("founders")?;
			}
			#[cfg(any(
				any(
					feature = "founding-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#founding_date) {
				serialize_struct.serialize_field("foundingDate", {
					struct SerializeWith<'a>(&'a Vec<FoundingDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#founding_date)
				})?;
			} else {
				serialize_struct.skip_field("foundingDate")?;
			}
			#[cfg(any(
				any(
					feature = "founding-location-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#founding_location) {
				serialize_struct.serialize_field("foundingLocation", {
					struct SerializeWith<'a>(&'a Vec<FoundingLocationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#founding_location)
				})?;
			} else {
				serialize_struct.skip_field("foundingLocation")?;
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
					feature = "has-merchant-return-policy-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_merchant_return_policy) {
				serialize_struct.serialize_field("hasMerchantReturnPolicy", {
					struct SerializeWith<'a>(&'a Vec<HasMerchantReturnPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_merchant_return_policy)
				})?;
			} else {
				serialize_struct.skip_field("hasMerchantReturnPolicy")?;
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
					feature = "has-product-return-policy-property-schema",
					feature = "attic-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_product_return_policy) {
				serialize_struct.serialize_field("hasProductReturnPolicy", {
					struct SerializeWith<'a>(&'a Vec<HasProductReturnPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_product_return_policy)
				})?;
			} else {
				serialize_struct.skip_field("hasProductReturnPolicy")?;
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
					feature = "iso-6523-code-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#iso_6523_code) {
				serialize_struct.serialize_field("iso6523Code", {
					struct SerializeWith<'a>(&'a Vec<Iso6523CodeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#iso_6523_code)
				})?;
			} else {
				serialize_struct.skip_field("iso6523Code")?;
			}
			#[cfg(any(
				any(
					feature = "keywords-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#keywords) {
				serialize_struct.serialize_field("keywords", {
					struct SerializeWith<'a>(&'a Vec<KeywordsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#keywords)
				})?;
			} else {
				serialize_struct.skip_field("keywords")?;
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
					feature = "legal-name-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#legal_name) {
				serialize_struct.serialize_field("legalName", {
					struct SerializeWith<'a>(&'a Vec<LegalNameProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legal_name)
				})?;
			} else {
				serialize_struct.skip_field("legalName")?;
			}
			#[cfg(any(
				any(
					feature = "lei-code-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#lei_code) {
				serialize_struct.serialize_field("leiCode", {
					struct SerializeWith<'a>(&'a Vec<LeiCodeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#lei_code)
				})?;
			} else {
				serialize_struct.skip_field("leiCode")?;
			}
			#[cfg(any(
				any(
					feature = "location-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#location) {
				serialize_struct.serialize_field("location", {
					struct SerializeWith<'a>(&'a Vec<LocationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#location)
				})?;
			} else {
				serialize_struct.skip_field("location")?;
			}
			#[cfg(any(
				any(feature = "logo-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#logo) {
				serialize_struct.serialize_field("logo", {
					struct SerializeWith<'a>(&'a Vec<LogoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#logo)
				})?;
			} else {
				serialize_struct.skip_field("logo")?;
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
				any(feature = "member-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#member) {
				serialize_struct.serialize_field("member", {
					struct SerializeWith<'a>(&'a Vec<MemberProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#member)
				})?;
			} else {
				serialize_struct.skip_field("member")?;
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
				any(
					feature = "members-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#members) {
				serialize_struct.serialize_field("members", {
					struct SerializeWith<'a>(&'a Vec<MembersProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#members)
				})?;
			} else {
				serialize_struct.skip_field("members")?;
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
					feature = "nonprofit-status-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#nonprofit_status) {
				serialize_struct.serialize_field("nonprofitStatus", {
					struct SerializeWith<'a>(&'a Vec<NonprofitStatusProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#nonprofit_status)
				})?;
			} else {
				serialize_struct.skip_field("nonprofitStatus")?;
			}
			#[cfg(any(
				any(
					feature = "number-of-employees-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#number_of_employees) {
				serialize_struct.serialize_field("numberOfEmployees", {
					struct SerializeWith<'a>(&'a Vec<NumberOfEmployeesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#number_of_employees)
				})?;
			} else {
				serialize_struct.skip_field("numberOfEmployees")?;
			}
			#[cfg(any(
				any(
					feature = "ownership-funding-info-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#ownership_funding_info) {
				serialize_struct.serialize_field("ownershipFundingInfo", {
					struct SerializeWith<'a>(&'a Vec<OwnershipFundingInfoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#ownership_funding_info)
				})?;
			} else {
				serialize_struct.skip_field("ownershipFundingInfo")?;
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
				any(
					feature = "parent-organization-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#parent_organization) {
				serialize_struct.serialize_field("parentOrganization", {
					struct SerializeWith<'a>(&'a Vec<ParentOrganizationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#parent_organization)
				})?;
			} else {
				serialize_struct.skip_field("parentOrganization")?;
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
				any(feature = "review-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#review) {
				serialize_struct.serialize_field("review", {
					struct SerializeWith<'a>(&'a Vec<ReviewProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#review)
				})?;
			} else {
				serialize_struct.skip_field("review")?;
			}
			#[cfg(any(
				any(
					feature = "reviews-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#reviews) {
				serialize_struct.serialize_field("reviews", {
					struct SerializeWith<'a>(&'a Vec<ReviewsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#reviews)
				})?;
			} else {
				serialize_struct.skip_field("reviews")?;
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
					feature = "service-area-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#service_area) {
				serialize_struct.serialize_field("serviceArea", {
					struct SerializeWith<'a>(&'a Vec<ServiceAreaProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#service_area)
				})?;
			} else {
				serialize_struct.skip_field("serviceArea")?;
			}
			#[cfg(any(
				any(feature = "slogan-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#slogan) {
				serialize_struct.serialize_field("slogan", {
					struct SerializeWith<'a>(&'a Vec<SloganProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#slogan)
				})?;
			} else {
				serialize_struct.skip_field("slogan")?;
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
				any(
					feature = "sub-organization-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#sub_organization) {
				serialize_struct.serialize_field("subOrganization", {
					struct SerializeWith<'a>(&'a Vec<SubOrganizationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sub_organization)
				})?;
			} else {
				serialize_struct.skip_field("subOrganization")?;
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
				any(
					feature = "ticker-symbol-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#ticker_symbol) {
				serialize_struct.serialize_field("tickerSymbol", {
					struct SerializeWith<'a>(&'a Vec<TickerSymbolProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#ticker_symbol)
				})?;
			} else {
				serialize_struct.skip_field("tickerSymbol")?;
			}
			#[cfg(any(
				any(
					feature = "unnamed-sources-policy-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#unnamed_sources_policy) {
				serialize_struct.serialize_field("unnamedSourcesPolicy", {
					struct SerializeWith<'a>(&'a Vec<UnnamedSourcesPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#unnamed_sources_policy)
				})?;
			} else {
				serialize_struct.skip_field("unnamedSourcesPolicy")?;
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
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Corporation {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(
						feature = "actionable-feedback-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ActionableFeedbackPolicy,
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
						feature = "agent-interaction-statistic-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				AgentInteractionStatistic,
				#[cfg(any(
					any(
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AggregateRating,
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternateName,
				#[cfg(any(
					any(feature = "alumni-property-schema", feature = "general-schema-section"),
					doc
				))]
				Alumni,
				#[cfg(any(
					any(
						feature = "area-served-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AreaServed,
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
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				Brand,
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
						feature = "corrections-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CorrectionsPolicy,
				#[cfg(any(
					any(
						feature = "department-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Department,
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
						feature = "dissolution-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DissolutionDate,
				#[cfg(any(
					any(
						feature = "diversity-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				DiversityPolicy,
				#[cfg(any(
					any(
						feature = "diversity-staffing-report-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				DiversityStaffingReport,
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
						feature = "employee-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Employee,
				#[cfg(any(
					any(
						feature = "employees-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Employees,
				#[cfg(any(
					any(
						feature = "ethics-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EthicsPolicy,
				#[cfg(any(
					any(feature = "event-property-schema", feature = "general-schema-section"),
					doc
				))]
				Event,
				#[cfg(any(
					any(feature = "events-property-schema", feature = "general-schema-section"),
					doc
				))]
				Events,
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
						feature = "founder-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Founder,
				#[cfg(any(
					any(
						feature = "founders-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Founders,
				#[cfg(any(
					any(
						feature = "founding-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FoundingDate,
				#[cfg(any(
					any(
						feature = "founding-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FoundingLocation,
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
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasMerchantReturnPolicy,
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
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				))]
				HasProductReturnPolicy,
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
						feature = "iso-6523-code-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Iso6523Code,
				#[cfg(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Keywords,
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
						feature = "legal-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				LegalName,
				#[cfg(any(
					any(
						feature = "lei-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				LeiCode,
				#[cfg(any(
					any(
						feature = "location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Location,
				#[cfg(any(
					any(feature = "logo-property-schema", feature = "general-schema-section"),
					doc
				))]
				Logo,
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
					any(feature = "member-property-schema", feature = "general-schema-section"),
					doc
				))]
				Member,
				#[cfg(any(
					any(
						feature = "member-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MemberOf,
				#[cfg(any(
					any(
						feature = "members-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Members,
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
						feature = "nonprofit-status-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NonprofitStatus,
				#[cfg(any(
					any(
						feature = "number-of-employees-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				NumberOfEmployees,
				#[cfg(any(
					any(
						feature = "ownership-funding-info-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				OwnershipFundingInfo,
				#[cfg(any(
					any(feature = "owns-property-schema", feature = "general-schema-section"),
					doc
				))]
				Owns,
				#[cfg(any(
					any(
						feature = "parent-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ParentOrganization,
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
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				Review,
				#[cfg(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Reviews,
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
						feature = "service-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ServiceArea,
				#[cfg(any(
					any(feature = "slogan-property-schema", feature = "general-schema-section"),
					doc
				))]
				Slogan,
				#[cfg(any(
					any(
						feature = "sponsor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Sponsor,
				#[cfg(any(
					any(
						feature = "sub-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubOrganization,
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubjectOf,
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
					any(
						feature = "ticker-symbol-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				TickerSymbol,
				#[cfg(any(
					any(
						feature = "unnamed-sources-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				UnnamedSourcesPolicy,
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
								feature = "actionable-feedback-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"actionableFeedbackPolicy" => Ok(Field::ActionableFeedbackPolicy),
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
								feature = "agent-interaction-statistic-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"agentInteractionStatistic" => Ok(Field::AgentInteractionStatistic),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"aggregateRating" => Ok(Field::AggregateRating),
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
								feature = "alumni-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alumni" => Ok(Field::Alumni),
						#[cfg(any(
							any(
								feature = "area-served-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"areaServed" => Ok(Field::AreaServed),
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
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"brand" => Ok(Field::Brand),
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
								feature = "corrections-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"correctionsPolicy" => Ok(Field::CorrectionsPolicy),
						#[cfg(any(
							any(
								feature = "department-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"department" => Ok(Field::Department),
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
								feature = "dissolution-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"dissolutionDate" => Ok(Field::DissolutionDate),
						#[cfg(any(
							any(
								feature = "diversity-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"diversityPolicy" => Ok(Field::DiversityPolicy),
						#[cfg(any(
							any(
								feature = "diversity-staffing-report-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"diversityStaffingReport" => Ok(Field::DiversityStaffingReport),
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
								feature = "employee-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"employee" => Ok(Field::Employee),
						#[cfg(any(
							any(
								feature = "employees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"employees" => Ok(Field::Employees),
						#[cfg(any(
							any(
								feature = "ethics-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"ethicsPolicy" => Ok(Field::EthicsPolicy),
						#[cfg(any(
							any(
								feature = "event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"event" => Ok(Field::Event),
						#[cfg(any(
							any(
								feature = "events-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"events" => Ok(Field::Events),
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
								feature = "founder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"founder" => Ok(Field::Founder),
						#[cfg(any(
							any(
								feature = "founders-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"founders" => Ok(Field::Founders),
						#[cfg(any(
							any(
								feature = "founding-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"foundingDate" => Ok(Field::FoundingDate),
						#[cfg(any(
							any(
								feature = "founding-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"foundingLocation" => Ok(Field::FoundingLocation),
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
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
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
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
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
								feature = "iso-6523-code-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"iso6523Code" => Ok(Field::Iso6523Code),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"keywords" => Ok(Field::Keywords),
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
								feature = "legal-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"legalName" => Ok(Field::LegalName),
						#[cfg(any(
							any(
								feature = "lei-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"leiCode" => Ok(Field::LeiCode),
						#[cfg(any(
							any(
								feature = "location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"location" => Ok(Field::Location),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"logo" => Ok(Field::Logo),
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
								feature = "member-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"member" => Ok(Field::Member),
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
								feature = "members-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"members" => Ok(Field::Members),
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
								feature = "nonprofit-status-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"nonprofitStatus" => Ok(Field::NonprofitStatus),
						#[cfg(any(
							any(
								feature = "number-of-employees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"numberOfEmployees" => Ok(Field::NumberOfEmployees),
						#[cfg(any(
							any(
								feature = "ownership-funding-info-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"ownershipFundingInfo" => Ok(Field::OwnershipFundingInfo),
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
								feature = "parent-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"parentOrganization" => Ok(Field::ParentOrganization),
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
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"review" => Ok(Field::Review),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"reviews" => Ok(Field::Reviews),
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
								feature = "service-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"serviceArea" => Ok(Field::ServiceArea),
						#[cfg(any(
							any(
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"slogan" => Ok(Field::Slogan),
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
								feature = "sub-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"subOrganization" => Ok(Field::SubOrganization),
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
								feature = "ticker-symbol-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"tickerSymbol" => Ok(Field::TickerSymbol),
						#[cfg(any(
							any(
								feature = "unnamed-sources-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"unnamedSourcesPolicy" => Ok(Field::UnnamedSourcesPolicy),
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
								feature = "actionable-feedback-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"actionableFeedbackPolicy" => Ok(Field::ActionableFeedbackPolicy),
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
								feature = "agent-interaction-statistic-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"agentInteractionStatistic" => Ok(Field::AgentInteractionStatistic),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"aggregateRating" => Ok(Field::AggregateRating),
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
								feature = "alumni-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alumni" => Ok(Field::Alumni),
						#[cfg(any(
							any(
								feature = "area-served-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"areaServed" => Ok(Field::AreaServed),
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
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"brand" => Ok(Field::Brand),
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
								feature = "corrections-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"correctionsPolicy" => Ok(Field::CorrectionsPolicy),
						#[cfg(any(
							any(
								feature = "department-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"department" => Ok(Field::Department),
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
								feature = "dissolution-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"dissolutionDate" => Ok(Field::DissolutionDate),
						#[cfg(any(
							any(
								feature = "diversity-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"diversityPolicy" => Ok(Field::DiversityPolicy),
						#[cfg(any(
							any(
								feature = "diversity-staffing-report-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"diversityStaffingReport" => Ok(Field::DiversityStaffingReport),
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
								feature = "employee-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"employee" => Ok(Field::Employee),
						#[cfg(any(
							any(
								feature = "employees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"employees" => Ok(Field::Employees),
						#[cfg(any(
							any(
								feature = "ethics-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"ethicsPolicy" => Ok(Field::EthicsPolicy),
						#[cfg(any(
							any(
								feature = "event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"event" => Ok(Field::Event),
						#[cfg(any(
							any(
								feature = "events-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"events" => Ok(Field::Events),
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
								feature = "founder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"founder" => Ok(Field::Founder),
						#[cfg(any(
							any(
								feature = "founders-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"founders" => Ok(Field::Founders),
						#[cfg(any(
							any(
								feature = "founding-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"foundingDate" => Ok(Field::FoundingDate),
						#[cfg(any(
							any(
								feature = "founding-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"foundingLocation" => Ok(Field::FoundingLocation),
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
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
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
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						b"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
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
								feature = "iso-6523-code-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"iso6523Code" => Ok(Field::Iso6523Code),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"keywords" => Ok(Field::Keywords),
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
								feature = "legal-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"legalName" => Ok(Field::LegalName),
						#[cfg(any(
							any(
								feature = "lei-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"leiCode" => Ok(Field::LeiCode),
						#[cfg(any(
							any(
								feature = "location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"location" => Ok(Field::Location),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"logo" => Ok(Field::Logo),
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
								feature = "member-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"member" => Ok(Field::Member),
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
								feature = "members-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"members" => Ok(Field::Members),
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
								feature = "nonprofit-status-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"nonprofitStatus" => Ok(Field::NonprofitStatus),
						#[cfg(any(
							any(
								feature = "number-of-employees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"numberOfEmployees" => Ok(Field::NumberOfEmployees),
						#[cfg(any(
							any(
								feature = "ownership-funding-info-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"ownershipFundingInfo" => Ok(Field::OwnershipFundingInfo),
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
								feature = "parent-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"parentOrganization" => Ok(Field::ParentOrganization),
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
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"review" => Ok(Field::Review),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"reviews" => Ok(Field::Reviews),
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
								feature = "service-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"serviceArea" => Ok(Field::ServiceArea),
						#[cfg(any(
							any(
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"slogan" => Ok(Field::Slogan),
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
								feature = "sub-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"subOrganization" => Ok(Field::SubOrganization),
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
								feature = "ticker-symbol-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"tickerSymbol" => Ok(Field::TickerSymbol),
						#[cfg(any(
							any(
								feature = "unnamed-sources-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"unnamedSourcesPolicy" => Ok(Field::UnnamedSourcesPolicy),
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
				type Value = Corporation;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Corporation")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(
							feature = "actionable-feedback-policy-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#actionable_feedback_policy_property = None;
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
							feature = "agent-interaction-statistic-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#agent_interaction_statistic_property = None;
					#[cfg(any(
						any(
							feature = "aggregate-rating-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#aggregate_rating_property = None;
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
							feature = "alumni-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alumni_property = None;
					#[cfg(any(
						any(
							feature = "area-served-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#area_served_property = None;
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
						any(feature = "brand-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#brand_property = None;
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
							feature = "corrections-policy-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#corrections_policy_property = None;
					#[cfg(any(
						any(
							feature = "department-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#department_property = None;
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
							feature = "dissolution-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#dissolution_date_property = None;
					#[cfg(any(
						any(
							feature = "diversity-policy-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#diversity_policy_property = None;
					#[cfg(any(
						any(
							feature = "diversity-staffing-report-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#diversity_staffing_report_property = None;
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
							feature = "employee-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#employee_property = None;
					#[cfg(any(
						any(
							feature = "employees-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#employees_property = None;
					#[cfg(any(
						any(
							feature = "ethics-policy-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#ethics_policy_property = None;
					#[cfg(any(
						any(feature = "event-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#event_property = None;
					#[cfg(any(
						any(
							feature = "events-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#events_property = None;
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
							feature = "founder-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#founder_property = None;
					#[cfg(any(
						any(
							feature = "founders-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#founders_property = None;
					#[cfg(any(
						any(
							feature = "founding-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#founding_date_property = None;
					#[cfg(any(
						any(
							feature = "founding-location-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#founding_location_property = None;
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
							feature = "has-merchant-return-policy-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_merchant_return_policy_property = None;
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
							feature = "has-product-return-policy-property-schema",
							feature = "attic-schema-section"
						),
						doc
					))]
					let mut r#has_product_return_policy_property = None;
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
							feature = "iso-6523-code-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#iso_6523_code_property = None;
					#[cfg(any(
						any(
							feature = "keywords-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#keywords_property = None;
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
							feature = "legal-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#legal_name_property = None;
					#[cfg(any(
						any(
							feature = "lei-code-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#lei_code_property = None;
					#[cfg(any(
						any(
							feature = "location-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#location_property = None;
					#[cfg(any(
						any(feature = "logo-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#logo_property = None;
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
							feature = "member-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#member_property = None;
					#[cfg(any(
						any(
							feature = "member-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#member_of_property = None;
					#[cfg(any(
						any(
							feature = "members-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#members_property = None;
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
							feature = "nonprofit-status-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#nonprofit_status_property = None;
					#[cfg(any(
						any(
							feature = "number-of-employees-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#number_of_employees_property = None;
					#[cfg(any(
						any(
							feature = "ownership-funding-info-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#ownership_funding_info_property = None;
					#[cfg(any(
						any(feature = "owns-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#owns_property = None;
					#[cfg(any(
						any(
							feature = "parent-organization-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#parent_organization_property = None;
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
							feature = "review-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#review_property = None;
					#[cfg(any(
						any(
							feature = "reviews-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#reviews_property = None;
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
							feature = "service-area-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#service_area_property = None;
					#[cfg(any(
						any(
							feature = "slogan-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#slogan_property = None;
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
							feature = "sub-organization-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#sub_organization_property = None;
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
						any(
							feature = "ticker-symbol-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#ticker_symbol_property = None;
					#[cfg(any(
						any(
							feature = "unnamed-sources-policy-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#unnamed_sources_policy_property = None;
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
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "actionable-feedback-policy-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ActionableFeedbackPolicy => {
								if r#actionable_feedback_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"actionableFeedbackPolicy",
									));
								}
								r#actionable_feedback_policy_property = Some({
									struct DeserializeWith(Vec<ActionableFeedbackPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "aggregate-rating-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AggregateRating => {
								if r#aggregate_rating_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"aggregateRating",
									));
								}
								r#aggregate_rating_property = Some({
									struct DeserializeWith(Vec<AggregateRatingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "alumni-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Alumni => {
								if r#alumni_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("alumni"));
								}
								r#alumni_property = Some({
									struct DeserializeWith(Vec<AlumniProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "area-served-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AreaServed => {
								if r#area_served_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"areaServed",
									));
								}
								r#area_served_property = Some({
									struct DeserializeWith(Vec<AreaServedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "corrections-policy-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::CorrectionsPolicy => {
								if r#corrections_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"correctionsPolicy",
									));
								}
								r#corrections_policy_property = Some({
									struct DeserializeWith(Vec<CorrectionsPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "department-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Department => {
								if r#department_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"department",
									));
								}
								r#department_property = Some({
									struct DeserializeWith(Vec<DepartmentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "dissolution-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DissolutionDate => {
								if r#dissolution_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"dissolutionDate",
									));
								}
								r#dissolution_date_property = Some({
									struct DeserializeWith(Vec<DissolutionDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "diversity-policy-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::DiversityPolicy => {
								if r#diversity_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"diversityPolicy",
									));
								}
								r#diversity_policy_property = Some({
									struct DeserializeWith(Vec<DiversityPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "diversity-staffing-report-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::DiversityStaffingReport => {
								if r#diversity_staffing_report_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"diversityStaffingReport",
									));
								}
								r#diversity_staffing_report_property = Some({
									struct DeserializeWith(Vec<DiversityStaffingReportProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "employee-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Employee => {
								if r#employee_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"employee",
									));
								}
								r#employee_property = Some({
									struct DeserializeWith(Vec<EmployeeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "employees-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Employees => {
								if r#employees_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"employees",
									));
								}
								r#employees_property = Some({
									struct DeserializeWith(Vec<EmployeesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "ethics-policy-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::EthicsPolicy => {
								if r#ethics_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"ethicsPolicy",
									));
								}
								r#ethics_policy_property = Some({
									struct DeserializeWith(Vec<EthicsPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "event-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Event => {
								if r#event_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("event"));
								}
								r#event_property = Some({
									struct DeserializeWith(Vec<EventProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "events-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Events => {
								if r#events_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("events"));
								}
								r#events_property = Some({
									struct DeserializeWith(Vec<EventsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "founder-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Founder => {
								if r#founder_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"founder",
									));
								}
								r#founder_property = Some({
									struct DeserializeWith(Vec<FounderProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "founders-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Founders => {
								if r#founders_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"founders",
									));
								}
								r#founders_property = Some({
									struct DeserializeWith(Vec<FoundersProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "founding-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FoundingDate => {
								if r#founding_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"foundingDate",
									));
								}
								r#founding_date_property = Some({
									struct DeserializeWith(Vec<FoundingDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "founding-location-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FoundingLocation => {
								if r#founding_location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"foundingLocation",
									));
								}
								r#founding_location_property = Some({
									struct DeserializeWith(Vec<FoundingLocationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "has-merchant-return-policy-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasMerchantReturnPolicy => {
								if r#has_merchant_return_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasMerchantReturnPolicy",
									));
								}
								r#has_merchant_return_policy_property = Some({
									struct DeserializeWith(Vec<HasMerchantReturnPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "has-product-return-policy-property-schema",
									feature = "attic-schema-section"
								),
								doc
							))]
							Field::HasProductReturnPolicy => {
								if r#has_product_return_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasProductReturnPolicy",
									));
								}
								r#has_product_return_policy_property = Some({
									struct DeserializeWith(Vec<HasProductReturnPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "iso-6523-code-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Iso6523Code => {
								if r#iso_6523_code_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"iso6523Code",
									));
								}
								r#iso_6523_code_property = Some({
									struct DeserializeWith(Vec<Iso6523CodeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "keywords-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Keywords => {
								if r#keywords_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"keywords",
									));
								}
								r#keywords_property = Some({
									struct DeserializeWith(Vec<KeywordsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "legal-name-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::LegalName => {
								if r#legal_name_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legalName",
									));
								}
								r#legal_name_property = Some({
									struct DeserializeWith(Vec<LegalNameProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "lei-code-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::LeiCode => {
								if r#lei_code_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"leiCode",
									));
								}
								r#lei_code_property = Some({
									struct DeserializeWith(Vec<LeiCodeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "location-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Location => {
								if r#location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"location",
									));
								}
								r#location_property = Some({
									struct DeserializeWith(Vec<LocationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "logo-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Logo => {
								if r#logo_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("logo"));
								}
								r#logo_property = Some({
									struct DeserializeWith(Vec<LogoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "member-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Member => {
								if r#member_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("member"));
								}
								r#member_property = Some({
									struct DeserializeWith(Vec<MemberProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "members-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Members => {
								if r#members_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"members",
									));
								}
								r#members_property = Some({
									struct DeserializeWith(Vec<MembersProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "nonprofit-status-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::NonprofitStatus => {
								if r#nonprofit_status_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"nonprofitStatus",
									));
								}
								r#nonprofit_status_property = Some({
									struct DeserializeWith(Vec<NonprofitStatusProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "number-of-employees-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::NumberOfEmployees => {
								if r#number_of_employees_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfEmployees",
									));
								}
								r#number_of_employees_property = Some({
									struct DeserializeWith(Vec<NumberOfEmployeesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "ownership-funding-info-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::OwnershipFundingInfo => {
								if r#ownership_funding_info_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"ownershipFundingInfo",
									));
								}
								r#ownership_funding_info_property = Some({
									struct DeserializeWith(Vec<OwnershipFundingInfoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "parent-organization-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ParentOrganization => {
								if r#parent_organization_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"parentOrganization",
									));
								}
								r#parent_organization_property = Some({
									struct DeserializeWith(Vec<ParentOrganizationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "review-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Review => {
								if r#review_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("review"));
								}
								r#review_property = Some({
									struct DeserializeWith(Vec<ReviewProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "reviews-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Reviews => {
								if r#reviews_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"reviews",
									));
								}
								r#reviews_property = Some({
									struct DeserializeWith(Vec<ReviewsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "service-area-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ServiceArea => {
								if r#service_area_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"serviceArea",
									));
								}
								r#service_area_property = Some({
									struct DeserializeWith(Vec<ServiceAreaProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "slogan-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Slogan => {
								if r#slogan_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("slogan"));
								}
								r#slogan_property = Some({
									struct DeserializeWith(Vec<SloganProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "sub-organization-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::SubOrganization => {
								if r#sub_organization_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subOrganization",
									));
								}
								r#sub_organization_property = Some({
									struct DeserializeWith(Vec<SubOrganizationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "ticker-symbol-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::TickerSymbol => {
								if r#ticker_symbol_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"tickerSymbol",
									));
								}
								r#ticker_symbol_property = Some({
									struct DeserializeWith(Vec<TickerSymbolProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "unnamed-sources-policy-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::UnnamedSourcesPolicy => {
								if r#unnamed_sources_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"unnamedSourcesPolicy",
									));
								}
								r#unnamed_sources_policy_property = Some({
									struct DeserializeWith(Vec<UnnamedSourcesPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(Corporation {
						#[cfg(any(
							any(
								feature = "actionable-feedback-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#actionable_feedback_policy: r#actionable_feedback_policy_property
							.unwrap_or_default(),
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
								feature = "agent-interaction-statistic-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#agent_interaction_statistic: r#agent_interaction_statistic_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
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
								feature = "alumni-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#alumni: r#alumni_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "area-served-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#area_served: r#area_served_property.unwrap_or_default(),
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
								feature = "brand-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#brand: r#brand_property.unwrap_or_default(),
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
								feature = "corrections-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#corrections_policy: r#corrections_policy_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "department-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#department: r#department_property.unwrap_or_default(),
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
								feature = "dissolution-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#dissolution_date: r#dissolution_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "diversity-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#diversity_policy: r#diversity_policy_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "diversity-staffing-report-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#diversity_staffing_report: r#diversity_staffing_report_property
							.unwrap_or_default(),
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
								feature = "employee-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#employee: r#employee_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "employees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#employees: r#employees_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "ethics-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#ethics_policy: r#ethics_policy_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#event: r#event_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "events-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#events: r#events_property.unwrap_or_default(),
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
								feature = "founder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#founder: r#founder_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "founders-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#founders: r#founders_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "founding-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#founding_date: r#founding_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "founding-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#founding_location: r#founding_location_property.unwrap_or_default(),
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
								feature = "has-merchant-return-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_merchant_return_policy: r#has_merchant_return_policy_property
							.unwrap_or_default(),
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
								feature = "has-product-return-policy-property-schema",
								feature = "attic-schema-section"
							),
							doc
						))]
						r#has_product_return_policy: r#has_product_return_policy_property
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
								feature = "iso-6523-code-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#iso_6523_code: r#iso_6523_code_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#keywords: r#keywords_property.unwrap_or_default(),
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
								feature = "legal-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#legal_name: r#legal_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "lei-code-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#lei_code: r#lei_code_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#location: r#location_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "logo-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#logo: r#logo_property.unwrap_or_default(),
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
								feature = "member-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#member: r#member_property.unwrap_or_default(),
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
								feature = "members-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#members: r#members_property.unwrap_or_default(),
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
								feature = "nonprofit-status-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#nonprofit_status: r#nonprofit_status_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "number-of-employees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#number_of_employees: r#number_of_employees_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "ownership-funding-info-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#ownership_funding_info: r#ownership_funding_info_property
							.unwrap_or_default(),
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
								feature = "parent-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#parent_organization: r#parent_organization_property.unwrap_or_default(),
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
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#review: r#review_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "reviews-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#reviews: r#reviews_property.unwrap_or_default(),
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
								feature = "service-area-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#service_area: r#service_area_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "slogan-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#slogan: r#slogan_property.unwrap_or_default(),
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
								feature = "sub-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#sub_organization: r#sub_organization_property.unwrap_or_default(),
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
								feature = "ticker-symbol-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#ticker_symbol: r#ticker_symbol_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "unnamed-sources-policy-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#unnamed_sources_policy: r#unnamed_sources_policy_property
							.unwrap_or_default(),
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
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(
						feature = "actionable-feedback-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"actionableFeedbackPolicy",
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
						feature = "agent-interaction-statistic-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"agentInteractionStatistic",
				#[cfg(any(
					any(
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"aggregateRating",
				#[cfg(any(
					any(
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternateName",
				#[cfg(any(
					any(feature = "alumni-property-schema", feature = "general-schema-section"),
					doc
				))]
				"alumni",
				#[cfg(any(
					any(
						feature = "area-served-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"areaServed",
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
					any(feature = "brand-property-schema", feature = "general-schema-section"),
					doc
				))]
				"brand",
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
						feature = "corrections-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"correctionsPolicy",
				#[cfg(any(
					any(
						feature = "department-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"department",
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
						feature = "dissolution-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"dissolutionDate",
				#[cfg(any(
					any(
						feature = "diversity-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"diversityPolicy",
				#[cfg(any(
					any(
						feature = "diversity-staffing-report-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"diversityStaffingReport",
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
						feature = "employee-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"employee",
				#[cfg(any(
					any(
						feature = "employees-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"employees",
				#[cfg(any(
					any(
						feature = "ethics-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"ethicsPolicy",
				#[cfg(any(
					any(feature = "event-property-schema", feature = "general-schema-section"),
					doc
				))]
				"event",
				#[cfg(any(
					any(feature = "events-property-schema", feature = "general-schema-section"),
					doc
				))]
				"events",
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
						feature = "founder-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"founder",
				#[cfg(any(
					any(
						feature = "founders-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"founders",
				#[cfg(any(
					any(
						feature = "founding-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"foundingDate",
				#[cfg(any(
					any(
						feature = "founding-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"foundingLocation",
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
						feature = "has-merchant-return-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasMerchantReturnPolicy",
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
						feature = "has-product-return-policy-property-schema",
						feature = "attic-schema-section"
					),
					doc
				))]
				"hasProductReturnPolicy",
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
						feature = "iso-6523-code-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"iso6523Code",
				#[cfg(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"keywords",
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
						feature = "legal-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"legalName",
				#[cfg(any(
					any(
						feature = "lei-code-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"leiCode",
				#[cfg(any(
					any(
						feature = "location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"location",
				#[cfg(any(
					any(feature = "logo-property-schema", feature = "general-schema-section"),
					doc
				))]
				"logo",
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
					any(feature = "member-property-schema", feature = "general-schema-section"),
					doc
				))]
				"member",
				#[cfg(any(
					any(
						feature = "member-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"memberOf",
				#[cfg(any(
					any(
						feature = "members-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"members",
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
						feature = "nonprofit-status-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"nonprofitStatus",
				#[cfg(any(
					any(
						feature = "number-of-employees-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"numberOfEmployees",
				#[cfg(any(
					any(
						feature = "ownership-funding-info-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"ownershipFundingInfo",
				#[cfg(any(
					any(feature = "owns-property-schema", feature = "general-schema-section"),
					doc
				))]
				"owns",
				#[cfg(any(
					any(
						feature = "parent-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"parentOrganization",
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
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				"review",
				#[cfg(any(
					any(
						feature = "reviews-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"reviews",
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
						feature = "service-area-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"serviceArea",
				#[cfg(any(
					any(feature = "slogan-property-schema", feature = "general-schema-section"),
					doc
				))]
				"slogan",
				#[cfg(any(
					any(
						feature = "sponsor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sponsor",
				#[cfg(any(
					any(
						feature = "sub-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subOrganization",
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subjectOf",
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
					any(
						feature = "ticker-symbol-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"tickerSymbol",
				#[cfg(any(
					any(
						feature = "unnamed-sources-policy-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"unnamedSourcesPolicy",
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
			];
			deserializer.deserialize_struct("Corporation", FIELDS, ClassVisitor)
		}
	}
}
