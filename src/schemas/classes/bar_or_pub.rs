use super::*;
/// <https://schema.org/BarOrPub>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct BarOrPub {
	pub r#accepts_reservations: Vec<AcceptsReservationsProperty>,
	pub r#actionable_feedback_policy: Vec<ActionableFeedbackPolicyProperty>,
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#address: Vec<AddressProperty>,
	pub r#agent_interaction_statistic: Vec<AgentInteractionStatisticProperty>,
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#alumni: Vec<AlumniProperty>,
	pub r#amenity_feature: Vec<AmenityFeatureProperty>,
	pub r#area_served: Vec<AreaServedProperty>,
	pub r#award: Vec<AwardProperty>,
	pub r#awards: Vec<AwardsProperty>,
	pub r#branch_code: Vec<BranchCodeProperty>,
	pub r#branch_of: Vec<BranchOfProperty>,
	pub r#brand: Vec<BrandProperty>,
	pub r#contact_point: Vec<ContactPointProperty>,
	pub r#contact_points: Vec<ContactPointsProperty>,
	pub r#contained_in: Vec<ContainedInProperty>,
	pub r#contained_in_place: Vec<ContainedInPlaceProperty>,
	pub r#contains_place: Vec<ContainsPlaceProperty>,
	pub r#corrections_policy: Vec<CorrectionsPolicyProperty>,
	pub r#currencies_accepted: Vec<CurrenciesAcceptedProperty>,
	pub r#department: Vec<DepartmentProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#dissolution_date: Vec<DissolutionDateProperty>,
	pub r#diversity_policy: Vec<DiversityPolicyProperty>,
	pub r#diversity_staffing_report: Vec<DiversityStaffingReportProperty>,
	pub r#duns: Vec<DunsProperty>,
	pub r#email: Vec<EmailProperty>,
	pub r#employee: Vec<EmployeeProperty>,
	pub r#employees: Vec<EmployeesProperty>,
	pub r#ethics_policy: Vec<EthicsPolicyProperty>,
	pub r#event: Vec<EventProperty>,
	pub r#events: Vec<EventsProperty>,
	pub r#fax_number: Vec<FaxNumberProperty>,
	pub r#founder: Vec<FounderProperty>,
	pub r#founders: Vec<FoundersProperty>,
	pub r#founding_date: Vec<FoundingDateProperty>,
	pub r#founding_location: Vec<FoundingLocationProperty>,
	pub r#funder: Vec<FunderProperty>,
	pub r#funding: Vec<FundingProperty>,
	pub r#geo: Vec<GeoProperty>,
	pub r#geo_contains: Vec<GeoContainsProperty>,
	pub r#geo_covered_by: Vec<GeoCoveredByProperty>,
	pub r#geo_covers: Vec<GeoCoversProperty>,
	pub r#geo_crosses: Vec<GeoCrossesProperty>,
	pub r#geo_disjoint: Vec<GeoDisjointProperty>,
	pub r#geo_equals: Vec<GeoEqualsProperty>,
	pub r#geo_intersects: Vec<GeoIntersectsProperty>,
	pub r#geo_overlaps: Vec<GeoOverlapsProperty>,
	pub r#geo_touches: Vec<GeoTouchesProperty>,
	pub r#geo_within: Vec<GeoWithinProperty>,
	pub r#global_location_number: Vec<GlobalLocationNumberProperty>,
	pub r#has_credential: Vec<HasCredentialProperty>,
	pub r#has_drive_through_service: Vec<HasDriveThroughServiceProperty>,
	pub r#has_map: Vec<HasMapProperty>,
	pub r#has_menu: Vec<HasMenuProperty>,
	pub r#has_merchant_return_policy: Vec<HasMerchantReturnPolicyProperty>,
	pub r#has_offer_catalog: Vec<HasOfferCatalogProperty>,
	pub r#has_pos: Vec<HasPosProperty>,
	pub r#has_product_return_policy: Vec<HasProductReturnPolicyProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	pub r#isic_v_4: Vec<IsicV4Property>,
	pub r#iso_6523_code: Vec<Iso6523CodeProperty>,
	pub r#keywords: Vec<KeywordsProperty>,
	pub r#knows_about: Vec<KnowsAboutProperty>,
	pub r#knows_language: Vec<KnowsLanguageProperty>,
	pub r#latitude: Vec<LatitudeProperty>,
	pub r#legal_name: Vec<LegalNameProperty>,
	pub r#lei_code: Vec<LeiCodeProperty>,
	pub r#location: Vec<LocationProperty>,
	pub r#logo: Vec<LogoProperty>,
	pub r#longitude: Vec<LongitudeProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#makes_offer: Vec<MakesOfferProperty>,
	pub r#map: Vec<MapProperty>,
	pub r#maps: Vec<MapsProperty>,
	pub r#maximum_attendee_capacity: Vec<MaximumAttendeeCapacityProperty>,
	pub r#member: Vec<MemberProperty>,
	pub r#member_of: Vec<MemberOfProperty>,
	pub r#members: Vec<MembersProperty>,
	pub r#menu: Vec<MenuProperty>,
	pub r#naics: Vec<NaicsProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#nonprofit_status: Vec<NonprofitStatusProperty>,
	pub r#number_of_employees: Vec<NumberOfEmployeesProperty>,
	pub r#opening_hours: Vec<OpeningHoursProperty>,
	pub r#opening_hours_specification: Vec<OpeningHoursSpecificationProperty>,
	pub r#ownership_funding_info: Vec<OwnershipFundingInfoProperty>,
	pub r#owns: Vec<OwnsProperty>,
	pub r#parent_organization: Vec<ParentOrganizationProperty>,
	pub r#payment_accepted: Vec<PaymentAcceptedProperty>,
	pub r#photo: Vec<PhotoProperty>,
	pub r#photos: Vec<PhotosProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#price_range: Vec<PriceRangeProperty>,
	pub r#public_access: Vec<PublicAccessProperty>,
	pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
	pub r#review: Vec<ReviewProperty>,
	pub r#reviews: Vec<ReviewsProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#seeks: Vec<SeeksProperty>,
	pub r#serves_cuisine: Vec<ServesCuisineProperty>,
	pub r#service_area: Vec<ServiceAreaProperty>,
	pub r#slogan: Vec<SloganProperty>,
	pub r#smoking_allowed: Vec<SmokingAllowedProperty>,
	pub r#special_opening_hours_specification: Vec<SpecialOpeningHoursSpecificationProperty>,
	pub r#sponsor: Vec<SponsorProperty>,
	pub r#star_rating: Vec<StarRatingProperty>,
	pub r#sub_organization: Vec<SubOrganizationProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#tax_id: Vec<TaxIdProperty>,
	pub r#telephone: Vec<TelephoneProperty>,
	pub r#tour_booking_page: Vec<TourBookingPageProperty>,
	pub r#unnamed_sources_policy: Vec<UnnamedSourcesPolicyProperty>,
	pub r#url: Vec<UrlProperty>,
	pub r#vat_id: Vec<VatIdProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for BarOrPub {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#accepts_reservations) as usize,
				!Vec::is_empty(&self.r#actionable_feedback_policy) as usize,
				!Vec::is_empty(&self.r#additional_property) as usize,
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#address) as usize,
				!Vec::is_empty(&self.r#agent_interaction_statistic) as usize,
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#alumni) as usize,
				!Vec::is_empty(&self.r#amenity_feature) as usize,
				!Vec::is_empty(&self.r#area_served) as usize,
				!Vec::is_empty(&self.r#award) as usize,
				!Vec::is_empty(&self.r#awards) as usize,
				!Vec::is_empty(&self.r#branch_code) as usize,
				!Vec::is_empty(&self.r#branch_of) as usize,
				!Vec::is_empty(&self.r#brand) as usize,
				!Vec::is_empty(&self.r#contact_point) as usize,
				!Vec::is_empty(&self.r#contact_points) as usize,
				!Vec::is_empty(&self.r#contained_in) as usize,
				!Vec::is_empty(&self.r#contained_in_place) as usize,
				!Vec::is_empty(&self.r#contains_place) as usize,
				!Vec::is_empty(&self.r#corrections_policy) as usize,
				!Vec::is_empty(&self.r#currencies_accepted) as usize,
				!Vec::is_empty(&self.r#department) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#dissolution_date) as usize,
				!Vec::is_empty(&self.r#diversity_policy) as usize,
				!Vec::is_empty(&self.r#diversity_staffing_report) as usize,
				!Vec::is_empty(&self.r#duns) as usize,
				!Vec::is_empty(&self.r#email) as usize,
				!Vec::is_empty(&self.r#employee) as usize,
				!Vec::is_empty(&self.r#employees) as usize,
				!Vec::is_empty(&self.r#ethics_policy) as usize,
				!Vec::is_empty(&self.r#event) as usize,
				!Vec::is_empty(&self.r#events) as usize,
				!Vec::is_empty(&self.r#fax_number) as usize,
				!Vec::is_empty(&self.r#founder) as usize,
				!Vec::is_empty(&self.r#founders) as usize,
				!Vec::is_empty(&self.r#founding_date) as usize,
				!Vec::is_empty(&self.r#founding_location) as usize,
				!Vec::is_empty(&self.r#funder) as usize,
				!Vec::is_empty(&self.r#funding) as usize,
				!Vec::is_empty(&self.r#geo) as usize,
				!Vec::is_empty(&self.r#geo_contains) as usize,
				!Vec::is_empty(&self.r#geo_covered_by) as usize,
				!Vec::is_empty(&self.r#geo_covers) as usize,
				!Vec::is_empty(&self.r#geo_crosses) as usize,
				!Vec::is_empty(&self.r#geo_disjoint) as usize,
				!Vec::is_empty(&self.r#geo_equals) as usize,
				!Vec::is_empty(&self.r#geo_intersects) as usize,
				!Vec::is_empty(&self.r#geo_overlaps) as usize,
				!Vec::is_empty(&self.r#geo_touches) as usize,
				!Vec::is_empty(&self.r#geo_within) as usize,
				!Vec::is_empty(&self.r#global_location_number) as usize,
				!Vec::is_empty(&self.r#has_credential) as usize,
				!Vec::is_empty(&self.r#has_drive_through_service) as usize,
				!Vec::is_empty(&self.r#has_map) as usize,
				!Vec::is_empty(&self.r#has_menu) as usize,
				!Vec::is_empty(&self.r#has_merchant_return_policy) as usize,
				!Vec::is_empty(&self.r#has_offer_catalog) as usize,
				!Vec::is_empty(&self.r#has_pos) as usize,
				!Vec::is_empty(&self.r#has_product_return_policy) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#interaction_statistic) as usize,
				!Vec::is_empty(&self.r#is_accessible_for_free) as usize,
				!Vec::is_empty(&self.r#isic_v_4) as usize,
				!Vec::is_empty(&self.r#iso_6523_code) as usize,
				!Vec::is_empty(&self.r#keywords) as usize,
				!Vec::is_empty(&self.r#knows_about) as usize,
				!Vec::is_empty(&self.r#knows_language) as usize,
				!Vec::is_empty(&self.r#latitude) as usize,
				!Vec::is_empty(&self.r#legal_name) as usize,
				!Vec::is_empty(&self.r#lei_code) as usize,
				!Vec::is_empty(&self.r#location) as usize,
				!Vec::is_empty(&self.r#logo) as usize,
				!Vec::is_empty(&self.r#longitude) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#makes_offer) as usize,
				!Vec::is_empty(&self.r#map) as usize,
				!Vec::is_empty(&self.r#maps) as usize,
				!Vec::is_empty(&self.r#maximum_attendee_capacity) as usize,
				!Vec::is_empty(&self.r#member) as usize,
				!Vec::is_empty(&self.r#member_of) as usize,
				!Vec::is_empty(&self.r#members) as usize,
				!Vec::is_empty(&self.r#menu) as usize,
				!Vec::is_empty(&self.r#naics) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#nonprofit_status) as usize,
				!Vec::is_empty(&self.r#number_of_employees) as usize,
				!Vec::is_empty(&self.r#opening_hours) as usize,
				!Vec::is_empty(&self.r#opening_hours_specification) as usize,
				!Vec::is_empty(&self.r#ownership_funding_info) as usize,
				!Vec::is_empty(&self.r#owns) as usize,
				!Vec::is_empty(&self.r#parent_organization) as usize,
				!Vec::is_empty(&self.r#payment_accepted) as usize,
				!Vec::is_empty(&self.r#photo) as usize,
				!Vec::is_empty(&self.r#photos) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#price_range) as usize,
				!Vec::is_empty(&self.r#public_access) as usize,
				!Vec::is_empty(&self.r#publishing_principles) as usize,
				!Vec::is_empty(&self.r#review) as usize,
				!Vec::is_empty(&self.r#reviews) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#seeks) as usize,
				!Vec::is_empty(&self.r#serves_cuisine) as usize,
				!Vec::is_empty(&self.r#service_area) as usize,
				!Vec::is_empty(&self.r#slogan) as usize,
				!Vec::is_empty(&self.r#smoking_allowed) as usize,
				!Vec::is_empty(&self.r#special_opening_hours_specification) as usize,
				!Vec::is_empty(&self.r#sponsor) as usize,
				!Vec::is_empty(&self.r#star_rating) as usize,
				!Vec::is_empty(&self.r#sub_organization) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#tax_id) as usize,
				!Vec::is_empty(&self.r#telephone) as usize,
				!Vec::is_empty(&self.r#tour_booking_page) as usize,
				!Vec::is_empty(&self.r#unnamed_sources_policy) as usize,
				!Vec::is_empty(&self.r#url) as usize,
				!Vec::is_empty(&self.r#vat_id) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "BarOrPub", len)?;
			if !Vec::is_empty(&self.r#accepts_reservations) {
				serialize_struct.serialize_field("acceptsReservations", {
					struct SerializeWith<'a>(&'a Vec<AcceptsReservationsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#accepts_reservations)
				})?;
			} else {
				serialize_struct.skip_field("acceptsReservations")?;
			}
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
			if !Vec::is_empty(&self.r#branch_code) {
				serialize_struct.serialize_field("branchCode", {
					struct SerializeWith<'a>(&'a Vec<BranchCodeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#branch_code)
				})?;
			} else {
				serialize_struct.skip_field("branchCode")?;
			}
			if !Vec::is_empty(&self.r#branch_of) {
				serialize_struct.serialize_field("branchOf", {
					struct SerializeWith<'a>(&'a Vec<BranchOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#branch_of)
				})?;
			} else {
				serialize_struct.skip_field("branchOf")?;
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
			if !Vec::is_empty(&self.r#contained_in) {
				serialize_struct.serialize_field("containedIn", {
					struct SerializeWith<'a>(&'a Vec<ContainedInProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#contained_in)
				})?;
			} else {
				serialize_struct.skip_field("containedIn")?;
			}
			if !Vec::is_empty(&self.r#contained_in_place) {
				serialize_struct.serialize_field("containedInPlace", {
					struct SerializeWith<'a>(&'a Vec<ContainedInPlaceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#contained_in_place)
				})?;
			} else {
				serialize_struct.skip_field("containedInPlace")?;
			}
			if !Vec::is_empty(&self.r#contains_place) {
				serialize_struct.serialize_field("containsPlace", {
					struct SerializeWith<'a>(&'a Vec<ContainsPlaceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#contains_place)
				})?;
			} else {
				serialize_struct.skip_field("containsPlace")?;
			}
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
			if !Vec::is_empty(&self.r#currencies_accepted) {
				serialize_struct.serialize_field("currenciesAccepted", {
					struct SerializeWith<'a>(&'a Vec<CurrenciesAcceptedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#currencies_accepted)
				})?;
			} else {
				serialize_struct.skip_field("currenciesAccepted")?;
			}
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
			if !Vec::is_empty(&self.r#geo) {
				serialize_struct.serialize_field("geo", {
					struct SerializeWith<'a>(&'a Vec<GeoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo)
				})?;
			} else {
				serialize_struct.skip_field("geo")?;
			}
			if !Vec::is_empty(&self.r#geo_contains) {
				serialize_struct.serialize_field("geoContains", {
					struct SerializeWith<'a>(&'a Vec<GeoContainsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_contains)
				})?;
			} else {
				serialize_struct.skip_field("geoContains")?;
			}
			if !Vec::is_empty(&self.r#geo_covered_by) {
				serialize_struct.serialize_field("geoCoveredBy", {
					struct SerializeWith<'a>(&'a Vec<GeoCoveredByProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_covered_by)
				})?;
			} else {
				serialize_struct.skip_field("geoCoveredBy")?;
			}
			if !Vec::is_empty(&self.r#geo_covers) {
				serialize_struct.serialize_field("geoCovers", {
					struct SerializeWith<'a>(&'a Vec<GeoCoversProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_covers)
				})?;
			} else {
				serialize_struct.skip_field("geoCovers")?;
			}
			if !Vec::is_empty(&self.r#geo_crosses) {
				serialize_struct.serialize_field("geoCrosses", {
					struct SerializeWith<'a>(&'a Vec<GeoCrossesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_crosses)
				})?;
			} else {
				serialize_struct.skip_field("geoCrosses")?;
			}
			if !Vec::is_empty(&self.r#geo_disjoint) {
				serialize_struct.serialize_field("geoDisjoint", {
					struct SerializeWith<'a>(&'a Vec<GeoDisjointProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_disjoint)
				})?;
			} else {
				serialize_struct.skip_field("geoDisjoint")?;
			}
			if !Vec::is_empty(&self.r#geo_equals) {
				serialize_struct.serialize_field("geoEquals", {
					struct SerializeWith<'a>(&'a Vec<GeoEqualsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_equals)
				})?;
			} else {
				serialize_struct.skip_field("geoEquals")?;
			}
			if !Vec::is_empty(&self.r#geo_intersects) {
				serialize_struct.serialize_field("geoIntersects", {
					struct SerializeWith<'a>(&'a Vec<GeoIntersectsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_intersects)
				})?;
			} else {
				serialize_struct.skip_field("geoIntersects")?;
			}
			if !Vec::is_empty(&self.r#geo_overlaps) {
				serialize_struct.serialize_field("geoOverlaps", {
					struct SerializeWith<'a>(&'a Vec<GeoOverlapsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_overlaps)
				})?;
			} else {
				serialize_struct.skip_field("geoOverlaps")?;
			}
			if !Vec::is_empty(&self.r#geo_touches) {
				serialize_struct.serialize_field("geoTouches", {
					struct SerializeWith<'a>(&'a Vec<GeoTouchesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_touches)
				})?;
			} else {
				serialize_struct.skip_field("geoTouches")?;
			}
			if !Vec::is_empty(&self.r#geo_within) {
				serialize_struct.serialize_field("geoWithin", {
					struct SerializeWith<'a>(&'a Vec<GeoWithinProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#geo_within)
				})?;
			} else {
				serialize_struct.skip_field("geoWithin")?;
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
			if !Vec::is_empty(&self.r#has_drive_through_service) {
				serialize_struct.serialize_field("hasDriveThroughService", {
					struct SerializeWith<'a>(&'a Vec<HasDriveThroughServiceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_drive_through_service)
				})?;
			} else {
				serialize_struct.skip_field("hasDriveThroughService")?;
			}
			if !Vec::is_empty(&self.r#has_map) {
				serialize_struct.serialize_field("hasMap", {
					struct SerializeWith<'a>(&'a Vec<HasMapProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_map)
				})?;
			} else {
				serialize_struct.skip_field("hasMap")?;
			}
			if !Vec::is_empty(&self.r#has_menu) {
				serialize_struct.serialize_field("hasMenu", {
					struct SerializeWith<'a>(&'a Vec<HasMenuProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_menu)
				})?;
			} else {
				serialize_struct.skip_field("hasMenu")?;
			}
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
			if !Vec::is_empty(&self.r#is_accessible_for_free) {
				serialize_struct.serialize_field("isAccessibleForFree", {
					struct SerializeWith<'a>(&'a Vec<IsAccessibleForFreeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_accessible_for_free)
				})?;
			} else {
				serialize_struct.skip_field("isAccessibleForFree")?;
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
			if !Vec::is_empty(&self.r#latitude) {
				serialize_struct.serialize_field("latitude", {
					struct SerializeWith<'a>(&'a Vec<LatitudeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#latitude)
				})?;
			} else {
				serialize_struct.skip_field("latitude")?;
			}
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
			if !Vec::is_empty(&self.r#longitude) {
				serialize_struct.serialize_field("longitude", {
					struct SerializeWith<'a>(&'a Vec<LongitudeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#longitude)
				})?;
			} else {
				serialize_struct.skip_field("longitude")?;
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
			if !Vec::is_empty(&self.r#map) {
				serialize_struct.serialize_field("map", {
					struct SerializeWith<'a>(&'a Vec<MapProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#map)
				})?;
			} else {
				serialize_struct.skip_field("map")?;
			}
			if !Vec::is_empty(&self.r#maps) {
				serialize_struct.serialize_field("maps", {
					struct SerializeWith<'a>(&'a Vec<MapsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#maps)
				})?;
			} else {
				serialize_struct.skip_field("maps")?;
			}
			if !Vec::is_empty(&self.r#maximum_attendee_capacity) {
				serialize_struct.serialize_field("maximumAttendeeCapacity", {
					struct SerializeWith<'a>(&'a Vec<MaximumAttendeeCapacityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#maximum_attendee_capacity)
				})?;
			} else {
				serialize_struct.skip_field("maximumAttendeeCapacity")?;
			}
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
			if !Vec::is_empty(&self.r#menu) {
				serialize_struct.serialize_field("menu", {
					struct SerializeWith<'a>(&'a Vec<MenuProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#menu)
				})?;
			} else {
				serialize_struct.skip_field("menu")?;
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
			if !Vec::is_empty(&self.r#opening_hours) {
				serialize_struct.serialize_field("openingHours", {
					struct SerializeWith<'a>(&'a Vec<OpeningHoursProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#opening_hours)
				})?;
			} else {
				serialize_struct.skip_field("openingHours")?;
			}
			if !Vec::is_empty(&self.r#opening_hours_specification) {
				serialize_struct.serialize_field("openingHoursSpecification", {
					struct SerializeWith<'a>(&'a Vec<OpeningHoursSpecificationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#opening_hours_specification)
				})?;
			} else {
				serialize_struct.skip_field("openingHoursSpecification")?;
			}
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
			if !Vec::is_empty(&self.r#payment_accepted) {
				serialize_struct.serialize_field("paymentAccepted", {
					struct SerializeWith<'a>(&'a Vec<PaymentAcceptedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#payment_accepted)
				})?;
			} else {
				serialize_struct.skip_field("paymentAccepted")?;
			}
			if !Vec::is_empty(&self.r#photo) {
				serialize_struct.serialize_field("photo", {
					struct SerializeWith<'a>(&'a Vec<PhotoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#photo)
				})?;
			} else {
				serialize_struct.skip_field("photo")?;
			}
			if !Vec::is_empty(&self.r#photos) {
				serialize_struct.serialize_field("photos", {
					struct SerializeWith<'a>(&'a Vec<PhotosProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#photos)
				})?;
			} else {
				serialize_struct.skip_field("photos")?;
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
			if !Vec::is_empty(&self.r#price_range) {
				serialize_struct.serialize_field("priceRange", {
					struct SerializeWith<'a>(&'a Vec<PriceRangeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#price_range)
				})?;
			} else {
				serialize_struct.skip_field("priceRange")?;
			}
			if !Vec::is_empty(&self.r#public_access) {
				serialize_struct.serialize_field("publicAccess", {
					struct SerializeWith<'a>(&'a Vec<PublicAccessProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#public_access)
				})?;
			} else {
				serialize_struct.skip_field("publicAccess")?;
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
			if !Vec::is_empty(&self.r#serves_cuisine) {
				serialize_struct.serialize_field("servesCuisine", {
					struct SerializeWith<'a>(&'a Vec<ServesCuisineProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#serves_cuisine)
				})?;
			} else {
				serialize_struct.skip_field("servesCuisine")?;
			}
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
			if !Vec::is_empty(&self.r#smoking_allowed) {
				serialize_struct.serialize_field("smokingAllowed", {
					struct SerializeWith<'a>(&'a Vec<SmokingAllowedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#smoking_allowed)
				})?;
			} else {
				serialize_struct.skip_field("smokingAllowed")?;
			}
			if !Vec::is_empty(&self.r#special_opening_hours_specification) {
				serialize_struct.serialize_field("specialOpeningHoursSpecification", {
					struct SerializeWith<'a>(&'a Vec<SpecialOpeningHoursSpecificationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#special_opening_hours_specification)
				})?;
			} else {
				serialize_struct.skip_field("specialOpeningHoursSpecification")?;
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
			if !Vec::is_empty(&self.r#star_rating) {
				serialize_struct.serialize_field("starRating", {
					struct SerializeWith<'a>(&'a Vec<StarRatingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#star_rating)
				})?;
			} else {
				serialize_struct.skip_field("starRating")?;
			}
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
			if !Vec::is_empty(&self.r#tour_booking_page) {
				serialize_struct.serialize_field("tourBookingPage", {
					struct SerializeWith<'a>(&'a Vec<TourBookingPageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#tour_booking_page)
				})?;
			} else {
				serialize_struct.skip_field("tourBookingPage")?;
			}
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
	impl<'de> Deserialize<'de> for BarOrPub {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AcceptsReservations,
				ActionableFeedbackPolicy,
				AdditionalProperty,
				AdditionalType,
				Address,
				AgentInteractionStatistic,
				AggregateRating,
				AlternateName,
				Alumni,
				AmenityFeature,
				AreaServed,
				Award,
				Awards,
				BranchCode,
				BranchOf,
				Brand,
				ContactPoint,
				ContactPoints,
				ContainedIn,
				ContainedInPlace,
				ContainsPlace,
				CorrectionsPolicy,
				CurrenciesAccepted,
				Department,
				Description,
				DisambiguatingDescription,
				DissolutionDate,
				DiversityPolicy,
				DiversityStaffingReport,
				Duns,
				Email,
				Employee,
				Employees,
				EthicsPolicy,
				Event,
				Events,
				FaxNumber,
				Founder,
				Founders,
				FoundingDate,
				FoundingLocation,
				Funder,
				Funding,
				Geo,
				GeoContains,
				GeoCoveredBy,
				GeoCovers,
				GeoCrosses,
				GeoDisjoint,
				GeoEquals,
				GeoIntersects,
				GeoOverlaps,
				GeoTouches,
				GeoWithin,
				GlobalLocationNumber,
				HasCredential,
				HasDriveThroughService,
				HasMap,
				HasMenu,
				HasMerchantReturnPolicy,
				HasOfferCatalog,
				HasPos,
				HasProductReturnPolicy,
				Identifier,
				Image,
				InteractionStatistic,
				IsAccessibleForFree,
				IsicV4,
				Iso6523Code,
				Keywords,
				KnowsAbout,
				KnowsLanguage,
				Latitude,
				LegalName,
				LeiCode,
				Location,
				Logo,
				Longitude,
				MainEntityOfPage,
				MakesOffer,
				Map,
				Maps,
				MaximumAttendeeCapacity,
				Member,
				MemberOf,
				Members,
				Menu,
				Naics,
				Name,
				NonprofitStatus,
				NumberOfEmployees,
				OpeningHours,
				OpeningHoursSpecification,
				OwnershipFundingInfo,
				Owns,
				ParentOrganization,
				PaymentAccepted,
				Photo,
				Photos,
				PotentialAction,
				PriceRange,
				PublicAccess,
				PublishingPrinciples,
				Review,
				Reviews,
				SameAs,
				Seeks,
				ServesCuisine,
				ServiceArea,
				Slogan,
				SmokingAllowed,
				SpecialOpeningHoursSpecification,
				Sponsor,
				StarRating,
				SubOrganization,
				SubjectOf,
				TaxId,
				Telephone,
				TourBookingPage,
				UnnamedSourcesPolicy,
				Url,
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
						"acceptsReservations" => Ok(Field::AcceptsReservations),
						"actionableFeedbackPolicy" => Ok(Field::ActionableFeedbackPolicy),
						"additionalProperty" => Ok(Field::AdditionalProperty),
						"additionalType" => Ok(Field::AdditionalType),
						"address" => Ok(Field::Address),
						"agentInteractionStatistic" => Ok(Field::AgentInteractionStatistic),
						"aggregateRating" => Ok(Field::AggregateRating),
						"alternateName" => Ok(Field::AlternateName),
						"alumni" => Ok(Field::Alumni),
						"amenityFeature" => Ok(Field::AmenityFeature),
						"areaServed" => Ok(Field::AreaServed),
						"award" => Ok(Field::Award),
						"awards" => Ok(Field::Awards),
						"branchCode" => Ok(Field::BranchCode),
						"branchOf" => Ok(Field::BranchOf),
						"brand" => Ok(Field::Brand),
						"contactPoint" => Ok(Field::ContactPoint),
						"contactPoints" => Ok(Field::ContactPoints),
						"containedIn" => Ok(Field::ContainedIn),
						"containedInPlace" => Ok(Field::ContainedInPlace),
						"containsPlace" => Ok(Field::ContainsPlace),
						"correctionsPolicy" => Ok(Field::CorrectionsPolicy),
						"currenciesAccepted" => Ok(Field::CurrenciesAccepted),
						"department" => Ok(Field::Department),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"dissolutionDate" => Ok(Field::DissolutionDate),
						"diversityPolicy" => Ok(Field::DiversityPolicy),
						"diversityStaffingReport" => Ok(Field::DiversityStaffingReport),
						"duns" => Ok(Field::Duns),
						"email" => Ok(Field::Email),
						"employee" => Ok(Field::Employee),
						"employees" => Ok(Field::Employees),
						"ethicsPolicy" => Ok(Field::EthicsPolicy),
						"event" => Ok(Field::Event),
						"events" => Ok(Field::Events),
						"faxNumber" => Ok(Field::FaxNumber),
						"founder" => Ok(Field::Founder),
						"founders" => Ok(Field::Founders),
						"foundingDate" => Ok(Field::FoundingDate),
						"foundingLocation" => Ok(Field::FoundingLocation),
						"funder" => Ok(Field::Funder),
						"funding" => Ok(Field::Funding),
						"geo" => Ok(Field::Geo),
						"geoContains" => Ok(Field::GeoContains),
						"geoCoveredBy" => Ok(Field::GeoCoveredBy),
						"geoCovers" => Ok(Field::GeoCovers),
						"geoCrosses" => Ok(Field::GeoCrosses),
						"geoDisjoint" => Ok(Field::GeoDisjoint),
						"geoEquals" => Ok(Field::GeoEquals),
						"geoIntersects" => Ok(Field::GeoIntersects),
						"geoOverlaps" => Ok(Field::GeoOverlaps),
						"geoTouches" => Ok(Field::GeoTouches),
						"geoWithin" => Ok(Field::GeoWithin),
						"globalLocationNumber" => Ok(Field::GlobalLocationNumber),
						"hasCredential" => Ok(Field::HasCredential),
						"hasDriveThroughService" => Ok(Field::HasDriveThroughService),
						"hasMap" => Ok(Field::HasMap),
						"hasMenu" => Ok(Field::HasMenu),
						"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						"hasPOS" => Ok(Field::HasPos),
						"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"interactionStatistic" => Ok(Field::InteractionStatistic),
						"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						"isicV4" => Ok(Field::IsicV4),
						"iso6523Code" => Ok(Field::Iso6523Code),
						"keywords" => Ok(Field::Keywords),
						"knowsAbout" => Ok(Field::KnowsAbout),
						"knowsLanguage" => Ok(Field::KnowsLanguage),
						"latitude" => Ok(Field::Latitude),
						"legalName" => Ok(Field::LegalName),
						"leiCode" => Ok(Field::LeiCode),
						"location" => Ok(Field::Location),
						"logo" => Ok(Field::Logo),
						"longitude" => Ok(Field::Longitude),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"makesOffer" => Ok(Field::MakesOffer),
						"map" => Ok(Field::Map),
						"maps" => Ok(Field::Maps),
						"maximumAttendeeCapacity" => Ok(Field::MaximumAttendeeCapacity),
						"member" => Ok(Field::Member),
						"memberOf" => Ok(Field::MemberOf),
						"members" => Ok(Field::Members),
						"menu" => Ok(Field::Menu),
						"naics" => Ok(Field::Naics),
						"name" => Ok(Field::Name),
						"nonprofitStatus" => Ok(Field::NonprofitStatus),
						"numberOfEmployees" => Ok(Field::NumberOfEmployees),
						"openingHours" => Ok(Field::OpeningHours),
						"openingHoursSpecification" => Ok(Field::OpeningHoursSpecification),
						"ownershipFundingInfo" => Ok(Field::OwnershipFundingInfo),
						"owns" => Ok(Field::Owns),
						"parentOrganization" => Ok(Field::ParentOrganization),
						"paymentAccepted" => Ok(Field::PaymentAccepted),
						"photo" => Ok(Field::Photo),
						"photos" => Ok(Field::Photos),
						"potentialAction" => Ok(Field::PotentialAction),
						"priceRange" => Ok(Field::PriceRange),
						"publicAccess" => Ok(Field::PublicAccess),
						"publishingPrinciples" => Ok(Field::PublishingPrinciples),
						"review" => Ok(Field::Review),
						"reviews" => Ok(Field::Reviews),
						"sameAs" => Ok(Field::SameAs),
						"seeks" => Ok(Field::Seeks),
						"servesCuisine" => Ok(Field::ServesCuisine),
						"serviceArea" => Ok(Field::ServiceArea),
						"slogan" => Ok(Field::Slogan),
						"smokingAllowed" => Ok(Field::SmokingAllowed),
						"specialOpeningHoursSpecification" => {
							Ok(Field::SpecialOpeningHoursSpecification)
						}
						"sponsor" => Ok(Field::Sponsor),
						"starRating" => Ok(Field::StarRating),
						"subOrganization" => Ok(Field::SubOrganization),
						"subjectOf" => Ok(Field::SubjectOf),
						"taxID" => Ok(Field::TaxId),
						"telephone" => Ok(Field::Telephone),
						"tourBookingPage" => Ok(Field::TourBookingPage),
						"unnamedSourcesPolicy" => Ok(Field::UnnamedSourcesPolicy),
						"url" => Ok(Field::Url),
						"vatID" => Ok(Field::VatId),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"acceptsReservations" => Ok(Field::AcceptsReservations),
						b"actionableFeedbackPolicy" => Ok(Field::ActionableFeedbackPolicy),
						b"additionalProperty" => Ok(Field::AdditionalProperty),
						b"additionalType" => Ok(Field::AdditionalType),
						b"address" => Ok(Field::Address),
						b"agentInteractionStatistic" => Ok(Field::AgentInteractionStatistic),
						b"aggregateRating" => Ok(Field::AggregateRating),
						b"alternateName" => Ok(Field::AlternateName),
						b"alumni" => Ok(Field::Alumni),
						b"amenityFeature" => Ok(Field::AmenityFeature),
						b"areaServed" => Ok(Field::AreaServed),
						b"award" => Ok(Field::Award),
						b"awards" => Ok(Field::Awards),
						b"branchCode" => Ok(Field::BranchCode),
						b"branchOf" => Ok(Field::BranchOf),
						b"brand" => Ok(Field::Brand),
						b"contactPoint" => Ok(Field::ContactPoint),
						b"contactPoints" => Ok(Field::ContactPoints),
						b"containedIn" => Ok(Field::ContainedIn),
						b"containedInPlace" => Ok(Field::ContainedInPlace),
						b"containsPlace" => Ok(Field::ContainsPlace),
						b"correctionsPolicy" => Ok(Field::CorrectionsPolicy),
						b"currenciesAccepted" => Ok(Field::CurrenciesAccepted),
						b"department" => Ok(Field::Department),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"dissolutionDate" => Ok(Field::DissolutionDate),
						b"diversityPolicy" => Ok(Field::DiversityPolicy),
						b"diversityStaffingReport" => Ok(Field::DiversityStaffingReport),
						b"duns" => Ok(Field::Duns),
						b"email" => Ok(Field::Email),
						b"employee" => Ok(Field::Employee),
						b"employees" => Ok(Field::Employees),
						b"ethicsPolicy" => Ok(Field::EthicsPolicy),
						b"event" => Ok(Field::Event),
						b"events" => Ok(Field::Events),
						b"faxNumber" => Ok(Field::FaxNumber),
						b"founder" => Ok(Field::Founder),
						b"founders" => Ok(Field::Founders),
						b"foundingDate" => Ok(Field::FoundingDate),
						b"foundingLocation" => Ok(Field::FoundingLocation),
						b"funder" => Ok(Field::Funder),
						b"funding" => Ok(Field::Funding),
						b"geo" => Ok(Field::Geo),
						b"geoContains" => Ok(Field::GeoContains),
						b"geoCoveredBy" => Ok(Field::GeoCoveredBy),
						b"geoCovers" => Ok(Field::GeoCovers),
						b"geoCrosses" => Ok(Field::GeoCrosses),
						b"geoDisjoint" => Ok(Field::GeoDisjoint),
						b"geoEquals" => Ok(Field::GeoEquals),
						b"geoIntersects" => Ok(Field::GeoIntersects),
						b"geoOverlaps" => Ok(Field::GeoOverlaps),
						b"geoTouches" => Ok(Field::GeoTouches),
						b"geoWithin" => Ok(Field::GeoWithin),
						b"globalLocationNumber" => Ok(Field::GlobalLocationNumber),
						b"hasCredential" => Ok(Field::HasCredential),
						b"hasDriveThroughService" => Ok(Field::HasDriveThroughService),
						b"hasMap" => Ok(Field::HasMap),
						b"hasMenu" => Ok(Field::HasMenu),
						b"hasMerchantReturnPolicy" => Ok(Field::HasMerchantReturnPolicy),
						b"hasOfferCatalog" => Ok(Field::HasOfferCatalog),
						b"hasPOS" => Ok(Field::HasPos),
						b"hasProductReturnPolicy" => Ok(Field::HasProductReturnPolicy),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"interactionStatistic" => Ok(Field::InteractionStatistic),
						b"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						b"isicV4" => Ok(Field::IsicV4),
						b"iso6523Code" => Ok(Field::Iso6523Code),
						b"keywords" => Ok(Field::Keywords),
						b"knowsAbout" => Ok(Field::KnowsAbout),
						b"knowsLanguage" => Ok(Field::KnowsLanguage),
						b"latitude" => Ok(Field::Latitude),
						b"legalName" => Ok(Field::LegalName),
						b"leiCode" => Ok(Field::LeiCode),
						b"location" => Ok(Field::Location),
						b"logo" => Ok(Field::Logo),
						b"longitude" => Ok(Field::Longitude),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"makesOffer" => Ok(Field::MakesOffer),
						b"map" => Ok(Field::Map),
						b"maps" => Ok(Field::Maps),
						b"maximumAttendeeCapacity" => Ok(Field::MaximumAttendeeCapacity),
						b"member" => Ok(Field::Member),
						b"memberOf" => Ok(Field::MemberOf),
						b"members" => Ok(Field::Members),
						b"menu" => Ok(Field::Menu),
						b"naics" => Ok(Field::Naics),
						b"name" => Ok(Field::Name),
						b"nonprofitStatus" => Ok(Field::NonprofitStatus),
						b"numberOfEmployees" => Ok(Field::NumberOfEmployees),
						b"openingHours" => Ok(Field::OpeningHours),
						b"openingHoursSpecification" => Ok(Field::OpeningHoursSpecification),
						b"ownershipFundingInfo" => Ok(Field::OwnershipFundingInfo),
						b"owns" => Ok(Field::Owns),
						b"parentOrganization" => Ok(Field::ParentOrganization),
						b"paymentAccepted" => Ok(Field::PaymentAccepted),
						b"photo" => Ok(Field::Photo),
						b"photos" => Ok(Field::Photos),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"priceRange" => Ok(Field::PriceRange),
						b"publicAccess" => Ok(Field::PublicAccess),
						b"publishingPrinciples" => Ok(Field::PublishingPrinciples),
						b"review" => Ok(Field::Review),
						b"reviews" => Ok(Field::Reviews),
						b"sameAs" => Ok(Field::SameAs),
						b"seeks" => Ok(Field::Seeks),
						b"servesCuisine" => Ok(Field::ServesCuisine),
						b"serviceArea" => Ok(Field::ServiceArea),
						b"slogan" => Ok(Field::Slogan),
						b"smokingAllowed" => Ok(Field::SmokingAllowed),
						b"specialOpeningHoursSpecification" => {
							Ok(Field::SpecialOpeningHoursSpecification)
						}
						b"sponsor" => Ok(Field::Sponsor),
						b"starRating" => Ok(Field::StarRating),
						b"subOrganization" => Ok(Field::SubOrganization),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"taxID" => Ok(Field::TaxId),
						b"telephone" => Ok(Field::Telephone),
						b"tourBookingPage" => Ok(Field::TourBookingPage),
						b"unnamedSourcesPolicy" => Ok(Field::UnnamedSourcesPolicy),
						b"url" => Ok(Field::Url),
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
				type Value = BarOrPub;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema BarOrPub")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#accepts_reservations_property = None;
					let mut r#actionable_feedback_policy_property = None;
					let mut r#additional_property_property = None;
					let mut r#additional_type_property = None;
					let mut r#address_property = None;
					let mut r#agent_interaction_statistic_property = None;
					let mut r#aggregate_rating_property = None;
					let mut r#alternate_name_property = None;
					let mut r#alumni_property = None;
					let mut r#amenity_feature_property = None;
					let mut r#area_served_property = None;
					let mut r#award_property = None;
					let mut r#awards_property = None;
					let mut r#branch_code_property = None;
					let mut r#branch_of_property = None;
					let mut r#brand_property = None;
					let mut r#contact_point_property = None;
					let mut r#contact_points_property = None;
					let mut r#contained_in_property = None;
					let mut r#contained_in_place_property = None;
					let mut r#contains_place_property = None;
					let mut r#corrections_policy_property = None;
					let mut r#currencies_accepted_property = None;
					let mut r#department_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#dissolution_date_property = None;
					let mut r#diversity_policy_property = None;
					let mut r#diversity_staffing_report_property = None;
					let mut r#duns_property = None;
					let mut r#email_property = None;
					let mut r#employee_property = None;
					let mut r#employees_property = None;
					let mut r#ethics_policy_property = None;
					let mut r#event_property = None;
					let mut r#events_property = None;
					let mut r#fax_number_property = None;
					let mut r#founder_property = None;
					let mut r#founders_property = None;
					let mut r#founding_date_property = None;
					let mut r#founding_location_property = None;
					let mut r#funder_property = None;
					let mut r#funding_property = None;
					let mut r#geo_property = None;
					let mut r#geo_contains_property = None;
					let mut r#geo_covered_by_property = None;
					let mut r#geo_covers_property = None;
					let mut r#geo_crosses_property = None;
					let mut r#geo_disjoint_property = None;
					let mut r#geo_equals_property = None;
					let mut r#geo_intersects_property = None;
					let mut r#geo_overlaps_property = None;
					let mut r#geo_touches_property = None;
					let mut r#geo_within_property = None;
					let mut r#global_location_number_property = None;
					let mut r#has_credential_property = None;
					let mut r#has_drive_through_service_property = None;
					let mut r#has_map_property = None;
					let mut r#has_menu_property = None;
					let mut r#has_merchant_return_policy_property = None;
					let mut r#has_offer_catalog_property = None;
					let mut r#has_pos_property = None;
					let mut r#has_product_return_policy_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#interaction_statistic_property = None;
					let mut r#is_accessible_for_free_property = None;
					let mut r#isic_v_4_property = None;
					let mut r#iso_6523_code_property = None;
					let mut r#keywords_property = None;
					let mut r#knows_about_property = None;
					let mut r#knows_language_property = None;
					let mut r#latitude_property = None;
					let mut r#legal_name_property = None;
					let mut r#lei_code_property = None;
					let mut r#location_property = None;
					let mut r#logo_property = None;
					let mut r#longitude_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#makes_offer_property = None;
					let mut r#map_property = None;
					let mut r#maps_property = None;
					let mut r#maximum_attendee_capacity_property = None;
					let mut r#member_property = None;
					let mut r#member_of_property = None;
					let mut r#members_property = None;
					let mut r#menu_property = None;
					let mut r#naics_property = None;
					let mut r#name_property = None;
					let mut r#nonprofit_status_property = None;
					let mut r#number_of_employees_property = None;
					let mut r#opening_hours_property = None;
					let mut r#opening_hours_specification_property = None;
					let mut r#ownership_funding_info_property = None;
					let mut r#owns_property = None;
					let mut r#parent_organization_property = None;
					let mut r#payment_accepted_property = None;
					let mut r#photo_property = None;
					let mut r#photos_property = None;
					let mut r#potential_action_property = None;
					let mut r#price_range_property = None;
					let mut r#public_access_property = None;
					let mut r#publishing_principles_property = None;
					let mut r#review_property = None;
					let mut r#reviews_property = None;
					let mut r#same_as_property = None;
					let mut r#seeks_property = None;
					let mut r#serves_cuisine_property = None;
					let mut r#service_area_property = None;
					let mut r#slogan_property = None;
					let mut r#smoking_allowed_property = None;
					let mut r#special_opening_hours_specification_property = None;
					let mut r#sponsor_property = None;
					let mut r#star_rating_property = None;
					let mut r#sub_organization_property = None;
					let mut r#subject_of_property = None;
					let mut r#tax_id_property = None;
					let mut r#telephone_property = None;
					let mut r#tour_booking_page_property = None;
					let mut r#unnamed_sources_policy_property = None;
					let mut r#url_property = None;
					let mut r#vat_id_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							Field::AcceptsReservations => {
								if r#accepts_reservations_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"acceptsReservations",
									));
								}
								r#accepts_reservations_property = Some({
									struct DeserializeWith(Vec<AcceptsReservationsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::BranchCode => {
								if r#branch_code_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"branchCode",
									));
								}
								r#branch_code_property = Some({
									struct DeserializeWith(Vec<BranchCodeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::BranchOf => {
								if r#branch_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"branchOf",
									));
								}
								r#branch_of_property = Some({
									struct DeserializeWith(Vec<BranchOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::ContainedIn => {
								if r#contained_in_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"containedIn",
									));
								}
								r#contained_in_property = Some({
									struct DeserializeWith(Vec<ContainedInProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ContainedInPlace => {
								if r#contained_in_place_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"containedInPlace",
									));
								}
								r#contained_in_place_property = Some({
									struct DeserializeWith(Vec<ContainedInPlaceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ContainsPlace => {
								if r#contains_place_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"containsPlace",
									));
								}
								r#contains_place_property = Some({
									struct DeserializeWith(Vec<ContainsPlaceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::CurrenciesAccepted => {
								if r#currencies_accepted_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"currenciesAccepted",
									));
								}
								r#currencies_accepted_property = Some({
									struct DeserializeWith(Vec<CurrenciesAcceptedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::Geo => {
								if r#geo_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("geo"));
								}
								r#geo_property = Some({
									struct DeserializeWith(Vec<GeoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoContains => {
								if r#geo_contains_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoContains",
									));
								}
								r#geo_contains_property = Some({
									struct DeserializeWith(Vec<GeoContainsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoCoveredBy => {
								if r#geo_covered_by_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoCoveredBy",
									));
								}
								r#geo_covered_by_property = Some({
									struct DeserializeWith(Vec<GeoCoveredByProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoCovers => {
								if r#geo_covers_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoCovers",
									));
								}
								r#geo_covers_property = Some({
									struct DeserializeWith(Vec<GeoCoversProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoCrosses => {
								if r#geo_crosses_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoCrosses",
									));
								}
								r#geo_crosses_property = Some({
									struct DeserializeWith(Vec<GeoCrossesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoDisjoint => {
								if r#geo_disjoint_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoDisjoint",
									));
								}
								r#geo_disjoint_property = Some({
									struct DeserializeWith(Vec<GeoDisjointProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoEquals => {
								if r#geo_equals_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoEquals",
									));
								}
								r#geo_equals_property = Some({
									struct DeserializeWith(Vec<GeoEqualsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoIntersects => {
								if r#geo_intersects_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoIntersects",
									));
								}
								r#geo_intersects_property = Some({
									struct DeserializeWith(Vec<GeoIntersectsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoOverlaps => {
								if r#geo_overlaps_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoOverlaps",
									));
								}
								r#geo_overlaps_property = Some({
									struct DeserializeWith(Vec<GeoOverlapsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoTouches => {
								if r#geo_touches_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoTouches",
									));
								}
								r#geo_touches_property = Some({
									struct DeserializeWith(Vec<GeoTouchesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::GeoWithin => {
								if r#geo_within_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoWithin",
									));
								}
								r#geo_within_property = Some({
									struct DeserializeWith(Vec<GeoWithinProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::HasDriveThroughService => {
								if r#has_drive_through_service_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasDriveThroughService",
									));
								}
								r#has_drive_through_service_property = Some({
									struct DeserializeWith(Vec<HasDriveThroughServiceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HasMap => {
								if r#has_map_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("hasMap"));
								}
								r#has_map_property = Some({
									struct DeserializeWith(Vec<HasMapProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HasMenu => {
								if r#has_menu_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasMenu",
									));
								}
								r#has_menu_property = Some({
									struct DeserializeWith(Vec<HasMenuProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::IsAccessibleForFree => {
								if r#is_accessible_for_free_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isAccessibleForFree",
									));
								}
								r#is_accessible_for_free_property = Some({
									struct DeserializeWith(Vec<IsAccessibleForFreeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Latitude => {
								if r#latitude_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"latitude",
									));
								}
								r#latitude_property = Some({
									struct DeserializeWith(Vec<LatitudeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::Longitude => {
								if r#longitude_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"longitude",
									));
								}
								r#longitude_property = Some({
									struct DeserializeWith(Vec<LongitudeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Map => {
								if r#map_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("map"));
								}
								r#map_property = Some({
									struct DeserializeWith(Vec<MapProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Maps => {
								if r#maps_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("maps"));
								}
								r#maps_property = Some({
									struct DeserializeWith(Vec<MapsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::MaximumAttendeeCapacity => {
								if r#maximum_attendee_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maximumAttendeeCapacity",
									));
								}
								r#maximum_attendee_capacity_property = Some({
									struct DeserializeWith(Vec<MaximumAttendeeCapacityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::Menu => {
								if r#menu_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("menu"));
								}
								r#menu_property = Some({
									struct DeserializeWith(Vec<MenuProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::OpeningHours => {
								if r#opening_hours_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"openingHours",
									));
								}
								r#opening_hours_property = Some({
									struct DeserializeWith(Vec<OpeningHoursProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::OpeningHoursSpecification => {
								if r#opening_hours_specification_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"openingHoursSpecification",
									));
								}
								r#opening_hours_specification_property = Some({
									struct DeserializeWith(Vec<OpeningHoursSpecificationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::PaymentAccepted => {
								if r#payment_accepted_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"paymentAccepted",
									));
								}
								r#payment_accepted_property = Some({
									struct DeserializeWith(Vec<PaymentAcceptedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Photo => {
								if r#photo_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("photo"));
								}
								r#photo_property = Some({
									struct DeserializeWith(Vec<PhotoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Photos => {
								if r#photos_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("photos"));
								}
								r#photos_property = Some({
									struct DeserializeWith(Vec<PhotosProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::PriceRange => {
								if r#price_range_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"priceRange",
									));
								}
								r#price_range_property = Some({
									struct DeserializeWith(Vec<PriceRangeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PublicAccess => {
								if r#public_access_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"publicAccess",
									));
								}
								r#public_access_property = Some({
									struct DeserializeWith(Vec<PublicAccessProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::ServesCuisine => {
								if r#serves_cuisine_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"servesCuisine",
									));
								}
								r#serves_cuisine_property = Some({
									struct DeserializeWith(Vec<ServesCuisineProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::SmokingAllowed => {
								if r#smoking_allowed_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"smokingAllowed",
									));
								}
								r#smoking_allowed_property = Some({
									struct DeserializeWith(Vec<SmokingAllowedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SpecialOpeningHoursSpecification => {
								if r#special_opening_hours_specification_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"specialOpeningHoursSpecification",
									));
								}
								r#special_opening_hours_specification_property = Some({
									struct DeserializeWith(
										Vec<SpecialOpeningHoursSpecificationProperty>,
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
							Field::StarRating => {
								if r#star_rating_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"starRating",
									));
								}
								r#star_rating_property = Some({
									struct DeserializeWith(Vec<StarRatingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::TourBookingPage => {
								if r#tour_booking_page_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"tourBookingPage",
									));
								}
								r#tour_booking_page_property = Some({
									struct DeserializeWith(Vec<TourBookingPageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
					Ok(BarOrPub {
						r#accepts_reservations: r#accepts_reservations_property.unwrap_or_default(),
						r#actionable_feedback_policy: r#actionable_feedback_policy_property
							.unwrap_or_default(),
						r#additional_property: r#additional_property_property.unwrap_or_default(),
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#address: r#address_property.unwrap_or_default(),
						r#agent_interaction_statistic: r#agent_interaction_statistic_property
							.unwrap_or_default(),
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#alumni: r#alumni_property.unwrap_or_default(),
						r#amenity_feature: r#amenity_feature_property.unwrap_or_default(),
						r#area_served: r#area_served_property.unwrap_or_default(),
						r#award: r#award_property.unwrap_or_default(),
						r#awards: r#awards_property.unwrap_or_default(),
						r#branch_code: r#branch_code_property.unwrap_or_default(),
						r#branch_of: r#branch_of_property.unwrap_or_default(),
						r#brand: r#brand_property.unwrap_or_default(),
						r#contact_point: r#contact_point_property.unwrap_or_default(),
						r#contact_points: r#contact_points_property.unwrap_or_default(),
						r#contained_in: r#contained_in_property.unwrap_or_default(),
						r#contained_in_place: r#contained_in_place_property.unwrap_or_default(),
						r#contains_place: r#contains_place_property.unwrap_or_default(),
						r#corrections_policy: r#corrections_policy_property.unwrap_or_default(),
						r#currencies_accepted: r#currencies_accepted_property.unwrap_or_default(),
						r#department: r#department_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#dissolution_date: r#dissolution_date_property.unwrap_or_default(),
						r#diversity_policy: r#diversity_policy_property.unwrap_or_default(),
						r#diversity_staffing_report: r#diversity_staffing_report_property
							.unwrap_or_default(),
						r#duns: r#duns_property.unwrap_or_default(),
						r#email: r#email_property.unwrap_or_default(),
						r#employee: r#employee_property.unwrap_or_default(),
						r#employees: r#employees_property.unwrap_or_default(),
						r#ethics_policy: r#ethics_policy_property.unwrap_or_default(),
						r#event: r#event_property.unwrap_or_default(),
						r#events: r#events_property.unwrap_or_default(),
						r#fax_number: r#fax_number_property.unwrap_or_default(),
						r#founder: r#founder_property.unwrap_or_default(),
						r#founders: r#founders_property.unwrap_or_default(),
						r#founding_date: r#founding_date_property.unwrap_or_default(),
						r#founding_location: r#founding_location_property.unwrap_or_default(),
						r#funder: r#funder_property.unwrap_or_default(),
						r#funding: r#funding_property.unwrap_or_default(),
						r#geo: r#geo_property.unwrap_or_default(),
						r#geo_contains: r#geo_contains_property.unwrap_or_default(),
						r#geo_covered_by: r#geo_covered_by_property.unwrap_or_default(),
						r#geo_covers: r#geo_covers_property.unwrap_or_default(),
						r#geo_crosses: r#geo_crosses_property.unwrap_or_default(),
						r#geo_disjoint: r#geo_disjoint_property.unwrap_or_default(),
						r#geo_equals: r#geo_equals_property.unwrap_or_default(),
						r#geo_intersects: r#geo_intersects_property.unwrap_or_default(),
						r#geo_overlaps: r#geo_overlaps_property.unwrap_or_default(),
						r#geo_touches: r#geo_touches_property.unwrap_or_default(),
						r#geo_within: r#geo_within_property.unwrap_or_default(),
						r#global_location_number: r#global_location_number_property
							.unwrap_or_default(),
						r#has_credential: r#has_credential_property.unwrap_or_default(),
						r#has_drive_through_service: r#has_drive_through_service_property
							.unwrap_or_default(),
						r#has_map: r#has_map_property.unwrap_or_default(),
						r#has_menu: r#has_menu_property.unwrap_or_default(),
						r#has_merchant_return_policy: r#has_merchant_return_policy_property
							.unwrap_or_default(),
						r#has_offer_catalog: r#has_offer_catalog_property.unwrap_or_default(),
						r#has_pos: r#has_pos_property.unwrap_or_default(),
						r#has_product_return_policy: r#has_product_return_policy_property
							.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#interaction_statistic: r#interaction_statistic_property
							.unwrap_or_default(),
						r#is_accessible_for_free: r#is_accessible_for_free_property
							.unwrap_or_default(),
						r#isic_v_4: r#isic_v_4_property.unwrap_or_default(),
						r#iso_6523_code: r#iso_6523_code_property.unwrap_or_default(),
						r#keywords: r#keywords_property.unwrap_or_default(),
						r#knows_about: r#knows_about_property.unwrap_or_default(),
						r#knows_language: r#knows_language_property.unwrap_or_default(),
						r#latitude: r#latitude_property.unwrap_or_default(),
						r#legal_name: r#legal_name_property.unwrap_or_default(),
						r#lei_code: r#lei_code_property.unwrap_or_default(),
						r#location: r#location_property.unwrap_or_default(),
						r#logo: r#logo_property.unwrap_or_default(),
						r#longitude: r#longitude_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#makes_offer: r#makes_offer_property.unwrap_or_default(),
						r#map: r#map_property.unwrap_or_default(),
						r#maps: r#maps_property.unwrap_or_default(),
						r#maximum_attendee_capacity: r#maximum_attendee_capacity_property
							.unwrap_or_default(),
						r#member: r#member_property.unwrap_or_default(),
						r#member_of: r#member_of_property.unwrap_or_default(),
						r#members: r#members_property.unwrap_or_default(),
						r#menu: r#menu_property.unwrap_or_default(),
						r#naics: r#naics_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#nonprofit_status: r#nonprofit_status_property.unwrap_or_default(),
						r#number_of_employees: r#number_of_employees_property.unwrap_or_default(),
						r#opening_hours: r#opening_hours_property.unwrap_or_default(),
						r#opening_hours_specification: r#opening_hours_specification_property
							.unwrap_or_default(),
						r#ownership_funding_info: r#ownership_funding_info_property
							.unwrap_or_default(),
						r#owns: r#owns_property.unwrap_or_default(),
						r#parent_organization: r#parent_organization_property.unwrap_or_default(),
						r#payment_accepted: r#payment_accepted_property.unwrap_or_default(),
						r#photo: r#photo_property.unwrap_or_default(),
						r#photos: r#photos_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#price_range: r#price_range_property.unwrap_or_default(),
						r#public_access: r#public_access_property.unwrap_or_default(),
						r#publishing_principles: r#publishing_principles_property
							.unwrap_or_default(),
						r#review: r#review_property.unwrap_or_default(),
						r#reviews: r#reviews_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#seeks: r#seeks_property.unwrap_or_default(),
						r#serves_cuisine: r#serves_cuisine_property.unwrap_or_default(),
						r#service_area: r#service_area_property.unwrap_or_default(),
						r#slogan: r#slogan_property.unwrap_or_default(),
						r#smoking_allowed: r#smoking_allowed_property.unwrap_or_default(),
						r#special_opening_hours_specification:
							r#special_opening_hours_specification_property.unwrap_or_default(),
						r#sponsor: r#sponsor_property.unwrap_or_default(),
						r#star_rating: r#star_rating_property.unwrap_or_default(),
						r#sub_organization: r#sub_organization_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#tax_id: r#tax_id_property.unwrap_or_default(),
						r#telephone: r#telephone_property.unwrap_or_default(),
						r#tour_booking_page: r#tour_booking_page_property.unwrap_or_default(),
						r#unnamed_sources_policy: r#unnamed_sources_policy_property
							.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
						r#vat_id: r#vat_id_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"acceptsReservations",
				"actionableFeedbackPolicy",
				"additionalProperty",
				"additionalType",
				"address",
				"agentInteractionStatistic",
				"aggregateRating",
				"alternateName",
				"alumni",
				"amenityFeature",
				"areaServed",
				"award",
				"awards",
				"branchCode",
				"branchOf",
				"brand",
				"contactPoint",
				"contactPoints",
				"containedIn",
				"containedInPlace",
				"containsPlace",
				"correctionsPolicy",
				"currenciesAccepted",
				"department",
				"description",
				"disambiguatingDescription",
				"dissolutionDate",
				"diversityPolicy",
				"diversityStaffingReport",
				"duns",
				"email",
				"employee",
				"employees",
				"ethicsPolicy",
				"event",
				"events",
				"faxNumber",
				"founder",
				"founders",
				"foundingDate",
				"foundingLocation",
				"funder",
				"funding",
				"geo",
				"geoContains",
				"geoCoveredBy",
				"geoCovers",
				"geoCrosses",
				"geoDisjoint",
				"geoEquals",
				"geoIntersects",
				"geoOverlaps",
				"geoTouches",
				"geoWithin",
				"globalLocationNumber",
				"hasCredential",
				"hasDriveThroughService",
				"hasMap",
				"hasMenu",
				"hasMerchantReturnPolicy",
				"hasOfferCatalog",
				"hasPOS",
				"hasProductReturnPolicy",
				"identifier",
				"image",
				"interactionStatistic",
				"isAccessibleForFree",
				"isicV4",
				"iso6523Code",
				"keywords",
				"knowsAbout",
				"knowsLanguage",
				"latitude",
				"legalName",
				"leiCode",
				"location",
				"logo",
				"longitude",
				"mainEntityOfPage",
				"makesOffer",
				"map",
				"maps",
				"maximumAttendeeCapacity",
				"member",
				"memberOf",
				"members",
				"menu",
				"naics",
				"name",
				"nonprofitStatus",
				"numberOfEmployees",
				"openingHours",
				"openingHoursSpecification",
				"ownershipFundingInfo",
				"owns",
				"parentOrganization",
				"paymentAccepted",
				"photo",
				"photos",
				"potentialAction",
				"priceRange",
				"publicAccess",
				"publishingPrinciples",
				"review",
				"reviews",
				"sameAs",
				"seeks",
				"servesCuisine",
				"serviceArea",
				"slogan",
				"smokingAllowed",
				"specialOpeningHoursSpecification",
				"sponsor",
				"starRating",
				"subOrganization",
				"subjectOf",
				"taxID",
				"telephone",
				"tourBookingPage",
				"unnamedSourcesPolicy",
				"url",
				"vatID",
			];
			deserializer.deserialize_struct("BarOrPub", FIELDS, ClassVisitor)
		}
	}
}
