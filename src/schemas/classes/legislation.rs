use super::*;
/// <https://schema.org/Legislation>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Legislation {
	pub r#about: Vec<AboutProperty>,
	pub r#abstract: Vec<AbstractProperty>,
	pub r#access_mode: Vec<AccessModeProperty>,
	pub r#access_mode_sufficient: Vec<AccessModeSufficientProperty>,
	pub r#accessibility_api: Vec<AccessibilityApiProperty>,
	pub r#accessibility_control: Vec<AccessibilityControlProperty>,
	pub r#accessibility_feature: Vec<AccessibilityFeatureProperty>,
	pub r#accessibility_hazard: Vec<AccessibilityHazardProperty>,
	pub r#accessibility_summary: Vec<AccessibilitySummaryProperty>,
	pub r#accountable_person: Vec<AccountablePersonProperty>,
	pub r#acquire_license_page: Vec<AcquireLicensePageProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#alternative_headline: Vec<AlternativeHeadlineProperty>,
	pub r#archived_at: Vec<ArchivedAtProperty>,
	pub r#assesses: Vec<AssessesProperty>,
	pub r#associated_media: Vec<AssociatedMediaProperty>,
	pub r#audience: Vec<AudienceProperty>,
	pub r#audio: Vec<AudioProperty>,
	pub r#author: Vec<AuthorProperty>,
	pub r#award: Vec<AwardProperty>,
	pub r#awards: Vec<AwardsProperty>,
	pub r#character: Vec<CharacterProperty>,
	pub r#citation: Vec<CitationProperty>,
	pub r#comment: Vec<CommentProperty>,
	pub r#comment_count: Vec<CommentCountProperty>,
	pub r#conditions_of_access: Vec<ConditionsOfAccessProperty>,
	pub r#content_location: Vec<ContentLocationProperty>,
	pub r#content_rating: Vec<ContentRatingProperty>,
	pub r#content_reference_time: Vec<ContentReferenceTimeProperty>,
	pub r#contributor: Vec<ContributorProperty>,
	pub r#copyright_holder: Vec<CopyrightHolderProperty>,
	pub r#copyright_notice: Vec<CopyrightNoticeProperty>,
	pub r#copyright_year: Vec<CopyrightYearProperty>,
	pub r#correction: Vec<CorrectionProperty>,
	pub r#country_of_origin: Vec<CountryOfOriginProperty>,
	pub r#creative_work_status: Vec<CreativeWorkStatusProperty>,
	pub r#creator: Vec<CreatorProperty>,
	pub r#credit_text: Vec<CreditTextProperty>,
	pub r#date_created: Vec<DateCreatedProperty>,
	pub r#date_modified: Vec<DateModifiedProperty>,
	pub r#date_published: Vec<DatePublishedProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#discussion_url: Vec<DiscussionUrlProperty>,
	pub r#edit_eidr: Vec<EditEidrProperty>,
	pub r#editor: Vec<EditorProperty>,
	pub r#educational_alignment: Vec<EducationalAlignmentProperty>,
	pub r#educational_level: Vec<EducationalLevelProperty>,
	pub r#educational_use: Vec<EducationalUseProperty>,
	pub r#encoding: Vec<EncodingProperty>,
	pub r#encoding_format: Vec<EncodingFormatProperty>,
	pub r#encodings: Vec<EncodingsProperty>,
	pub r#example_of_work: Vec<ExampleOfWorkProperty>,
	pub r#expires: Vec<ExpiresProperty>,
	pub r#file_format: Vec<FileFormatProperty>,
	pub r#funder: Vec<FunderProperty>,
	pub r#funding: Vec<FundingProperty>,
	pub r#genre: Vec<GenreProperty>,
	pub r#has_part: Vec<HasPartProperty>,
	pub r#headline: Vec<HeadlineProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#in_language: Vec<InLanguageProperty>,
	pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
	pub r#interactivity_type: Vec<InteractivityTypeProperty>,
	pub r#interpreted_as_claim: Vec<InterpretedAsClaimProperty>,
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	pub r#is_based_on: Vec<IsBasedOnProperty>,
	pub r#is_based_on_url: Vec<IsBasedOnUrlProperty>,
	pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
	pub r#is_part_of: Vec<IsPartOfProperty>,
	pub r#jurisdiction: Vec<JurisdictionProperty>,
	pub r#keywords: Vec<KeywordsProperty>,
	pub r#learning_resource_type: Vec<LearningResourceTypeProperty>,
	pub r#legislation_applies: Vec<LegislationAppliesProperty>,
	pub r#legislation_changes: Vec<LegislationChangesProperty>,
	pub r#legislation_consolidates: Vec<LegislationConsolidatesProperty>,
	pub r#legislation_date: Vec<LegislationDateProperty>,
	pub r#legislation_date_version: Vec<LegislationDateVersionProperty>,
	pub r#legislation_identifier: Vec<LegislationIdentifierProperty>,
	pub r#legislation_jurisdiction: Vec<LegislationJurisdictionProperty>,
	pub r#legislation_legal_force: Vec<LegislationLegalForceProperty>,
	pub r#legislation_passed_by: Vec<LegislationPassedByProperty>,
	pub r#legislation_responsible: Vec<LegislationResponsibleProperty>,
	pub r#legislation_transposes: Vec<LegislationTransposesProperty>,
	pub r#legislation_type: Vec<LegislationTypeProperty>,
	pub r#license: Vec<LicenseProperty>,
	pub r#location_created: Vec<LocationCreatedProperty>,
	pub r#main_entity: Vec<MainEntityProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#maintainer: Vec<MaintainerProperty>,
	pub r#material: Vec<MaterialProperty>,
	pub r#material_extent: Vec<MaterialExtentProperty>,
	pub r#mentions: Vec<MentionsProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#offers: Vec<OffersProperty>,
	pub r#pattern: Vec<PatternProperty>,
	pub r#position: Vec<PositionProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#producer: Vec<ProducerProperty>,
	pub r#provider: Vec<ProviderProperty>,
	pub r#publication: Vec<PublicationProperty>,
	pub r#publisher: Vec<PublisherProperty>,
	pub r#publisher_imprint: Vec<PublisherImprintProperty>,
	pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
	pub r#recorded_at: Vec<RecordedAtProperty>,
	pub r#released_event: Vec<ReleasedEventProperty>,
	pub r#review: Vec<ReviewProperty>,
	pub r#reviews: Vec<ReviewsProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#schema_version: Vec<SchemaVersionProperty>,
	pub r#sd_date_published: Vec<SdDatePublishedProperty>,
	pub r#sd_license: Vec<SdLicenseProperty>,
	pub r#sd_publisher: Vec<SdPublisherProperty>,
	pub r#size: Vec<SizeProperty>,
	pub r#source_organization: Vec<SourceOrganizationProperty>,
	pub r#spatial: Vec<SpatialProperty>,
	pub r#spatial_coverage: Vec<SpatialCoverageProperty>,
	pub r#sponsor: Vec<SponsorProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#teaches: Vec<TeachesProperty>,
	pub r#temporal: Vec<TemporalProperty>,
	pub r#temporal_coverage: Vec<TemporalCoverageProperty>,
	pub r#text: Vec<TextProperty>,
	pub r#thumbnail: Vec<ThumbnailProperty>,
	pub r#thumbnail_url: Vec<ThumbnailUrlProperty>,
	pub r#time_required: Vec<TimeRequiredProperty>,
	pub r#translation_of_work: Vec<TranslationOfWorkProperty>,
	pub r#translator: Vec<TranslatorProperty>,
	pub r#typical_age_range: Vec<TypicalAgeRangeProperty>,
	pub r#url: Vec<UrlProperty>,
	pub r#usage_info: Vec<UsageInfoProperty>,
	pub r#version: Vec<VersionProperty>,
	pub r#video: Vec<VideoProperty>,
	pub r#work_example: Vec<WorkExampleProperty>,
	pub r#work_translation: Vec<WorkTranslationProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Legislation {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#about) as usize,
				!Vec::is_empty(&self.r#abstract) as usize,
				!Vec::is_empty(&self.r#access_mode) as usize,
				!Vec::is_empty(&self.r#access_mode_sufficient) as usize,
				!Vec::is_empty(&self.r#accessibility_api) as usize,
				!Vec::is_empty(&self.r#accessibility_control) as usize,
				!Vec::is_empty(&self.r#accessibility_feature) as usize,
				!Vec::is_empty(&self.r#accessibility_hazard) as usize,
				!Vec::is_empty(&self.r#accessibility_summary) as usize,
				!Vec::is_empty(&self.r#accountable_person) as usize,
				!Vec::is_empty(&self.r#acquire_license_page) as usize,
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#alternative_headline) as usize,
				!Vec::is_empty(&self.r#archived_at) as usize,
				!Vec::is_empty(&self.r#assesses) as usize,
				!Vec::is_empty(&self.r#associated_media) as usize,
				!Vec::is_empty(&self.r#audience) as usize,
				!Vec::is_empty(&self.r#audio) as usize,
				!Vec::is_empty(&self.r#author) as usize,
				!Vec::is_empty(&self.r#award) as usize,
				!Vec::is_empty(&self.r#awards) as usize,
				!Vec::is_empty(&self.r#character) as usize,
				!Vec::is_empty(&self.r#citation) as usize,
				!Vec::is_empty(&self.r#comment) as usize,
				!Vec::is_empty(&self.r#comment_count) as usize,
				!Vec::is_empty(&self.r#conditions_of_access) as usize,
				!Vec::is_empty(&self.r#content_location) as usize,
				!Vec::is_empty(&self.r#content_rating) as usize,
				!Vec::is_empty(&self.r#content_reference_time) as usize,
				!Vec::is_empty(&self.r#contributor) as usize,
				!Vec::is_empty(&self.r#copyright_holder) as usize,
				!Vec::is_empty(&self.r#copyright_notice) as usize,
				!Vec::is_empty(&self.r#copyright_year) as usize,
				!Vec::is_empty(&self.r#correction) as usize,
				!Vec::is_empty(&self.r#country_of_origin) as usize,
				!Vec::is_empty(&self.r#creative_work_status) as usize,
				!Vec::is_empty(&self.r#creator) as usize,
				!Vec::is_empty(&self.r#credit_text) as usize,
				!Vec::is_empty(&self.r#date_created) as usize,
				!Vec::is_empty(&self.r#date_modified) as usize,
				!Vec::is_empty(&self.r#date_published) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#discussion_url) as usize,
				!Vec::is_empty(&self.r#edit_eidr) as usize,
				!Vec::is_empty(&self.r#editor) as usize,
				!Vec::is_empty(&self.r#educational_alignment) as usize,
				!Vec::is_empty(&self.r#educational_level) as usize,
				!Vec::is_empty(&self.r#educational_use) as usize,
				!Vec::is_empty(&self.r#encoding) as usize,
				!Vec::is_empty(&self.r#encoding_format) as usize,
				!Vec::is_empty(&self.r#encodings) as usize,
				!Vec::is_empty(&self.r#example_of_work) as usize,
				!Vec::is_empty(&self.r#expires) as usize,
				!Vec::is_empty(&self.r#file_format) as usize,
				!Vec::is_empty(&self.r#funder) as usize,
				!Vec::is_empty(&self.r#funding) as usize,
				!Vec::is_empty(&self.r#genre) as usize,
				!Vec::is_empty(&self.r#has_part) as usize,
				!Vec::is_empty(&self.r#headline) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#in_language) as usize,
				!Vec::is_empty(&self.r#interaction_statistic) as usize,
				!Vec::is_empty(&self.r#interactivity_type) as usize,
				!Vec::is_empty(&self.r#interpreted_as_claim) as usize,
				!Vec::is_empty(&self.r#is_accessible_for_free) as usize,
				!Vec::is_empty(&self.r#is_based_on) as usize,
				!Vec::is_empty(&self.r#is_based_on_url) as usize,
				!Vec::is_empty(&self.r#is_family_friendly) as usize,
				!Vec::is_empty(&self.r#is_part_of) as usize,
				!Vec::is_empty(&self.r#jurisdiction) as usize,
				!Vec::is_empty(&self.r#keywords) as usize,
				!Vec::is_empty(&self.r#learning_resource_type) as usize,
				!Vec::is_empty(&self.r#legislation_applies) as usize,
				!Vec::is_empty(&self.r#legislation_changes) as usize,
				!Vec::is_empty(&self.r#legislation_consolidates) as usize,
				!Vec::is_empty(&self.r#legislation_date) as usize,
				!Vec::is_empty(&self.r#legislation_date_version) as usize,
				!Vec::is_empty(&self.r#legislation_identifier) as usize,
				!Vec::is_empty(&self.r#legislation_jurisdiction) as usize,
				!Vec::is_empty(&self.r#legislation_legal_force) as usize,
				!Vec::is_empty(&self.r#legislation_passed_by) as usize,
				!Vec::is_empty(&self.r#legislation_responsible) as usize,
				!Vec::is_empty(&self.r#legislation_transposes) as usize,
				!Vec::is_empty(&self.r#legislation_type) as usize,
				!Vec::is_empty(&self.r#license) as usize,
				!Vec::is_empty(&self.r#location_created) as usize,
				!Vec::is_empty(&self.r#main_entity) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#maintainer) as usize,
				!Vec::is_empty(&self.r#material) as usize,
				!Vec::is_empty(&self.r#material_extent) as usize,
				!Vec::is_empty(&self.r#mentions) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#pattern) as usize,
				!Vec::is_empty(&self.r#position) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#producer) as usize,
				!Vec::is_empty(&self.r#provider) as usize,
				!Vec::is_empty(&self.r#publication) as usize,
				!Vec::is_empty(&self.r#publisher) as usize,
				!Vec::is_empty(&self.r#publisher_imprint) as usize,
				!Vec::is_empty(&self.r#publishing_principles) as usize,
				!Vec::is_empty(&self.r#recorded_at) as usize,
				!Vec::is_empty(&self.r#released_event) as usize,
				!Vec::is_empty(&self.r#review) as usize,
				!Vec::is_empty(&self.r#reviews) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#schema_version) as usize,
				!Vec::is_empty(&self.r#sd_date_published) as usize,
				!Vec::is_empty(&self.r#sd_license) as usize,
				!Vec::is_empty(&self.r#sd_publisher) as usize,
				!Vec::is_empty(&self.r#size) as usize,
				!Vec::is_empty(&self.r#source_organization) as usize,
				!Vec::is_empty(&self.r#spatial) as usize,
				!Vec::is_empty(&self.r#spatial_coverage) as usize,
				!Vec::is_empty(&self.r#sponsor) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#teaches) as usize,
				!Vec::is_empty(&self.r#temporal) as usize,
				!Vec::is_empty(&self.r#temporal_coverage) as usize,
				!Vec::is_empty(&self.r#text) as usize,
				!Vec::is_empty(&self.r#thumbnail) as usize,
				!Vec::is_empty(&self.r#thumbnail_url) as usize,
				!Vec::is_empty(&self.r#time_required) as usize,
				!Vec::is_empty(&self.r#translation_of_work) as usize,
				!Vec::is_empty(&self.r#translator) as usize,
				!Vec::is_empty(&self.r#typical_age_range) as usize,
				!Vec::is_empty(&self.r#url) as usize,
				!Vec::is_empty(&self.r#usage_info) as usize,
				!Vec::is_empty(&self.r#version) as usize,
				!Vec::is_empty(&self.r#video) as usize,
				!Vec::is_empty(&self.r#work_example) as usize,
				!Vec::is_empty(&self.r#work_translation) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "Legislation", len)?;
			if !Vec::is_empty(&self.r#about) {
				serialize_struct.serialize_field("about", {
					struct SerializeWith<'a>(&'a Vec<AboutProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#about)
				})?;
			} else {
				serialize_struct.skip_field("about")?;
			}
			if !Vec::is_empty(&self.r#abstract) {
				serialize_struct.serialize_field("abstract", {
					struct SerializeWith<'a>(&'a Vec<AbstractProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#abstract)
				})?;
			} else {
				serialize_struct.skip_field("abstract")?;
			}
			if !Vec::is_empty(&self.r#access_mode) {
				serialize_struct.serialize_field("accessMode", {
					struct SerializeWith<'a>(&'a Vec<AccessModeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#access_mode)
				})?;
			} else {
				serialize_struct.skip_field("accessMode")?;
			}
			if !Vec::is_empty(&self.r#access_mode_sufficient) {
				serialize_struct.serialize_field("accessModeSufficient", {
					struct SerializeWith<'a>(&'a Vec<AccessModeSufficientProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#access_mode_sufficient)
				})?;
			} else {
				serialize_struct.skip_field("accessModeSufficient")?;
			}
			if !Vec::is_empty(&self.r#accessibility_api) {
				serialize_struct.serialize_field("accessibilityAPI", {
					struct SerializeWith<'a>(&'a Vec<AccessibilityApiProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#accessibility_api)
				})?;
			} else {
				serialize_struct.skip_field("accessibilityAPI")?;
			}
			if !Vec::is_empty(&self.r#accessibility_control) {
				serialize_struct.serialize_field("accessibilityControl", {
					struct SerializeWith<'a>(&'a Vec<AccessibilityControlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#accessibility_control)
				})?;
			} else {
				serialize_struct.skip_field("accessibilityControl")?;
			}
			if !Vec::is_empty(&self.r#accessibility_feature) {
				serialize_struct.serialize_field("accessibilityFeature", {
					struct SerializeWith<'a>(&'a Vec<AccessibilityFeatureProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#accessibility_feature)
				})?;
			} else {
				serialize_struct.skip_field("accessibilityFeature")?;
			}
			if !Vec::is_empty(&self.r#accessibility_hazard) {
				serialize_struct.serialize_field("accessibilityHazard", {
					struct SerializeWith<'a>(&'a Vec<AccessibilityHazardProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#accessibility_hazard)
				})?;
			} else {
				serialize_struct.skip_field("accessibilityHazard")?;
			}
			if !Vec::is_empty(&self.r#accessibility_summary) {
				serialize_struct.serialize_field("accessibilitySummary", {
					struct SerializeWith<'a>(&'a Vec<AccessibilitySummaryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#accessibility_summary)
				})?;
			} else {
				serialize_struct.skip_field("accessibilitySummary")?;
			}
			if !Vec::is_empty(&self.r#accountable_person) {
				serialize_struct.serialize_field("accountablePerson", {
					struct SerializeWith<'a>(&'a Vec<AccountablePersonProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#accountable_person)
				})?;
			} else {
				serialize_struct.skip_field("accountablePerson")?;
			}
			if !Vec::is_empty(&self.r#acquire_license_page) {
				serialize_struct.serialize_field("acquireLicensePage", {
					struct SerializeWith<'a>(&'a Vec<AcquireLicensePageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#acquire_license_page)
				})?;
			} else {
				serialize_struct.skip_field("acquireLicensePage")?;
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
			if !Vec::is_empty(&self.r#alternative_headline) {
				serialize_struct.serialize_field("alternativeHeadline", {
					struct SerializeWith<'a>(&'a Vec<AlternativeHeadlineProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#alternative_headline)
				})?;
			} else {
				serialize_struct.skip_field("alternativeHeadline")?;
			}
			if !Vec::is_empty(&self.r#archived_at) {
				serialize_struct.serialize_field("archivedAt", {
					struct SerializeWith<'a>(&'a Vec<ArchivedAtProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#archived_at)
				})?;
			} else {
				serialize_struct.skip_field("archivedAt")?;
			}
			if !Vec::is_empty(&self.r#assesses) {
				serialize_struct.serialize_field("assesses", {
					struct SerializeWith<'a>(&'a Vec<AssessesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#assesses)
				})?;
			} else {
				serialize_struct.skip_field("assesses")?;
			}
			if !Vec::is_empty(&self.r#associated_media) {
				serialize_struct.serialize_field("associatedMedia", {
					struct SerializeWith<'a>(&'a Vec<AssociatedMediaProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#associated_media)
				})?;
			} else {
				serialize_struct.skip_field("associatedMedia")?;
			}
			if !Vec::is_empty(&self.r#audience) {
				serialize_struct.serialize_field("audience", {
					struct SerializeWith<'a>(&'a Vec<AudienceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#audience)
				})?;
			} else {
				serialize_struct.skip_field("audience")?;
			}
			if !Vec::is_empty(&self.r#audio) {
				serialize_struct.serialize_field("audio", {
					struct SerializeWith<'a>(&'a Vec<AudioProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#audio)
				})?;
			} else {
				serialize_struct.skip_field("audio")?;
			}
			if !Vec::is_empty(&self.r#author) {
				serialize_struct.serialize_field("author", {
					struct SerializeWith<'a>(&'a Vec<AuthorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#author)
				})?;
			} else {
				serialize_struct.skip_field("author")?;
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
			if !Vec::is_empty(&self.r#character) {
				serialize_struct.serialize_field("character", {
					struct SerializeWith<'a>(&'a Vec<CharacterProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#character)
				})?;
			} else {
				serialize_struct.skip_field("character")?;
			}
			if !Vec::is_empty(&self.r#citation) {
				serialize_struct.serialize_field("citation", {
					struct SerializeWith<'a>(&'a Vec<CitationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#citation)
				})?;
			} else {
				serialize_struct.skip_field("citation")?;
			}
			if !Vec::is_empty(&self.r#comment) {
				serialize_struct.serialize_field("comment", {
					struct SerializeWith<'a>(&'a Vec<CommentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#comment)
				})?;
			} else {
				serialize_struct.skip_field("comment")?;
			}
			if !Vec::is_empty(&self.r#comment_count) {
				serialize_struct.serialize_field("commentCount", {
					struct SerializeWith<'a>(&'a Vec<CommentCountProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#comment_count)
				})?;
			} else {
				serialize_struct.skip_field("commentCount")?;
			}
			if !Vec::is_empty(&self.r#conditions_of_access) {
				serialize_struct.serialize_field("conditionsOfAccess", {
					struct SerializeWith<'a>(&'a Vec<ConditionsOfAccessProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#conditions_of_access)
				})?;
			} else {
				serialize_struct.skip_field("conditionsOfAccess")?;
			}
			if !Vec::is_empty(&self.r#content_location) {
				serialize_struct.serialize_field("contentLocation", {
					struct SerializeWith<'a>(&'a Vec<ContentLocationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#content_location)
				})?;
			} else {
				serialize_struct.skip_field("contentLocation")?;
			}
			if !Vec::is_empty(&self.r#content_rating) {
				serialize_struct.serialize_field("contentRating", {
					struct SerializeWith<'a>(&'a Vec<ContentRatingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#content_rating)
				})?;
			} else {
				serialize_struct.skip_field("contentRating")?;
			}
			if !Vec::is_empty(&self.r#content_reference_time) {
				serialize_struct.serialize_field("contentReferenceTime", {
					struct SerializeWith<'a>(&'a Vec<ContentReferenceTimeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#content_reference_time)
				})?;
			} else {
				serialize_struct.skip_field("contentReferenceTime")?;
			}
			if !Vec::is_empty(&self.r#contributor) {
				serialize_struct.serialize_field("contributor", {
					struct SerializeWith<'a>(&'a Vec<ContributorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#contributor)
				})?;
			} else {
				serialize_struct.skip_field("contributor")?;
			}
			if !Vec::is_empty(&self.r#copyright_holder) {
				serialize_struct.serialize_field("copyrightHolder", {
					struct SerializeWith<'a>(&'a Vec<CopyrightHolderProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#copyright_holder)
				})?;
			} else {
				serialize_struct.skip_field("copyrightHolder")?;
			}
			if !Vec::is_empty(&self.r#copyright_notice) {
				serialize_struct.serialize_field("copyrightNotice", {
					struct SerializeWith<'a>(&'a Vec<CopyrightNoticeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#copyright_notice)
				})?;
			} else {
				serialize_struct.skip_field("copyrightNotice")?;
			}
			if !Vec::is_empty(&self.r#copyright_year) {
				serialize_struct.serialize_field("copyrightYear", {
					struct SerializeWith<'a>(&'a Vec<CopyrightYearProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#copyright_year)
				})?;
			} else {
				serialize_struct.skip_field("copyrightYear")?;
			}
			if !Vec::is_empty(&self.r#correction) {
				serialize_struct.serialize_field("correction", {
					struct SerializeWith<'a>(&'a Vec<CorrectionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#correction)
				})?;
			} else {
				serialize_struct.skip_field("correction")?;
			}
			if !Vec::is_empty(&self.r#country_of_origin) {
				serialize_struct.serialize_field("countryOfOrigin", {
					struct SerializeWith<'a>(&'a Vec<CountryOfOriginProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#country_of_origin)
				})?;
			} else {
				serialize_struct.skip_field("countryOfOrigin")?;
			}
			if !Vec::is_empty(&self.r#creative_work_status) {
				serialize_struct.serialize_field("creativeWorkStatus", {
					struct SerializeWith<'a>(&'a Vec<CreativeWorkStatusProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#creative_work_status)
				})?;
			} else {
				serialize_struct.skip_field("creativeWorkStatus")?;
			}
			if !Vec::is_empty(&self.r#creator) {
				serialize_struct.serialize_field("creator", {
					struct SerializeWith<'a>(&'a Vec<CreatorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#creator)
				})?;
			} else {
				serialize_struct.skip_field("creator")?;
			}
			if !Vec::is_empty(&self.r#credit_text) {
				serialize_struct.serialize_field("creditText", {
					struct SerializeWith<'a>(&'a Vec<CreditTextProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#credit_text)
				})?;
			} else {
				serialize_struct.skip_field("creditText")?;
			}
			if !Vec::is_empty(&self.r#date_created) {
				serialize_struct.serialize_field("dateCreated", {
					struct SerializeWith<'a>(&'a Vec<DateCreatedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#date_created)
				})?;
			} else {
				serialize_struct.skip_field("dateCreated")?;
			}
			if !Vec::is_empty(&self.r#date_modified) {
				serialize_struct.serialize_field("dateModified", {
					struct SerializeWith<'a>(&'a Vec<DateModifiedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#date_modified)
				})?;
			} else {
				serialize_struct.skip_field("dateModified")?;
			}
			if !Vec::is_empty(&self.r#date_published) {
				serialize_struct.serialize_field("datePublished", {
					struct SerializeWith<'a>(&'a Vec<DatePublishedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#date_published)
				})?;
			} else {
				serialize_struct.skip_field("datePublished")?;
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
			if !Vec::is_empty(&self.r#discussion_url) {
				serialize_struct.serialize_field("discussionUrl", {
					struct SerializeWith<'a>(&'a Vec<DiscussionUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#discussion_url)
				})?;
			} else {
				serialize_struct.skip_field("discussionUrl")?;
			}
			if !Vec::is_empty(&self.r#edit_eidr) {
				serialize_struct.serialize_field("editEIDR", {
					struct SerializeWith<'a>(&'a Vec<EditEidrProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#edit_eidr)
				})?;
			} else {
				serialize_struct.skip_field("editEIDR")?;
			}
			if !Vec::is_empty(&self.r#editor) {
				serialize_struct.serialize_field("editor", {
					struct SerializeWith<'a>(&'a Vec<EditorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#editor)
				})?;
			} else {
				serialize_struct.skip_field("editor")?;
			}
			if !Vec::is_empty(&self.r#educational_alignment) {
				serialize_struct.serialize_field("educationalAlignment", {
					struct SerializeWith<'a>(&'a Vec<EducationalAlignmentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#educational_alignment)
				})?;
			} else {
				serialize_struct.skip_field("educationalAlignment")?;
			}
			if !Vec::is_empty(&self.r#educational_level) {
				serialize_struct.serialize_field("educationalLevel", {
					struct SerializeWith<'a>(&'a Vec<EducationalLevelProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#educational_level)
				})?;
			} else {
				serialize_struct.skip_field("educationalLevel")?;
			}
			if !Vec::is_empty(&self.r#educational_use) {
				serialize_struct.serialize_field("educationalUse", {
					struct SerializeWith<'a>(&'a Vec<EducationalUseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#educational_use)
				})?;
			} else {
				serialize_struct.skip_field("educationalUse")?;
			}
			if !Vec::is_empty(&self.r#encoding) {
				serialize_struct.serialize_field("encoding", {
					struct SerializeWith<'a>(&'a Vec<EncodingProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#encoding)
				})?;
			} else {
				serialize_struct.skip_field("encoding")?;
			}
			if !Vec::is_empty(&self.r#encoding_format) {
				serialize_struct.serialize_field("encodingFormat", {
					struct SerializeWith<'a>(&'a Vec<EncodingFormatProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#encoding_format)
				})?;
			} else {
				serialize_struct.skip_field("encodingFormat")?;
			}
			if !Vec::is_empty(&self.r#encodings) {
				serialize_struct.serialize_field("encodings", {
					struct SerializeWith<'a>(&'a Vec<EncodingsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#encodings)
				})?;
			} else {
				serialize_struct.skip_field("encodings")?;
			}
			if !Vec::is_empty(&self.r#example_of_work) {
				serialize_struct.serialize_field("exampleOfWork", {
					struct SerializeWith<'a>(&'a Vec<ExampleOfWorkProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#example_of_work)
				})?;
			} else {
				serialize_struct.skip_field("exampleOfWork")?;
			}
			if !Vec::is_empty(&self.r#expires) {
				serialize_struct.serialize_field("expires", {
					struct SerializeWith<'a>(&'a Vec<ExpiresProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#expires)
				})?;
			} else {
				serialize_struct.skip_field("expires")?;
			}
			if !Vec::is_empty(&self.r#file_format) {
				serialize_struct.serialize_field("fileFormat", {
					struct SerializeWith<'a>(&'a Vec<FileFormatProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#file_format)
				})?;
			} else {
				serialize_struct.skip_field("fileFormat")?;
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
			if !Vec::is_empty(&self.r#genre) {
				serialize_struct.serialize_field("genre", {
					struct SerializeWith<'a>(&'a Vec<GenreProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#genre)
				})?;
			} else {
				serialize_struct.skip_field("genre")?;
			}
			if !Vec::is_empty(&self.r#has_part) {
				serialize_struct.serialize_field("hasPart", {
					struct SerializeWith<'a>(&'a Vec<HasPartProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#has_part)
				})?;
			} else {
				serialize_struct.skip_field("hasPart")?;
			}
			if !Vec::is_empty(&self.r#headline) {
				serialize_struct.serialize_field("headline", {
					struct SerializeWith<'a>(&'a Vec<HeadlineProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#headline)
				})?;
			} else {
				serialize_struct.skip_field("headline")?;
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
			if !Vec::is_empty(&self.r#in_language) {
				serialize_struct.serialize_field("inLanguage", {
					struct SerializeWith<'a>(&'a Vec<InLanguageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#in_language)
				})?;
			} else {
				serialize_struct.skip_field("inLanguage")?;
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
			if !Vec::is_empty(&self.r#interactivity_type) {
				serialize_struct.serialize_field("interactivityType", {
					struct SerializeWith<'a>(&'a Vec<InteractivityTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#interactivity_type)
				})?;
			} else {
				serialize_struct.skip_field("interactivityType")?;
			}
			if !Vec::is_empty(&self.r#interpreted_as_claim) {
				serialize_struct.serialize_field("interpretedAsClaim", {
					struct SerializeWith<'a>(&'a Vec<InterpretedAsClaimProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#interpreted_as_claim)
				})?;
			} else {
				serialize_struct.skip_field("interpretedAsClaim")?;
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
			if !Vec::is_empty(&self.r#is_based_on) {
				serialize_struct.serialize_field("isBasedOn", {
					struct SerializeWith<'a>(&'a Vec<IsBasedOnProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_based_on)
				})?;
			} else {
				serialize_struct.skip_field("isBasedOn")?;
			}
			if !Vec::is_empty(&self.r#is_based_on_url) {
				serialize_struct.serialize_field("isBasedOnUrl", {
					struct SerializeWith<'a>(&'a Vec<IsBasedOnUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_based_on_url)
				})?;
			} else {
				serialize_struct.skip_field("isBasedOnUrl")?;
			}
			if !Vec::is_empty(&self.r#is_family_friendly) {
				serialize_struct.serialize_field("isFamilyFriendly", {
					struct SerializeWith<'a>(&'a Vec<IsFamilyFriendlyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_family_friendly)
				})?;
			} else {
				serialize_struct.skip_field("isFamilyFriendly")?;
			}
			if !Vec::is_empty(&self.r#is_part_of) {
				serialize_struct.serialize_field("isPartOf", {
					struct SerializeWith<'a>(&'a Vec<IsPartOfProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#is_part_of)
				})?;
			} else {
				serialize_struct.skip_field("isPartOf")?;
			}
			if !Vec::is_empty(&self.r#jurisdiction) {
				serialize_struct.serialize_field("jurisdiction", {
					struct SerializeWith<'a>(&'a Vec<JurisdictionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#jurisdiction)
				})?;
			} else {
				serialize_struct.skip_field("jurisdiction")?;
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
			if !Vec::is_empty(&self.r#learning_resource_type) {
				serialize_struct.serialize_field("learningResourceType", {
					struct SerializeWith<'a>(&'a Vec<LearningResourceTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#learning_resource_type)
				})?;
			} else {
				serialize_struct.skip_field("learningResourceType")?;
			}
			if !Vec::is_empty(&self.r#legislation_applies) {
				serialize_struct.serialize_field("legislationApplies", {
					struct SerializeWith<'a>(&'a Vec<LegislationAppliesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_applies)
				})?;
			} else {
				serialize_struct.skip_field("legislationApplies")?;
			}
			if !Vec::is_empty(&self.r#legislation_changes) {
				serialize_struct.serialize_field("legislationChanges", {
					struct SerializeWith<'a>(&'a Vec<LegislationChangesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_changes)
				})?;
			} else {
				serialize_struct.skip_field("legislationChanges")?;
			}
			if !Vec::is_empty(&self.r#legislation_consolidates) {
				serialize_struct.serialize_field("legislationConsolidates", {
					struct SerializeWith<'a>(&'a Vec<LegislationConsolidatesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_consolidates)
				})?;
			} else {
				serialize_struct.skip_field("legislationConsolidates")?;
			}
			if !Vec::is_empty(&self.r#legislation_date) {
				serialize_struct.serialize_field("legislationDate", {
					struct SerializeWith<'a>(&'a Vec<LegislationDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_date)
				})?;
			} else {
				serialize_struct.skip_field("legislationDate")?;
			}
			if !Vec::is_empty(&self.r#legislation_date_version) {
				serialize_struct.serialize_field("legislationDateVersion", {
					struct SerializeWith<'a>(&'a Vec<LegislationDateVersionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_date_version)
				})?;
			} else {
				serialize_struct.skip_field("legislationDateVersion")?;
			}
			if !Vec::is_empty(&self.r#legislation_identifier) {
				serialize_struct.serialize_field("legislationIdentifier", {
					struct SerializeWith<'a>(&'a Vec<LegislationIdentifierProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_identifier)
				})?;
			} else {
				serialize_struct.skip_field("legislationIdentifier")?;
			}
			if !Vec::is_empty(&self.r#legislation_jurisdiction) {
				serialize_struct.serialize_field("legislationJurisdiction", {
					struct SerializeWith<'a>(&'a Vec<LegislationJurisdictionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_jurisdiction)
				})?;
			} else {
				serialize_struct.skip_field("legislationJurisdiction")?;
			}
			if !Vec::is_empty(&self.r#legislation_legal_force) {
				serialize_struct.serialize_field("legislationLegalForce", {
					struct SerializeWith<'a>(&'a Vec<LegislationLegalForceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_legal_force)
				})?;
			} else {
				serialize_struct.skip_field("legislationLegalForce")?;
			}
			if !Vec::is_empty(&self.r#legislation_passed_by) {
				serialize_struct.serialize_field("legislationPassedBy", {
					struct SerializeWith<'a>(&'a Vec<LegislationPassedByProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_passed_by)
				})?;
			} else {
				serialize_struct.skip_field("legislationPassedBy")?;
			}
			if !Vec::is_empty(&self.r#legislation_responsible) {
				serialize_struct.serialize_field("legislationResponsible", {
					struct SerializeWith<'a>(&'a Vec<LegislationResponsibleProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_responsible)
				})?;
			} else {
				serialize_struct.skip_field("legislationResponsible")?;
			}
			if !Vec::is_empty(&self.r#legislation_transposes) {
				serialize_struct.serialize_field("legislationTransposes", {
					struct SerializeWith<'a>(&'a Vec<LegislationTransposesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_transposes)
				})?;
			} else {
				serialize_struct.skip_field("legislationTransposes")?;
			}
			if !Vec::is_empty(&self.r#legislation_type) {
				serialize_struct.serialize_field("legislationType", {
					struct SerializeWith<'a>(&'a Vec<LegislationTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#legislation_type)
				})?;
			} else {
				serialize_struct.skip_field("legislationType")?;
			}
			if !Vec::is_empty(&self.r#license) {
				serialize_struct.serialize_field("license", {
					struct SerializeWith<'a>(&'a Vec<LicenseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#license)
				})?;
			} else {
				serialize_struct.skip_field("license")?;
			}
			if !Vec::is_empty(&self.r#location_created) {
				serialize_struct.serialize_field("locationCreated", {
					struct SerializeWith<'a>(&'a Vec<LocationCreatedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#location_created)
				})?;
			} else {
				serialize_struct.skip_field("locationCreated")?;
			}
			if !Vec::is_empty(&self.r#main_entity) {
				serialize_struct.serialize_field("mainEntity", {
					struct SerializeWith<'a>(&'a Vec<MainEntityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#main_entity)
				})?;
			} else {
				serialize_struct.skip_field("mainEntity")?;
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
			if !Vec::is_empty(&self.r#maintainer) {
				serialize_struct.serialize_field("maintainer", {
					struct SerializeWith<'a>(&'a Vec<MaintainerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#maintainer)
				})?;
			} else {
				serialize_struct.skip_field("maintainer")?;
			}
			if !Vec::is_empty(&self.r#material) {
				serialize_struct.serialize_field("material", {
					struct SerializeWith<'a>(&'a Vec<MaterialProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#material)
				})?;
			} else {
				serialize_struct.skip_field("material")?;
			}
			if !Vec::is_empty(&self.r#material_extent) {
				serialize_struct.serialize_field("materialExtent", {
					struct SerializeWith<'a>(&'a Vec<MaterialExtentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#material_extent)
				})?;
			} else {
				serialize_struct.skip_field("materialExtent")?;
			}
			if !Vec::is_empty(&self.r#mentions) {
				serialize_struct.serialize_field("mentions", {
					struct SerializeWith<'a>(&'a Vec<MentionsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#mentions)
				})?;
			} else {
				serialize_struct.skip_field("mentions")?;
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
			if !Vec::is_empty(&self.r#offers) {
				serialize_struct.serialize_field("offers", {
					struct SerializeWith<'a>(&'a Vec<OffersProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#offers)
				})?;
			} else {
				serialize_struct.skip_field("offers")?;
			}
			if !Vec::is_empty(&self.r#pattern) {
				serialize_struct.serialize_field("pattern", {
					struct SerializeWith<'a>(&'a Vec<PatternProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#pattern)
				})?;
			} else {
				serialize_struct.skip_field("pattern")?;
			}
			if !Vec::is_empty(&self.r#position) {
				serialize_struct.serialize_field("position", {
					struct SerializeWith<'a>(&'a Vec<PositionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#position)
				})?;
			} else {
				serialize_struct.skip_field("position")?;
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
			if !Vec::is_empty(&self.r#producer) {
				serialize_struct.serialize_field("producer", {
					struct SerializeWith<'a>(&'a Vec<ProducerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#producer)
				})?;
			} else {
				serialize_struct.skip_field("producer")?;
			}
			if !Vec::is_empty(&self.r#provider) {
				serialize_struct.serialize_field("provider", {
					struct SerializeWith<'a>(&'a Vec<ProviderProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#provider)
				})?;
			} else {
				serialize_struct.skip_field("provider")?;
			}
			if !Vec::is_empty(&self.r#publication) {
				serialize_struct.serialize_field("publication", {
					struct SerializeWith<'a>(&'a Vec<PublicationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#publication)
				})?;
			} else {
				serialize_struct.skip_field("publication")?;
			}
			if !Vec::is_empty(&self.r#publisher) {
				serialize_struct.serialize_field("publisher", {
					struct SerializeWith<'a>(&'a Vec<PublisherProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#publisher)
				})?;
			} else {
				serialize_struct.skip_field("publisher")?;
			}
			if !Vec::is_empty(&self.r#publisher_imprint) {
				serialize_struct.serialize_field("publisherImprint", {
					struct SerializeWith<'a>(&'a Vec<PublisherImprintProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#publisher_imprint)
				})?;
			} else {
				serialize_struct.skip_field("publisherImprint")?;
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
			if !Vec::is_empty(&self.r#recorded_at) {
				serialize_struct.serialize_field("recordedAt", {
					struct SerializeWith<'a>(&'a Vec<RecordedAtProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#recorded_at)
				})?;
			} else {
				serialize_struct.skip_field("recordedAt")?;
			}
			if !Vec::is_empty(&self.r#released_event) {
				serialize_struct.serialize_field("releasedEvent", {
					struct SerializeWith<'a>(&'a Vec<ReleasedEventProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#released_event)
				})?;
			} else {
				serialize_struct.skip_field("releasedEvent")?;
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
			if !Vec::is_empty(&self.r#schema_version) {
				serialize_struct.serialize_field("schemaVersion", {
					struct SerializeWith<'a>(&'a Vec<SchemaVersionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#schema_version)
				})?;
			} else {
				serialize_struct.skip_field("schemaVersion")?;
			}
			if !Vec::is_empty(&self.r#sd_date_published) {
				serialize_struct.serialize_field("sdDatePublished", {
					struct SerializeWith<'a>(&'a Vec<SdDatePublishedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sd_date_published)
				})?;
			} else {
				serialize_struct.skip_field("sdDatePublished")?;
			}
			if !Vec::is_empty(&self.r#sd_license) {
				serialize_struct.serialize_field("sdLicense", {
					struct SerializeWith<'a>(&'a Vec<SdLicenseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sd_license)
				})?;
			} else {
				serialize_struct.skip_field("sdLicense")?;
			}
			if !Vec::is_empty(&self.r#sd_publisher) {
				serialize_struct.serialize_field("sdPublisher", {
					struct SerializeWith<'a>(&'a Vec<SdPublisherProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sd_publisher)
				})?;
			} else {
				serialize_struct.skip_field("sdPublisher")?;
			}
			if !Vec::is_empty(&self.r#size) {
				serialize_struct.serialize_field("size", {
					struct SerializeWith<'a>(&'a Vec<SizeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#size)
				})?;
			} else {
				serialize_struct.skip_field("size")?;
			}
			if !Vec::is_empty(&self.r#source_organization) {
				serialize_struct.serialize_field("sourceOrganization", {
					struct SerializeWith<'a>(&'a Vec<SourceOrganizationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#source_organization)
				})?;
			} else {
				serialize_struct.skip_field("sourceOrganization")?;
			}
			if !Vec::is_empty(&self.r#spatial) {
				serialize_struct.serialize_field("spatial", {
					struct SerializeWith<'a>(&'a Vec<SpatialProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#spatial)
				})?;
			} else {
				serialize_struct.skip_field("spatial")?;
			}
			if !Vec::is_empty(&self.r#spatial_coverage) {
				serialize_struct.serialize_field("spatialCoverage", {
					struct SerializeWith<'a>(&'a Vec<SpatialCoverageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#spatial_coverage)
				})?;
			} else {
				serialize_struct.skip_field("spatialCoverage")?;
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
			if !Vec::is_empty(&self.r#teaches) {
				serialize_struct.serialize_field("teaches", {
					struct SerializeWith<'a>(&'a Vec<TeachesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#teaches)
				})?;
			} else {
				serialize_struct.skip_field("teaches")?;
			}
			if !Vec::is_empty(&self.r#temporal) {
				serialize_struct.serialize_field("temporal", {
					struct SerializeWith<'a>(&'a Vec<TemporalProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#temporal)
				})?;
			} else {
				serialize_struct.skip_field("temporal")?;
			}
			if !Vec::is_empty(&self.r#temporal_coverage) {
				serialize_struct.serialize_field("temporalCoverage", {
					struct SerializeWith<'a>(&'a Vec<TemporalCoverageProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#temporal_coverage)
				})?;
			} else {
				serialize_struct.skip_field("temporalCoverage")?;
			}
			if !Vec::is_empty(&self.r#text) {
				serialize_struct.serialize_field("text", {
					struct SerializeWith<'a>(&'a Vec<TextProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#text)
				})?;
			} else {
				serialize_struct.skip_field("text")?;
			}
			if !Vec::is_empty(&self.r#thumbnail) {
				serialize_struct.serialize_field("thumbnail", {
					struct SerializeWith<'a>(&'a Vec<ThumbnailProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#thumbnail)
				})?;
			} else {
				serialize_struct.skip_field("thumbnail")?;
			}
			if !Vec::is_empty(&self.r#thumbnail_url) {
				serialize_struct.serialize_field("thumbnailUrl", {
					struct SerializeWith<'a>(&'a Vec<ThumbnailUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#thumbnail_url)
				})?;
			} else {
				serialize_struct.skip_field("thumbnailUrl")?;
			}
			if !Vec::is_empty(&self.r#time_required) {
				serialize_struct.serialize_field("timeRequired", {
					struct SerializeWith<'a>(&'a Vec<TimeRequiredProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#time_required)
				})?;
			} else {
				serialize_struct.skip_field("timeRequired")?;
			}
			if !Vec::is_empty(&self.r#translation_of_work) {
				serialize_struct.serialize_field("translationOfWork", {
					struct SerializeWith<'a>(&'a Vec<TranslationOfWorkProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#translation_of_work)
				})?;
			} else {
				serialize_struct.skip_field("translationOfWork")?;
			}
			if !Vec::is_empty(&self.r#translator) {
				serialize_struct.serialize_field("translator", {
					struct SerializeWith<'a>(&'a Vec<TranslatorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#translator)
				})?;
			} else {
				serialize_struct.skip_field("translator")?;
			}
			if !Vec::is_empty(&self.r#typical_age_range) {
				serialize_struct.serialize_field("typicalAgeRange", {
					struct SerializeWith<'a>(&'a Vec<TypicalAgeRangeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#typical_age_range)
				})?;
			} else {
				serialize_struct.skip_field("typicalAgeRange")?;
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
			if !Vec::is_empty(&self.r#usage_info) {
				serialize_struct.serialize_field("usageInfo", {
					struct SerializeWith<'a>(&'a Vec<UsageInfoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#usage_info)
				})?;
			} else {
				serialize_struct.skip_field("usageInfo")?;
			}
			if !Vec::is_empty(&self.r#version) {
				serialize_struct.serialize_field("version", {
					struct SerializeWith<'a>(&'a Vec<VersionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#version)
				})?;
			} else {
				serialize_struct.skip_field("version")?;
			}
			if !Vec::is_empty(&self.r#video) {
				serialize_struct.serialize_field("video", {
					struct SerializeWith<'a>(&'a Vec<VideoProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#video)
				})?;
			} else {
				serialize_struct.skip_field("video")?;
			}
			if !Vec::is_empty(&self.r#work_example) {
				serialize_struct.serialize_field("workExample", {
					struct SerializeWith<'a>(&'a Vec<WorkExampleProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#work_example)
				})?;
			} else {
				serialize_struct.skip_field("workExample")?;
			}
			if !Vec::is_empty(&self.r#work_translation) {
				serialize_struct.serialize_field("workTranslation", {
					struct SerializeWith<'a>(&'a Vec<WorkTranslationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#work_translation)
				})?;
			} else {
				serialize_struct.skip_field("workTranslation")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Legislation {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				About,
				Abstract,
				AccessMode,
				AccessModeSufficient,
				AccessibilityApi,
				AccessibilityControl,
				AccessibilityFeature,
				AccessibilityHazard,
				AccessibilitySummary,
				AccountablePerson,
				AcquireLicensePage,
				AdditionalType,
				AggregateRating,
				AlternateName,
				AlternativeHeadline,
				ArchivedAt,
				Assesses,
				AssociatedMedia,
				Audience,
				Audio,
				Author,
				Award,
				Awards,
				Character,
				Citation,
				Comment,
				CommentCount,
				ConditionsOfAccess,
				ContentLocation,
				ContentRating,
				ContentReferenceTime,
				Contributor,
				CopyrightHolder,
				CopyrightNotice,
				CopyrightYear,
				Correction,
				CountryOfOrigin,
				CreativeWorkStatus,
				Creator,
				CreditText,
				DateCreated,
				DateModified,
				DatePublished,
				Description,
				DisambiguatingDescription,
				DiscussionUrl,
				EditEidr,
				Editor,
				EducationalAlignment,
				EducationalLevel,
				EducationalUse,
				Encoding,
				EncodingFormat,
				Encodings,
				ExampleOfWork,
				Expires,
				FileFormat,
				Funder,
				Funding,
				Genre,
				HasPart,
				Headline,
				Identifier,
				Image,
				InLanguage,
				InteractionStatistic,
				InteractivityType,
				InterpretedAsClaim,
				IsAccessibleForFree,
				IsBasedOn,
				IsBasedOnUrl,
				IsFamilyFriendly,
				IsPartOf,
				Jurisdiction,
				Keywords,
				LearningResourceType,
				LegislationApplies,
				LegislationChanges,
				LegislationConsolidates,
				LegislationDate,
				LegislationDateVersion,
				LegislationIdentifier,
				LegislationJurisdiction,
				LegislationLegalForce,
				LegislationPassedBy,
				LegislationResponsible,
				LegislationTransposes,
				LegislationType,
				License,
				LocationCreated,
				MainEntity,
				MainEntityOfPage,
				Maintainer,
				Material,
				MaterialExtent,
				Mentions,
				Name,
				Offers,
				Pattern,
				Position,
				PotentialAction,
				Producer,
				Provider,
				Publication,
				Publisher,
				PublisherImprint,
				PublishingPrinciples,
				RecordedAt,
				ReleasedEvent,
				Review,
				Reviews,
				SameAs,
				SchemaVersion,
				SdDatePublished,
				SdLicense,
				SdPublisher,
				Size,
				SourceOrganization,
				Spatial,
				SpatialCoverage,
				Sponsor,
				SubjectOf,
				Teaches,
				Temporal,
				TemporalCoverage,
				Text,
				Thumbnail,
				ThumbnailUrl,
				TimeRequired,
				TranslationOfWork,
				Translator,
				TypicalAgeRange,
				Url,
				UsageInfo,
				Version,
				Video,
				WorkExample,
				WorkTranslation,
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
						"about" => Ok(Field::About),
						"abstract" => Ok(Field::Abstract),
						"accessMode" => Ok(Field::AccessMode),
						"accessModeSufficient" => Ok(Field::AccessModeSufficient),
						"accessibilityAPI" => Ok(Field::AccessibilityApi),
						"accessibilityControl" => Ok(Field::AccessibilityControl),
						"accessibilityFeature" => Ok(Field::AccessibilityFeature),
						"accessibilityHazard" => Ok(Field::AccessibilityHazard),
						"accessibilitySummary" => Ok(Field::AccessibilitySummary),
						"accountablePerson" => Ok(Field::AccountablePerson),
						"acquireLicensePage" => Ok(Field::AcquireLicensePage),
						"additionalType" => Ok(Field::AdditionalType),
						"aggregateRating" => Ok(Field::AggregateRating),
						"alternateName" => Ok(Field::AlternateName),
						"alternativeHeadline" => Ok(Field::AlternativeHeadline),
						"archivedAt" => Ok(Field::ArchivedAt),
						"assesses" => Ok(Field::Assesses),
						"associatedMedia" => Ok(Field::AssociatedMedia),
						"audience" => Ok(Field::Audience),
						"audio" => Ok(Field::Audio),
						"author" => Ok(Field::Author),
						"award" => Ok(Field::Award),
						"awards" => Ok(Field::Awards),
						"character" => Ok(Field::Character),
						"citation" => Ok(Field::Citation),
						"comment" => Ok(Field::Comment),
						"commentCount" => Ok(Field::CommentCount),
						"conditionsOfAccess" => Ok(Field::ConditionsOfAccess),
						"contentLocation" => Ok(Field::ContentLocation),
						"contentRating" => Ok(Field::ContentRating),
						"contentReferenceTime" => Ok(Field::ContentReferenceTime),
						"contributor" => Ok(Field::Contributor),
						"copyrightHolder" => Ok(Field::CopyrightHolder),
						"copyrightNotice" => Ok(Field::CopyrightNotice),
						"copyrightYear" => Ok(Field::CopyrightYear),
						"correction" => Ok(Field::Correction),
						"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						"creativeWorkStatus" => Ok(Field::CreativeWorkStatus),
						"creator" => Ok(Field::Creator),
						"creditText" => Ok(Field::CreditText),
						"dateCreated" => Ok(Field::DateCreated),
						"dateModified" => Ok(Field::DateModified),
						"datePublished" => Ok(Field::DatePublished),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"discussionUrl" => Ok(Field::DiscussionUrl),
						"editEIDR" => Ok(Field::EditEidr),
						"editor" => Ok(Field::Editor),
						"educationalAlignment" => Ok(Field::EducationalAlignment),
						"educationalLevel" => Ok(Field::EducationalLevel),
						"educationalUse" => Ok(Field::EducationalUse),
						"encoding" => Ok(Field::Encoding),
						"encodingFormat" => Ok(Field::EncodingFormat),
						"encodings" => Ok(Field::Encodings),
						"exampleOfWork" => Ok(Field::ExampleOfWork),
						"expires" => Ok(Field::Expires),
						"fileFormat" => Ok(Field::FileFormat),
						"funder" => Ok(Field::Funder),
						"funding" => Ok(Field::Funding),
						"genre" => Ok(Field::Genre),
						"hasPart" => Ok(Field::HasPart),
						"headline" => Ok(Field::Headline),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"inLanguage" => Ok(Field::InLanguage),
						"interactionStatistic" => Ok(Field::InteractionStatistic),
						"interactivityType" => Ok(Field::InteractivityType),
						"interpretedAsClaim" => Ok(Field::InterpretedAsClaim),
						"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						"isBasedOn" => Ok(Field::IsBasedOn),
						"isBasedOnUrl" => Ok(Field::IsBasedOnUrl),
						"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						"isPartOf" => Ok(Field::IsPartOf),
						"jurisdiction" => Ok(Field::Jurisdiction),
						"keywords" => Ok(Field::Keywords),
						"learningResourceType" => Ok(Field::LearningResourceType),
						"legislationApplies" => Ok(Field::LegislationApplies),
						"legislationChanges" => Ok(Field::LegislationChanges),
						"legislationConsolidates" => Ok(Field::LegislationConsolidates),
						"legislationDate" => Ok(Field::LegislationDate),
						"legislationDateVersion" => Ok(Field::LegislationDateVersion),
						"legislationIdentifier" => Ok(Field::LegislationIdentifier),
						"legislationJurisdiction" => Ok(Field::LegislationJurisdiction),
						"legislationLegalForce" => Ok(Field::LegislationLegalForce),
						"legislationPassedBy" => Ok(Field::LegislationPassedBy),
						"legislationResponsible" => Ok(Field::LegislationResponsible),
						"legislationTransposes" => Ok(Field::LegislationTransposes),
						"legislationType" => Ok(Field::LegislationType),
						"license" => Ok(Field::License),
						"locationCreated" => Ok(Field::LocationCreated),
						"mainEntity" => Ok(Field::MainEntity),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"maintainer" => Ok(Field::Maintainer),
						"material" => Ok(Field::Material),
						"materialExtent" => Ok(Field::MaterialExtent),
						"mentions" => Ok(Field::Mentions),
						"name" => Ok(Field::Name),
						"offers" => Ok(Field::Offers),
						"pattern" => Ok(Field::Pattern),
						"position" => Ok(Field::Position),
						"potentialAction" => Ok(Field::PotentialAction),
						"producer" => Ok(Field::Producer),
						"provider" => Ok(Field::Provider),
						"publication" => Ok(Field::Publication),
						"publisher" => Ok(Field::Publisher),
						"publisherImprint" => Ok(Field::PublisherImprint),
						"publishingPrinciples" => Ok(Field::PublishingPrinciples),
						"recordedAt" => Ok(Field::RecordedAt),
						"releasedEvent" => Ok(Field::ReleasedEvent),
						"review" => Ok(Field::Review),
						"reviews" => Ok(Field::Reviews),
						"sameAs" => Ok(Field::SameAs),
						"schemaVersion" => Ok(Field::SchemaVersion),
						"sdDatePublished" => Ok(Field::SdDatePublished),
						"sdLicense" => Ok(Field::SdLicense),
						"sdPublisher" => Ok(Field::SdPublisher),
						"size" => Ok(Field::Size),
						"sourceOrganization" => Ok(Field::SourceOrganization),
						"spatial" => Ok(Field::Spatial),
						"spatialCoverage" => Ok(Field::SpatialCoverage),
						"sponsor" => Ok(Field::Sponsor),
						"subjectOf" => Ok(Field::SubjectOf),
						"teaches" => Ok(Field::Teaches),
						"temporal" => Ok(Field::Temporal),
						"temporalCoverage" => Ok(Field::TemporalCoverage),
						"text" => Ok(Field::Text),
						"thumbnail" => Ok(Field::Thumbnail),
						"thumbnailUrl" => Ok(Field::ThumbnailUrl),
						"timeRequired" => Ok(Field::TimeRequired),
						"translationOfWork" => Ok(Field::TranslationOfWork),
						"translator" => Ok(Field::Translator),
						"typicalAgeRange" => Ok(Field::TypicalAgeRange),
						"url" => Ok(Field::Url),
						"usageInfo" => Ok(Field::UsageInfo),
						"version" => Ok(Field::Version),
						"video" => Ok(Field::Video),
						"workExample" => Ok(Field::WorkExample),
						"workTranslation" => Ok(Field::WorkTranslation),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"about" => Ok(Field::About),
						b"abstract" => Ok(Field::Abstract),
						b"accessMode" => Ok(Field::AccessMode),
						b"accessModeSufficient" => Ok(Field::AccessModeSufficient),
						b"accessibilityAPI" => Ok(Field::AccessibilityApi),
						b"accessibilityControl" => Ok(Field::AccessibilityControl),
						b"accessibilityFeature" => Ok(Field::AccessibilityFeature),
						b"accessibilityHazard" => Ok(Field::AccessibilityHazard),
						b"accessibilitySummary" => Ok(Field::AccessibilitySummary),
						b"accountablePerson" => Ok(Field::AccountablePerson),
						b"acquireLicensePage" => Ok(Field::AcquireLicensePage),
						b"additionalType" => Ok(Field::AdditionalType),
						b"aggregateRating" => Ok(Field::AggregateRating),
						b"alternateName" => Ok(Field::AlternateName),
						b"alternativeHeadline" => Ok(Field::AlternativeHeadline),
						b"archivedAt" => Ok(Field::ArchivedAt),
						b"assesses" => Ok(Field::Assesses),
						b"associatedMedia" => Ok(Field::AssociatedMedia),
						b"audience" => Ok(Field::Audience),
						b"audio" => Ok(Field::Audio),
						b"author" => Ok(Field::Author),
						b"award" => Ok(Field::Award),
						b"awards" => Ok(Field::Awards),
						b"character" => Ok(Field::Character),
						b"citation" => Ok(Field::Citation),
						b"comment" => Ok(Field::Comment),
						b"commentCount" => Ok(Field::CommentCount),
						b"conditionsOfAccess" => Ok(Field::ConditionsOfAccess),
						b"contentLocation" => Ok(Field::ContentLocation),
						b"contentRating" => Ok(Field::ContentRating),
						b"contentReferenceTime" => Ok(Field::ContentReferenceTime),
						b"contributor" => Ok(Field::Contributor),
						b"copyrightHolder" => Ok(Field::CopyrightHolder),
						b"copyrightNotice" => Ok(Field::CopyrightNotice),
						b"copyrightYear" => Ok(Field::CopyrightYear),
						b"correction" => Ok(Field::Correction),
						b"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						b"creativeWorkStatus" => Ok(Field::CreativeWorkStatus),
						b"creator" => Ok(Field::Creator),
						b"creditText" => Ok(Field::CreditText),
						b"dateCreated" => Ok(Field::DateCreated),
						b"dateModified" => Ok(Field::DateModified),
						b"datePublished" => Ok(Field::DatePublished),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"discussionUrl" => Ok(Field::DiscussionUrl),
						b"editEIDR" => Ok(Field::EditEidr),
						b"editor" => Ok(Field::Editor),
						b"educationalAlignment" => Ok(Field::EducationalAlignment),
						b"educationalLevel" => Ok(Field::EducationalLevel),
						b"educationalUse" => Ok(Field::EducationalUse),
						b"encoding" => Ok(Field::Encoding),
						b"encodingFormat" => Ok(Field::EncodingFormat),
						b"encodings" => Ok(Field::Encodings),
						b"exampleOfWork" => Ok(Field::ExampleOfWork),
						b"expires" => Ok(Field::Expires),
						b"fileFormat" => Ok(Field::FileFormat),
						b"funder" => Ok(Field::Funder),
						b"funding" => Ok(Field::Funding),
						b"genre" => Ok(Field::Genre),
						b"hasPart" => Ok(Field::HasPart),
						b"headline" => Ok(Field::Headline),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"inLanguage" => Ok(Field::InLanguage),
						b"interactionStatistic" => Ok(Field::InteractionStatistic),
						b"interactivityType" => Ok(Field::InteractivityType),
						b"interpretedAsClaim" => Ok(Field::InterpretedAsClaim),
						b"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						b"isBasedOn" => Ok(Field::IsBasedOn),
						b"isBasedOnUrl" => Ok(Field::IsBasedOnUrl),
						b"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						b"isPartOf" => Ok(Field::IsPartOf),
						b"jurisdiction" => Ok(Field::Jurisdiction),
						b"keywords" => Ok(Field::Keywords),
						b"learningResourceType" => Ok(Field::LearningResourceType),
						b"legislationApplies" => Ok(Field::LegislationApplies),
						b"legislationChanges" => Ok(Field::LegislationChanges),
						b"legislationConsolidates" => Ok(Field::LegislationConsolidates),
						b"legislationDate" => Ok(Field::LegislationDate),
						b"legislationDateVersion" => Ok(Field::LegislationDateVersion),
						b"legislationIdentifier" => Ok(Field::LegislationIdentifier),
						b"legislationJurisdiction" => Ok(Field::LegislationJurisdiction),
						b"legislationLegalForce" => Ok(Field::LegislationLegalForce),
						b"legislationPassedBy" => Ok(Field::LegislationPassedBy),
						b"legislationResponsible" => Ok(Field::LegislationResponsible),
						b"legislationTransposes" => Ok(Field::LegislationTransposes),
						b"legislationType" => Ok(Field::LegislationType),
						b"license" => Ok(Field::License),
						b"locationCreated" => Ok(Field::LocationCreated),
						b"mainEntity" => Ok(Field::MainEntity),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"maintainer" => Ok(Field::Maintainer),
						b"material" => Ok(Field::Material),
						b"materialExtent" => Ok(Field::MaterialExtent),
						b"mentions" => Ok(Field::Mentions),
						b"name" => Ok(Field::Name),
						b"offers" => Ok(Field::Offers),
						b"pattern" => Ok(Field::Pattern),
						b"position" => Ok(Field::Position),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"producer" => Ok(Field::Producer),
						b"provider" => Ok(Field::Provider),
						b"publication" => Ok(Field::Publication),
						b"publisher" => Ok(Field::Publisher),
						b"publisherImprint" => Ok(Field::PublisherImprint),
						b"publishingPrinciples" => Ok(Field::PublishingPrinciples),
						b"recordedAt" => Ok(Field::RecordedAt),
						b"releasedEvent" => Ok(Field::ReleasedEvent),
						b"review" => Ok(Field::Review),
						b"reviews" => Ok(Field::Reviews),
						b"sameAs" => Ok(Field::SameAs),
						b"schemaVersion" => Ok(Field::SchemaVersion),
						b"sdDatePublished" => Ok(Field::SdDatePublished),
						b"sdLicense" => Ok(Field::SdLicense),
						b"sdPublisher" => Ok(Field::SdPublisher),
						b"size" => Ok(Field::Size),
						b"sourceOrganization" => Ok(Field::SourceOrganization),
						b"spatial" => Ok(Field::Spatial),
						b"spatialCoverage" => Ok(Field::SpatialCoverage),
						b"sponsor" => Ok(Field::Sponsor),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"teaches" => Ok(Field::Teaches),
						b"temporal" => Ok(Field::Temporal),
						b"temporalCoverage" => Ok(Field::TemporalCoverage),
						b"text" => Ok(Field::Text),
						b"thumbnail" => Ok(Field::Thumbnail),
						b"thumbnailUrl" => Ok(Field::ThumbnailUrl),
						b"timeRequired" => Ok(Field::TimeRequired),
						b"translationOfWork" => Ok(Field::TranslationOfWork),
						b"translator" => Ok(Field::Translator),
						b"typicalAgeRange" => Ok(Field::TypicalAgeRange),
						b"url" => Ok(Field::Url),
						b"usageInfo" => Ok(Field::UsageInfo),
						b"version" => Ok(Field::Version),
						b"video" => Ok(Field::Video),
						b"workExample" => Ok(Field::WorkExample),
						b"workTranslation" => Ok(Field::WorkTranslation),
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
				type Value = Legislation;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Legislation")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#about_property = None;
					let mut r#abstract_property = None;
					let mut r#access_mode_property = None;
					let mut r#access_mode_sufficient_property = None;
					let mut r#accessibility_api_property = None;
					let mut r#accessibility_control_property = None;
					let mut r#accessibility_feature_property = None;
					let mut r#accessibility_hazard_property = None;
					let mut r#accessibility_summary_property = None;
					let mut r#accountable_person_property = None;
					let mut r#acquire_license_page_property = None;
					let mut r#additional_type_property = None;
					let mut r#aggregate_rating_property = None;
					let mut r#alternate_name_property = None;
					let mut r#alternative_headline_property = None;
					let mut r#archived_at_property = None;
					let mut r#assesses_property = None;
					let mut r#associated_media_property = None;
					let mut r#audience_property = None;
					let mut r#audio_property = None;
					let mut r#author_property = None;
					let mut r#award_property = None;
					let mut r#awards_property = None;
					let mut r#character_property = None;
					let mut r#citation_property = None;
					let mut r#comment_property = None;
					let mut r#comment_count_property = None;
					let mut r#conditions_of_access_property = None;
					let mut r#content_location_property = None;
					let mut r#content_rating_property = None;
					let mut r#content_reference_time_property = None;
					let mut r#contributor_property = None;
					let mut r#copyright_holder_property = None;
					let mut r#copyright_notice_property = None;
					let mut r#copyright_year_property = None;
					let mut r#correction_property = None;
					let mut r#country_of_origin_property = None;
					let mut r#creative_work_status_property = None;
					let mut r#creator_property = None;
					let mut r#credit_text_property = None;
					let mut r#date_created_property = None;
					let mut r#date_modified_property = None;
					let mut r#date_published_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#discussion_url_property = None;
					let mut r#edit_eidr_property = None;
					let mut r#editor_property = None;
					let mut r#educational_alignment_property = None;
					let mut r#educational_level_property = None;
					let mut r#educational_use_property = None;
					let mut r#encoding_property = None;
					let mut r#encoding_format_property = None;
					let mut r#encodings_property = None;
					let mut r#example_of_work_property = None;
					let mut r#expires_property = None;
					let mut r#file_format_property = None;
					let mut r#funder_property = None;
					let mut r#funding_property = None;
					let mut r#genre_property = None;
					let mut r#has_part_property = None;
					let mut r#headline_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#in_language_property = None;
					let mut r#interaction_statistic_property = None;
					let mut r#interactivity_type_property = None;
					let mut r#interpreted_as_claim_property = None;
					let mut r#is_accessible_for_free_property = None;
					let mut r#is_based_on_property = None;
					let mut r#is_based_on_url_property = None;
					let mut r#is_family_friendly_property = None;
					let mut r#is_part_of_property = None;
					let mut r#jurisdiction_property = None;
					let mut r#keywords_property = None;
					let mut r#learning_resource_type_property = None;
					let mut r#legislation_applies_property = None;
					let mut r#legislation_changes_property = None;
					let mut r#legislation_consolidates_property = None;
					let mut r#legislation_date_property = None;
					let mut r#legislation_date_version_property = None;
					let mut r#legislation_identifier_property = None;
					let mut r#legislation_jurisdiction_property = None;
					let mut r#legislation_legal_force_property = None;
					let mut r#legislation_passed_by_property = None;
					let mut r#legislation_responsible_property = None;
					let mut r#legislation_transposes_property = None;
					let mut r#legislation_type_property = None;
					let mut r#license_property = None;
					let mut r#location_created_property = None;
					let mut r#main_entity_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#maintainer_property = None;
					let mut r#material_property = None;
					let mut r#material_extent_property = None;
					let mut r#mentions_property = None;
					let mut r#name_property = None;
					let mut r#offers_property = None;
					let mut r#pattern_property = None;
					let mut r#position_property = None;
					let mut r#potential_action_property = None;
					let mut r#producer_property = None;
					let mut r#provider_property = None;
					let mut r#publication_property = None;
					let mut r#publisher_property = None;
					let mut r#publisher_imprint_property = None;
					let mut r#publishing_principles_property = None;
					let mut r#recorded_at_property = None;
					let mut r#released_event_property = None;
					let mut r#review_property = None;
					let mut r#reviews_property = None;
					let mut r#same_as_property = None;
					let mut r#schema_version_property = None;
					let mut r#sd_date_published_property = None;
					let mut r#sd_license_property = None;
					let mut r#sd_publisher_property = None;
					let mut r#size_property = None;
					let mut r#source_organization_property = None;
					let mut r#spatial_property = None;
					let mut r#spatial_coverage_property = None;
					let mut r#sponsor_property = None;
					let mut r#subject_of_property = None;
					let mut r#teaches_property = None;
					let mut r#temporal_property = None;
					let mut r#temporal_coverage_property = None;
					let mut r#text_property = None;
					let mut r#thumbnail_property = None;
					let mut r#thumbnail_url_property = None;
					let mut r#time_required_property = None;
					let mut r#translation_of_work_property = None;
					let mut r#translator_property = None;
					let mut r#typical_age_range_property = None;
					let mut r#url_property = None;
					let mut r#usage_info_property = None;
					let mut r#version_property = None;
					let mut r#video_property = None;
					let mut r#work_example_property = None;
					let mut r#work_translation_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							Field::About => {
								if r#about_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("about"));
								}
								r#about_property = Some({
									struct DeserializeWith(Vec<AboutProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Abstract => {
								if r#abstract_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"abstract",
									));
								}
								r#abstract_property = Some({
									struct DeserializeWith(Vec<AbstractProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AccessMode => {
								if r#access_mode_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accessMode",
									));
								}
								r#access_mode_property = Some({
									struct DeserializeWith(Vec<AccessModeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AccessModeSufficient => {
								if r#access_mode_sufficient_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accessModeSufficient",
									));
								}
								r#access_mode_sufficient_property = Some({
									struct DeserializeWith(Vec<AccessModeSufficientProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AccessibilityApi => {
								if r#accessibility_api_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accessibilityAPI",
									));
								}
								r#accessibility_api_property = Some({
									struct DeserializeWith(Vec<AccessibilityApiProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AccessibilityControl => {
								if r#accessibility_control_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accessibilityControl",
									));
								}
								r#accessibility_control_property = Some({
									struct DeserializeWith(Vec<AccessibilityControlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AccessibilityFeature => {
								if r#accessibility_feature_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accessibilityFeature",
									));
								}
								r#accessibility_feature_property = Some({
									struct DeserializeWith(Vec<AccessibilityFeatureProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AccessibilityHazard => {
								if r#accessibility_hazard_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accessibilityHazard",
									));
								}
								r#accessibility_hazard_property = Some({
									struct DeserializeWith(Vec<AccessibilityHazardProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AccessibilitySummary => {
								if r#accessibility_summary_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accessibilitySummary",
									));
								}
								r#accessibility_summary_property = Some({
									struct DeserializeWith(Vec<AccessibilitySummaryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AccountablePerson => {
								if r#accountable_person_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"accountablePerson",
									));
								}
								r#accountable_person_property = Some({
									struct DeserializeWith(Vec<AccountablePersonProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AcquireLicensePage => {
								if r#acquire_license_page_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"acquireLicensePage",
									));
								}
								r#acquire_license_page_property = Some({
									struct DeserializeWith(Vec<AcquireLicensePageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::AlternativeHeadline => {
								if r#alternative_headline_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"alternativeHeadline",
									));
								}
								r#alternative_headline_property = Some({
									struct DeserializeWith(Vec<AlternativeHeadlineProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ArchivedAt => {
								if r#archived_at_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"archivedAt",
									));
								}
								r#archived_at_property = Some({
									struct DeserializeWith(Vec<ArchivedAtProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Assesses => {
								if r#assesses_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"assesses",
									));
								}
								r#assesses_property = Some({
									struct DeserializeWith(Vec<AssessesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::AssociatedMedia => {
								if r#associated_media_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"associatedMedia",
									));
								}
								r#associated_media_property = Some({
									struct DeserializeWith(Vec<AssociatedMediaProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Audience => {
								if r#audience_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"audience",
									));
								}
								r#audience_property = Some({
									struct DeserializeWith(Vec<AudienceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Audio => {
								if r#audio_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("audio"));
								}
								r#audio_property = Some({
									struct DeserializeWith(Vec<AudioProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Author => {
								if r#author_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("author"));
								}
								r#author_property = Some({
									struct DeserializeWith(Vec<AuthorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Character => {
								if r#character_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"character",
									));
								}
								r#character_property = Some({
									struct DeserializeWith(Vec<CharacterProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Citation => {
								if r#citation_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"citation",
									));
								}
								r#citation_property = Some({
									struct DeserializeWith(Vec<CitationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Comment => {
								if r#comment_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"comment",
									));
								}
								r#comment_property = Some({
									struct DeserializeWith(Vec<CommentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CommentCount => {
								if r#comment_count_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"commentCount",
									));
								}
								r#comment_count_property = Some({
									struct DeserializeWith(Vec<CommentCountProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ConditionsOfAccess => {
								if r#conditions_of_access_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"conditionsOfAccess",
									));
								}
								r#conditions_of_access_property = Some({
									struct DeserializeWith(Vec<ConditionsOfAccessProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ContentLocation => {
								if r#content_location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contentLocation",
									));
								}
								r#content_location_property = Some({
									struct DeserializeWith(Vec<ContentLocationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ContentRating => {
								if r#content_rating_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contentRating",
									));
								}
								r#content_rating_property = Some({
									struct DeserializeWith(Vec<ContentRatingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ContentReferenceTime => {
								if r#content_reference_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contentReferenceTime",
									));
								}
								r#content_reference_time_property = Some({
									struct DeserializeWith(Vec<ContentReferenceTimeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Contributor => {
								if r#contributor_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contributor",
									));
								}
								r#contributor_property = Some({
									struct DeserializeWith(Vec<ContributorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CopyrightHolder => {
								if r#copyright_holder_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"copyrightHolder",
									));
								}
								r#copyright_holder_property = Some({
									struct DeserializeWith(Vec<CopyrightHolderProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CopyrightNotice => {
								if r#copyright_notice_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"copyrightNotice",
									));
								}
								r#copyright_notice_property = Some({
									struct DeserializeWith(Vec<CopyrightNoticeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CopyrightYear => {
								if r#copyright_year_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"copyrightYear",
									));
								}
								r#copyright_year_property = Some({
									struct DeserializeWith(Vec<CopyrightYearProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Correction => {
								if r#correction_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"correction",
									));
								}
								r#correction_property = Some({
									struct DeserializeWith(Vec<CorrectionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CountryOfOrigin => {
								if r#country_of_origin_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"countryOfOrigin",
									));
								}
								r#country_of_origin_property = Some({
									struct DeserializeWith(Vec<CountryOfOriginProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CreativeWorkStatus => {
								if r#creative_work_status_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"creativeWorkStatus",
									));
								}
								r#creative_work_status_property = Some({
									struct DeserializeWith(Vec<CreativeWorkStatusProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Creator => {
								if r#creator_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"creator",
									));
								}
								r#creator_property = Some({
									struct DeserializeWith(Vec<CreatorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CreditText => {
								if r#credit_text_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"creditText",
									));
								}
								r#credit_text_property = Some({
									struct DeserializeWith(Vec<CreditTextProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::DateCreated => {
								if r#date_created_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"dateCreated",
									));
								}
								r#date_created_property = Some({
									struct DeserializeWith(Vec<DateCreatedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::DateModified => {
								if r#date_modified_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"dateModified",
									));
								}
								r#date_modified_property = Some({
									struct DeserializeWith(Vec<DateModifiedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::DatePublished => {
								if r#date_published_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"datePublished",
									));
								}
								r#date_published_property = Some({
									struct DeserializeWith(Vec<DatePublishedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::DiscussionUrl => {
								if r#discussion_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"discussionUrl",
									));
								}
								r#discussion_url_property = Some({
									struct DeserializeWith(Vec<DiscussionUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::EditEidr => {
								if r#edit_eidr_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"editEIDR",
									));
								}
								r#edit_eidr_property = Some({
									struct DeserializeWith(Vec<EditEidrProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Editor => {
								if r#editor_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("editor"));
								}
								r#editor_property = Some({
									struct DeserializeWith(Vec<EditorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::EducationalAlignment => {
								if r#educational_alignment_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"educationalAlignment",
									));
								}
								r#educational_alignment_property = Some({
									struct DeserializeWith(Vec<EducationalAlignmentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::EducationalLevel => {
								if r#educational_level_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"educationalLevel",
									));
								}
								r#educational_level_property = Some({
									struct DeserializeWith(Vec<EducationalLevelProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::EducationalUse => {
								if r#educational_use_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"educationalUse",
									));
								}
								r#educational_use_property = Some({
									struct DeserializeWith(Vec<EducationalUseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Encoding => {
								if r#encoding_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"encoding",
									));
								}
								r#encoding_property = Some({
									struct DeserializeWith(Vec<EncodingProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::EncodingFormat => {
								if r#encoding_format_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"encodingFormat",
									));
								}
								r#encoding_format_property = Some({
									struct DeserializeWith(Vec<EncodingFormatProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Encodings => {
								if r#encodings_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"encodings",
									));
								}
								r#encodings_property = Some({
									struct DeserializeWith(Vec<EncodingsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ExampleOfWork => {
								if r#example_of_work_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"exampleOfWork",
									));
								}
								r#example_of_work_property = Some({
									struct DeserializeWith(Vec<ExampleOfWorkProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Expires => {
								if r#expires_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"expires",
									));
								}
								r#expires_property = Some({
									struct DeserializeWith(Vec<ExpiresProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::FileFormat => {
								if r#file_format_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"fileFormat",
									));
								}
								r#file_format_property = Some({
									struct DeserializeWith(Vec<FileFormatProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Genre => {
								if r#genre_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("genre"));
								}
								r#genre_property = Some({
									struct DeserializeWith(Vec<GenreProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::HasPart => {
								if r#has_part_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasPart",
									));
								}
								r#has_part_property = Some({
									struct DeserializeWith(Vec<HasPartProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Headline => {
								if r#headline_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"headline",
									));
								}
								r#headline_property = Some({
									struct DeserializeWith(Vec<HeadlineProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::InLanguage => {
								if r#in_language_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"inLanguage",
									));
								}
								r#in_language_property = Some({
									struct DeserializeWith(Vec<InLanguageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::InteractivityType => {
								if r#interactivity_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"interactivityType",
									));
								}
								r#interactivity_type_property = Some({
									struct DeserializeWith(Vec<InteractivityTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::InterpretedAsClaim => {
								if r#interpreted_as_claim_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"interpretedAsClaim",
									));
								}
								r#interpreted_as_claim_property = Some({
									struct DeserializeWith(Vec<InterpretedAsClaimProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::IsBasedOn => {
								if r#is_based_on_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isBasedOn",
									));
								}
								r#is_based_on_property = Some({
									struct DeserializeWith(Vec<IsBasedOnProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsBasedOnUrl => {
								if r#is_based_on_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isBasedOnUrl",
									));
								}
								r#is_based_on_url_property = Some({
									struct DeserializeWith(Vec<IsBasedOnUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsFamilyFriendly => {
								if r#is_family_friendly_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isFamilyFriendly",
									));
								}
								r#is_family_friendly_property = Some({
									struct DeserializeWith(Vec<IsFamilyFriendlyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::IsPartOf => {
								if r#is_part_of_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"isPartOf",
									));
								}
								r#is_part_of_property = Some({
									struct DeserializeWith(Vec<IsPartOfProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Jurisdiction => {
								if r#jurisdiction_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"jurisdiction",
									));
								}
								r#jurisdiction_property = Some({
									struct DeserializeWith(Vec<JurisdictionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::LearningResourceType => {
								if r#learning_resource_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"learningResourceType",
									));
								}
								r#learning_resource_type_property = Some({
									struct DeserializeWith(Vec<LearningResourceTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationApplies => {
								if r#legislation_applies_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationApplies",
									));
								}
								r#legislation_applies_property = Some({
									struct DeserializeWith(Vec<LegislationAppliesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationChanges => {
								if r#legislation_changes_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationChanges",
									));
								}
								r#legislation_changes_property = Some({
									struct DeserializeWith(Vec<LegislationChangesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationConsolidates => {
								if r#legislation_consolidates_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationConsolidates",
									));
								}
								r#legislation_consolidates_property = Some({
									struct DeserializeWith(Vec<LegislationConsolidatesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationDate => {
								if r#legislation_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationDate",
									));
								}
								r#legislation_date_property = Some({
									struct DeserializeWith(Vec<LegislationDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationDateVersion => {
								if r#legislation_date_version_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationDateVersion",
									));
								}
								r#legislation_date_version_property = Some({
									struct DeserializeWith(Vec<LegislationDateVersionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationIdentifier => {
								if r#legislation_identifier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationIdentifier",
									));
								}
								r#legislation_identifier_property = Some({
									struct DeserializeWith(Vec<LegislationIdentifierProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationJurisdiction => {
								if r#legislation_jurisdiction_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationJurisdiction",
									));
								}
								r#legislation_jurisdiction_property = Some({
									struct DeserializeWith(Vec<LegislationJurisdictionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationLegalForce => {
								if r#legislation_legal_force_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationLegalForce",
									));
								}
								r#legislation_legal_force_property = Some({
									struct DeserializeWith(Vec<LegislationLegalForceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationPassedBy => {
								if r#legislation_passed_by_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationPassedBy",
									));
								}
								r#legislation_passed_by_property = Some({
									struct DeserializeWith(Vec<LegislationPassedByProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationResponsible => {
								if r#legislation_responsible_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationResponsible",
									));
								}
								r#legislation_responsible_property = Some({
									struct DeserializeWith(Vec<LegislationResponsibleProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationTransposes => {
								if r#legislation_transposes_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationTransposes",
									));
								}
								r#legislation_transposes_property = Some({
									struct DeserializeWith(Vec<LegislationTransposesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LegislationType => {
								if r#legislation_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"legislationType",
									));
								}
								r#legislation_type_property = Some({
									struct DeserializeWith(Vec<LegislationTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::License => {
								if r#license_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"license",
									));
								}
								r#license_property = Some({
									struct DeserializeWith(Vec<LicenseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::LocationCreated => {
								if r#location_created_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"locationCreated",
									));
								}
								r#location_created_property = Some({
									struct DeserializeWith(Vec<LocationCreatedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::MainEntity => {
								if r#main_entity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mainEntity",
									));
								}
								r#main_entity_property = Some({
									struct DeserializeWith(Vec<MainEntityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Maintainer => {
								if r#maintainer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maintainer",
									));
								}
								r#maintainer_property = Some({
									struct DeserializeWith(Vec<MaintainerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Material => {
								if r#material_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"material",
									));
								}
								r#material_property = Some({
									struct DeserializeWith(Vec<MaterialProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::MaterialExtent => {
								if r#material_extent_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"materialExtent",
									));
								}
								r#material_extent_property = Some({
									struct DeserializeWith(Vec<MaterialExtentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Mentions => {
								if r#mentions_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mentions",
									));
								}
								r#mentions_property = Some({
									struct DeserializeWith(Vec<MentionsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Offers => {
								if r#offers_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("offers"));
								}
								r#offers_property = Some({
									struct DeserializeWith(Vec<OffersProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Pattern => {
								if r#pattern_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"pattern",
									));
								}
								r#pattern_property = Some({
									struct DeserializeWith(Vec<PatternProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Position => {
								if r#position_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"position",
									));
								}
								r#position_property = Some({
									struct DeserializeWith(Vec<PositionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Producer => {
								if r#producer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"producer",
									));
								}
								r#producer_property = Some({
									struct DeserializeWith(Vec<ProducerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Provider => {
								if r#provider_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"provider",
									));
								}
								r#provider_property = Some({
									struct DeserializeWith(Vec<ProviderProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Publication => {
								if r#publication_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"publication",
									));
								}
								r#publication_property = Some({
									struct DeserializeWith(Vec<PublicationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Publisher => {
								if r#publisher_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"publisher",
									));
								}
								r#publisher_property = Some({
									struct DeserializeWith(Vec<PublisherProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PublisherImprint => {
								if r#publisher_imprint_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"publisherImprint",
									));
								}
								r#publisher_imprint_property = Some({
									struct DeserializeWith(Vec<PublisherImprintProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::RecordedAt => {
								if r#recorded_at_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"recordedAt",
									));
								}
								r#recorded_at_property = Some({
									struct DeserializeWith(Vec<RecordedAtProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ReleasedEvent => {
								if r#released_event_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"releasedEvent",
									));
								}
								r#released_event_property = Some({
									struct DeserializeWith(Vec<ReleasedEventProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::SchemaVersion => {
								if r#schema_version_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"schemaVersion",
									));
								}
								r#schema_version_property = Some({
									struct DeserializeWith(Vec<SchemaVersionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SdDatePublished => {
								if r#sd_date_published_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sdDatePublished",
									));
								}
								r#sd_date_published_property = Some({
									struct DeserializeWith(Vec<SdDatePublishedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SdLicense => {
								if r#sd_license_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sdLicense",
									));
								}
								r#sd_license_property = Some({
									struct DeserializeWith(Vec<SdLicenseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SdPublisher => {
								if r#sd_publisher_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sdPublisher",
									));
								}
								r#sd_publisher_property = Some({
									struct DeserializeWith(Vec<SdPublisherProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Size => {
								if r#size_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("size"));
								}
								r#size_property = Some({
									struct DeserializeWith(Vec<SizeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SourceOrganization => {
								if r#source_organization_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sourceOrganization",
									));
								}
								r#source_organization_property = Some({
									struct DeserializeWith(Vec<SourceOrganizationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Spatial => {
								if r#spatial_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"spatial",
									));
								}
								r#spatial_property = Some({
									struct DeserializeWith(Vec<SpatialProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::SpatialCoverage => {
								if r#spatial_coverage_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"spatialCoverage",
									));
								}
								r#spatial_coverage_property = Some({
									struct DeserializeWith(Vec<SpatialCoverageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::Teaches => {
								if r#teaches_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"teaches",
									));
								}
								r#teaches_property = Some({
									struct DeserializeWith(Vec<TeachesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Temporal => {
								if r#temporal_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"temporal",
									));
								}
								r#temporal_property = Some({
									struct DeserializeWith(Vec<TemporalProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::TemporalCoverage => {
								if r#temporal_coverage_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"temporalCoverage",
									));
								}
								r#temporal_coverage_property = Some({
									struct DeserializeWith(Vec<TemporalCoverageProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Text => {
								if r#text_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("text"));
								}
								r#text_property = Some({
									struct DeserializeWith(Vec<TextProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Thumbnail => {
								if r#thumbnail_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"thumbnail",
									));
								}
								r#thumbnail_property = Some({
									struct DeserializeWith(Vec<ThumbnailProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::ThumbnailUrl => {
								if r#thumbnail_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"thumbnailUrl",
									));
								}
								r#thumbnail_url_property = Some({
									struct DeserializeWith(Vec<ThumbnailUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::TimeRequired => {
								if r#time_required_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"timeRequired",
									));
								}
								r#time_required_property = Some({
									struct DeserializeWith(Vec<TimeRequiredProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::TranslationOfWork => {
								if r#translation_of_work_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"translationOfWork",
									));
								}
								r#translation_of_work_property = Some({
									struct DeserializeWith(Vec<TranslationOfWorkProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Translator => {
								if r#translator_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"translator",
									));
								}
								r#translator_property = Some({
									struct DeserializeWith(Vec<TranslatorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::TypicalAgeRange => {
								if r#typical_age_range_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"typicalAgeRange",
									));
								}
								r#typical_age_range_property = Some({
									struct DeserializeWith(Vec<TypicalAgeRangeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
							Field::UsageInfo => {
								if r#usage_info_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"usageInfo",
									));
								}
								r#usage_info_property = Some({
									struct DeserializeWith(Vec<UsageInfoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Version => {
								if r#version_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"version",
									));
								}
								r#version_property = Some({
									struct DeserializeWith(Vec<VersionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::Video => {
								if r#video_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("video"));
								}
								r#video_property = Some({
									struct DeserializeWith(Vec<VideoProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::WorkExample => {
								if r#work_example_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"workExample",
									));
								}
								r#work_example_property = Some({
									struct DeserializeWith(Vec<WorkExampleProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::WorkTranslation => {
								if r#work_translation_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"workTranslation",
									));
								}
								r#work_translation_property = Some({
									struct DeserializeWith(Vec<WorkTranslationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(Legislation {
						r#about: r#about_property.unwrap_or_default(),
						r#abstract: r#abstract_property.unwrap_or_default(),
						r#access_mode: r#access_mode_property.unwrap_or_default(),
						r#access_mode_sufficient: r#access_mode_sufficient_property
							.unwrap_or_default(),
						r#accessibility_api: r#accessibility_api_property.unwrap_or_default(),
						r#accessibility_control: r#accessibility_control_property
							.unwrap_or_default(),
						r#accessibility_feature: r#accessibility_feature_property
							.unwrap_or_default(),
						r#accessibility_hazard: r#accessibility_hazard_property.unwrap_or_default(),
						r#accessibility_summary: r#accessibility_summary_property
							.unwrap_or_default(),
						r#accountable_person: r#accountable_person_property.unwrap_or_default(),
						r#acquire_license_page: r#acquire_license_page_property.unwrap_or_default(),
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#alternative_headline: r#alternative_headline_property.unwrap_or_default(),
						r#archived_at: r#archived_at_property.unwrap_or_default(),
						r#assesses: r#assesses_property.unwrap_or_default(),
						r#associated_media: r#associated_media_property.unwrap_or_default(),
						r#audience: r#audience_property.unwrap_or_default(),
						r#audio: r#audio_property.unwrap_or_default(),
						r#author: r#author_property.unwrap_or_default(),
						r#award: r#award_property.unwrap_or_default(),
						r#awards: r#awards_property.unwrap_or_default(),
						r#character: r#character_property.unwrap_or_default(),
						r#citation: r#citation_property.unwrap_or_default(),
						r#comment: r#comment_property.unwrap_or_default(),
						r#comment_count: r#comment_count_property.unwrap_or_default(),
						r#conditions_of_access: r#conditions_of_access_property.unwrap_or_default(),
						r#content_location: r#content_location_property.unwrap_or_default(),
						r#content_rating: r#content_rating_property.unwrap_or_default(),
						r#content_reference_time: r#content_reference_time_property
							.unwrap_or_default(),
						r#contributor: r#contributor_property.unwrap_or_default(),
						r#copyright_holder: r#copyright_holder_property.unwrap_or_default(),
						r#copyright_notice: r#copyright_notice_property.unwrap_or_default(),
						r#copyright_year: r#copyright_year_property.unwrap_or_default(),
						r#correction: r#correction_property.unwrap_or_default(),
						r#country_of_origin: r#country_of_origin_property.unwrap_or_default(),
						r#creative_work_status: r#creative_work_status_property.unwrap_or_default(),
						r#creator: r#creator_property.unwrap_or_default(),
						r#credit_text: r#credit_text_property.unwrap_or_default(),
						r#date_created: r#date_created_property.unwrap_or_default(),
						r#date_modified: r#date_modified_property.unwrap_or_default(),
						r#date_published: r#date_published_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#discussion_url: r#discussion_url_property.unwrap_or_default(),
						r#edit_eidr: r#edit_eidr_property.unwrap_or_default(),
						r#editor: r#editor_property.unwrap_or_default(),
						r#educational_alignment: r#educational_alignment_property
							.unwrap_or_default(),
						r#educational_level: r#educational_level_property.unwrap_or_default(),
						r#educational_use: r#educational_use_property.unwrap_or_default(),
						r#encoding: r#encoding_property.unwrap_or_default(),
						r#encoding_format: r#encoding_format_property.unwrap_or_default(),
						r#encodings: r#encodings_property.unwrap_or_default(),
						r#example_of_work: r#example_of_work_property.unwrap_or_default(),
						r#expires: r#expires_property.unwrap_or_default(),
						r#file_format: r#file_format_property.unwrap_or_default(),
						r#funder: r#funder_property.unwrap_or_default(),
						r#funding: r#funding_property.unwrap_or_default(),
						r#genre: r#genre_property.unwrap_or_default(),
						r#has_part: r#has_part_property.unwrap_or_default(),
						r#headline: r#headline_property.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#in_language: r#in_language_property.unwrap_or_default(),
						r#interaction_statistic: r#interaction_statistic_property
							.unwrap_or_default(),
						r#interactivity_type: r#interactivity_type_property.unwrap_or_default(),
						r#interpreted_as_claim: r#interpreted_as_claim_property.unwrap_or_default(),
						r#is_accessible_for_free: r#is_accessible_for_free_property
							.unwrap_or_default(),
						r#is_based_on: r#is_based_on_property.unwrap_or_default(),
						r#is_based_on_url: r#is_based_on_url_property.unwrap_or_default(),
						r#is_family_friendly: r#is_family_friendly_property.unwrap_or_default(),
						r#is_part_of: r#is_part_of_property.unwrap_or_default(),
						r#jurisdiction: r#jurisdiction_property.unwrap_or_default(),
						r#keywords: r#keywords_property.unwrap_or_default(),
						r#learning_resource_type: r#learning_resource_type_property
							.unwrap_or_default(),
						r#legislation_applies: r#legislation_applies_property.unwrap_or_default(),
						r#legislation_changes: r#legislation_changes_property.unwrap_or_default(),
						r#legislation_consolidates: r#legislation_consolidates_property
							.unwrap_or_default(),
						r#legislation_date: r#legislation_date_property.unwrap_or_default(),
						r#legislation_date_version: r#legislation_date_version_property
							.unwrap_or_default(),
						r#legislation_identifier: r#legislation_identifier_property
							.unwrap_or_default(),
						r#legislation_jurisdiction: r#legislation_jurisdiction_property
							.unwrap_or_default(),
						r#legislation_legal_force: r#legislation_legal_force_property
							.unwrap_or_default(),
						r#legislation_passed_by: r#legislation_passed_by_property
							.unwrap_or_default(),
						r#legislation_responsible: r#legislation_responsible_property
							.unwrap_or_default(),
						r#legislation_transposes: r#legislation_transposes_property
							.unwrap_or_default(),
						r#legislation_type: r#legislation_type_property.unwrap_or_default(),
						r#license: r#license_property.unwrap_or_default(),
						r#location_created: r#location_created_property.unwrap_or_default(),
						r#main_entity: r#main_entity_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#maintainer: r#maintainer_property.unwrap_or_default(),
						r#material: r#material_property.unwrap_or_default(),
						r#material_extent: r#material_extent_property.unwrap_or_default(),
						r#mentions: r#mentions_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#pattern: r#pattern_property.unwrap_or_default(),
						r#position: r#position_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#producer: r#producer_property.unwrap_or_default(),
						r#provider: r#provider_property.unwrap_or_default(),
						r#publication: r#publication_property.unwrap_or_default(),
						r#publisher: r#publisher_property.unwrap_or_default(),
						r#publisher_imprint: r#publisher_imprint_property.unwrap_or_default(),
						r#publishing_principles: r#publishing_principles_property
							.unwrap_or_default(),
						r#recorded_at: r#recorded_at_property.unwrap_or_default(),
						r#released_event: r#released_event_property.unwrap_or_default(),
						r#review: r#review_property.unwrap_or_default(),
						r#reviews: r#reviews_property.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#schema_version: r#schema_version_property.unwrap_or_default(),
						r#sd_date_published: r#sd_date_published_property.unwrap_or_default(),
						r#sd_license: r#sd_license_property.unwrap_or_default(),
						r#sd_publisher: r#sd_publisher_property.unwrap_or_default(),
						r#size: r#size_property.unwrap_or_default(),
						r#source_organization: r#source_organization_property.unwrap_or_default(),
						r#spatial: r#spatial_property.unwrap_or_default(),
						r#spatial_coverage: r#spatial_coverage_property.unwrap_or_default(),
						r#sponsor: r#sponsor_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#teaches: r#teaches_property.unwrap_or_default(),
						r#temporal: r#temporal_property.unwrap_or_default(),
						r#temporal_coverage: r#temporal_coverage_property.unwrap_or_default(),
						r#text: r#text_property.unwrap_or_default(),
						r#thumbnail: r#thumbnail_property.unwrap_or_default(),
						r#thumbnail_url: r#thumbnail_url_property.unwrap_or_default(),
						r#time_required: r#time_required_property.unwrap_or_default(),
						r#translation_of_work: r#translation_of_work_property.unwrap_or_default(),
						r#translator: r#translator_property.unwrap_or_default(),
						r#typical_age_range: r#typical_age_range_property.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
						r#usage_info: r#usage_info_property.unwrap_or_default(),
						r#version: r#version_property.unwrap_or_default(),
						r#video: r#video_property.unwrap_or_default(),
						r#work_example: r#work_example_property.unwrap_or_default(),
						r#work_translation: r#work_translation_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"about",
				"abstract",
				"accessMode",
				"accessModeSufficient",
				"accessibilityAPI",
				"accessibilityControl",
				"accessibilityFeature",
				"accessibilityHazard",
				"accessibilitySummary",
				"accountablePerson",
				"acquireLicensePage",
				"additionalType",
				"aggregateRating",
				"alternateName",
				"alternativeHeadline",
				"archivedAt",
				"assesses",
				"associatedMedia",
				"audience",
				"audio",
				"author",
				"award",
				"awards",
				"character",
				"citation",
				"comment",
				"commentCount",
				"conditionsOfAccess",
				"contentLocation",
				"contentRating",
				"contentReferenceTime",
				"contributor",
				"copyrightHolder",
				"copyrightNotice",
				"copyrightYear",
				"correction",
				"countryOfOrigin",
				"creativeWorkStatus",
				"creator",
				"creditText",
				"dateCreated",
				"dateModified",
				"datePublished",
				"description",
				"disambiguatingDescription",
				"discussionUrl",
				"editEIDR",
				"editor",
				"educationalAlignment",
				"educationalLevel",
				"educationalUse",
				"encoding",
				"encodingFormat",
				"encodings",
				"exampleOfWork",
				"expires",
				"fileFormat",
				"funder",
				"funding",
				"genre",
				"hasPart",
				"headline",
				"identifier",
				"image",
				"inLanguage",
				"interactionStatistic",
				"interactivityType",
				"interpretedAsClaim",
				"isAccessibleForFree",
				"isBasedOn",
				"isBasedOnUrl",
				"isFamilyFriendly",
				"isPartOf",
				"jurisdiction",
				"keywords",
				"learningResourceType",
				"legislationApplies",
				"legislationChanges",
				"legislationConsolidates",
				"legislationDate",
				"legislationDateVersion",
				"legislationIdentifier",
				"legislationJurisdiction",
				"legislationLegalForce",
				"legislationPassedBy",
				"legislationResponsible",
				"legislationTransposes",
				"legislationType",
				"license",
				"locationCreated",
				"mainEntity",
				"mainEntityOfPage",
				"maintainer",
				"material",
				"materialExtent",
				"mentions",
				"name",
				"offers",
				"pattern",
				"position",
				"potentialAction",
				"producer",
				"provider",
				"publication",
				"publisher",
				"publisherImprint",
				"publishingPrinciples",
				"recordedAt",
				"releasedEvent",
				"review",
				"reviews",
				"sameAs",
				"schemaVersion",
				"sdDatePublished",
				"sdLicense",
				"sdPublisher",
				"size",
				"sourceOrganization",
				"spatial",
				"spatialCoverage",
				"sponsor",
				"subjectOf",
				"teaches",
				"temporal",
				"temporalCoverage",
				"text",
				"thumbnail",
				"thumbnailUrl",
				"timeRequired",
				"translationOfWork",
				"translator",
				"typicalAgeRange",
				"url",
				"usageInfo",
				"version",
				"video",
				"workExample",
				"workTranslation",
			];
			deserializer.deserialize_struct("Legislation", FIELDS, ClassVisitor)
		}
	}
}
