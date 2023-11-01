use super::*;
/// <https://schema.org/VideoObjectSnapshot>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct VideoObjectSnapshot {
	#[cfg(any(
		any(feature = "about-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#about: Vec<AboutProperty>,
	#[cfg(any(
		any(
			feature = "abstract-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#abstract: Vec<AbstractProperty>,
	#[cfg(any(
		any(
			feature = "access-mode-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#access_mode: Vec<AccessModeProperty>,
	#[cfg(any(
		any(
			feature = "access-mode-sufficient-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#access_mode_sufficient: Vec<AccessModeSufficientProperty>,
	#[cfg(any(
		any(
			feature = "accessibility-api-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#accessibility_api: Vec<AccessibilityApiProperty>,
	#[cfg(any(
		any(
			feature = "accessibility-control-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#accessibility_control: Vec<AccessibilityControlProperty>,
	#[cfg(any(
		any(
			feature = "accessibility-feature-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#accessibility_feature: Vec<AccessibilityFeatureProperty>,
	#[cfg(any(
		any(
			feature = "accessibility-hazard-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#accessibility_hazard: Vec<AccessibilityHazardProperty>,
	#[cfg(any(
		any(
			feature = "accessibility-summary-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#accessibility_summary: Vec<AccessibilitySummaryProperty>,
	#[cfg(any(
		any(
			feature = "accountable-person-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#accountable_person: Vec<AccountablePersonProperty>,
	#[cfg(any(
		any(
			feature = "acquire-license-page-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#acquire_license_page: Vec<AcquireLicensePageProperty>,
	#[cfg(any(
		any(feature = "actor-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#actor: Vec<ActorProperty>,
	#[cfg(any(
		any(feature = "actors-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#actors: Vec<ActorsProperty>,
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
		any(
			feature = "alternative-headline-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternative_headline: Vec<AlternativeHeadlineProperty>,
	#[cfg(any(
		any(
			feature = "archived-at-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#archived_at: Vec<ArchivedAtProperty>,
	#[cfg(any(
		any(
			feature = "assesses-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#assesses: Vec<AssessesProperty>,
	#[cfg(any(
		any(
			feature = "associated-article-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#associated_article: Vec<AssociatedArticleProperty>,
	#[cfg(any(
		any(
			feature = "associated-media-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#associated_media: Vec<AssociatedMediaProperty>,
	#[cfg(any(
		any(
			feature = "audience-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#audience: Vec<AudienceProperty>,
	#[cfg(any(
		any(feature = "audio-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#audio: Vec<AudioProperty>,
	#[cfg(any(
		any(feature = "author-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#author: Vec<AuthorProperty>,
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
			feature = "bitrate-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#bitrate: Vec<BitrateProperty>,
	#[cfg(any(
		any(
			feature = "caption-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#caption: Vec<CaptionProperty>,
	#[cfg(any(
		any(
			feature = "character-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#character: Vec<CharacterProperty>,
	#[cfg(any(
		any(
			feature = "citation-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#citation: Vec<CitationProperty>,
	#[cfg(any(
		any(
			feature = "comment-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#comment: Vec<CommentProperty>,
	#[cfg(any(
		any(
			feature = "comment-count-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#comment_count: Vec<CommentCountProperty>,
	#[cfg(any(
		any(
			feature = "conditions-of-access-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#conditions_of_access: Vec<ConditionsOfAccessProperty>,
	#[cfg(any(
		any(
			feature = "content-location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#content_location: Vec<ContentLocationProperty>,
	#[cfg(any(
		any(
			feature = "content-rating-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#content_rating: Vec<ContentRatingProperty>,
	#[cfg(any(
		any(
			feature = "content-reference-time-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#content_reference_time: Vec<ContentReferenceTimeProperty>,
	#[cfg(any(
		any(
			feature = "content-size-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#content_size: Vec<ContentSizeProperty>,
	#[cfg(any(
		any(
			feature = "content-url-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#content_url: Vec<ContentUrlProperty>,
	#[cfg(any(
		any(
			feature = "contributor-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#contributor: Vec<ContributorProperty>,
	#[cfg(any(
		any(
			feature = "copyright-holder-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#copyright_holder: Vec<CopyrightHolderProperty>,
	#[cfg(any(
		any(
			feature = "copyright-notice-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#copyright_notice: Vec<CopyrightNoticeProperty>,
	#[cfg(any(
		any(
			feature = "copyright-year-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#copyright_year: Vec<CopyrightYearProperty>,
	#[cfg(any(
		any(
			feature = "correction-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#correction: Vec<CorrectionProperty>,
	#[cfg(any(
		any(
			feature = "country-of-origin-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#country_of_origin: Vec<CountryOfOriginProperty>,
	#[cfg(any(
		any(
			feature = "creative-work-status-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#creative_work_status: Vec<CreativeWorkStatusProperty>,
	#[cfg(any(
		any(
			feature = "creator-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#creator: Vec<CreatorProperty>,
	#[cfg(any(
		any(
			feature = "credit-text-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#credit_text: Vec<CreditTextProperty>,
	#[cfg(any(
		any(
			feature = "date-created-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#date_created: Vec<DateCreatedProperty>,
	#[cfg(any(
		any(
			feature = "date-modified-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#date_modified: Vec<DateModifiedProperty>,
	#[cfg(any(
		any(
			feature = "date-published-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#date_published: Vec<DatePublishedProperty>,
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
			feature = "director-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#director: Vec<DirectorProperty>,
	#[cfg(any(
		any(
			feature = "directors-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#directors: Vec<DirectorsProperty>,
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
			feature = "discussion-url-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#discussion_url: Vec<DiscussionUrlProperty>,
	#[cfg(any(
		any(
			feature = "duration-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#duration: Vec<DurationProperty>,
	#[cfg(any(
		any(
			feature = "edit-eidr-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#edit_eidr: Vec<EditEidrProperty>,
	#[cfg(any(
		any(feature = "editor-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#editor: Vec<EditorProperty>,
	#[cfg(any(
		any(
			feature = "educational-alignment-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#educational_alignment: Vec<EducationalAlignmentProperty>,
	#[cfg(any(
		any(
			feature = "educational-level-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#educational_level: Vec<EducationalLevelProperty>,
	#[cfg(any(
		any(
			feature = "educational-use-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#educational_use: Vec<EducationalUseProperty>,
	#[cfg(any(
		any(
			feature = "embed-url-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#embed_url: Vec<EmbedUrlProperty>,
	#[cfg(any(
		any(
			feature = "embedded-text-caption-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#embedded_text_caption: Vec<EmbeddedTextCaptionProperty>,
	#[cfg(any(
		any(
			feature = "encodes-creative-work-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#encodes_creative_work: Vec<EncodesCreativeWorkProperty>,
	#[cfg(any(
		any(
			feature = "encoding-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#encoding: Vec<EncodingProperty>,
	#[cfg(any(
		any(
			feature = "encoding-format-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#encoding_format: Vec<EncodingFormatProperty>,
	#[cfg(any(
		any(
			feature = "encodings-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#encodings: Vec<EncodingsProperty>,
	#[cfg(any(
		any(
			feature = "end-time-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#end_time: Vec<EndTimeProperty>,
	#[cfg(any(
		any(
			feature = "example-of-work-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#example_of_work: Vec<ExampleOfWorkProperty>,
	#[cfg(any(
		any(
			feature = "expires-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#expires: Vec<ExpiresProperty>,
	#[cfg(any(
		any(
			feature = "file-format-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#file_format: Vec<FileFormatProperty>,
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
		any(feature = "genre-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#genre: Vec<GenreProperty>,
	#[cfg(any(
		any(
			feature = "has-part-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#has_part: Vec<HasPartProperty>,
	#[cfg(any(
		any(
			feature = "headline-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#headline: Vec<HeadlineProperty>,
	#[cfg(any(
		any(feature = "height-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#height: Vec<HeightProperty>,
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
			feature = "in-language-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#in_language: Vec<InLanguageProperty>,
	#[cfg(any(
		any(
			feature = "ineligible-region-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#ineligible_region: Vec<IneligibleRegionProperty>,
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
			feature = "interactivity-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#interactivity_type: Vec<InteractivityTypeProperty>,
	#[cfg(any(
		any(
			feature = "interpreted-as-claim-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#interpreted_as_claim: Vec<InterpretedAsClaimProperty>,
	#[cfg(any(
		any(
			feature = "is-accessible-for-free-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	#[cfg(any(
		any(
			feature = "is-based-on-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_based_on: Vec<IsBasedOnProperty>,
	#[cfg(any(
		any(
			feature = "is-based-on-url-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_based_on_url: Vec<IsBasedOnUrlProperty>,
	#[cfg(any(
		any(
			feature = "is-family-friendly-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
	#[cfg(any(
		any(
			feature = "is-part-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_part_of: Vec<IsPartOfProperty>,
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
			feature = "learning-resource-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#learning_resource_type: Vec<LearningResourceTypeProperty>,
	#[cfg(any(
		any(
			feature = "license-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#license: Vec<LicenseProperty>,
	#[cfg(any(
		any(
			feature = "location-created-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#location_created: Vec<LocationCreatedProperty>,
	#[cfg(any(
		any(
			feature = "main-entity-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#main_entity: Vec<MainEntityProperty>,
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
			feature = "maintainer-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#maintainer: Vec<MaintainerProperty>,
	#[cfg(any(
		any(
			feature = "material-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#material: Vec<MaterialProperty>,
	#[cfg(any(
		any(
			feature = "material-extent-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#material_extent: Vec<MaterialExtentProperty>,
	#[cfg(any(
		any(
			feature = "mentions-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#mentions: Vec<MentionsProperty>,
	#[cfg(any(
		any(
			feature = "music-by-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#music_by: Vec<MusicByProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(feature = "offers-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#offers: Vec<OffersProperty>,
	#[cfg(any(
		any(
			feature = "pattern-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#pattern: Vec<PatternProperty>,
	#[cfg(any(
		any(
			feature = "player-type-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#player_type: Vec<PlayerTypeProperty>,
	#[cfg(any(
		any(
			feature = "position-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#position: Vec<PositionProperty>,
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
			feature = "producer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#producer: Vec<ProducerProperty>,
	#[cfg(any(
		any(
			feature = "production-company-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#production_company: Vec<ProductionCompanyProperty>,
	#[cfg(any(
		any(
			feature = "provider-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#provider: Vec<ProviderProperty>,
	#[cfg(any(
		any(
			feature = "publication-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#publication: Vec<PublicationProperty>,
	#[cfg(any(
		any(
			feature = "publisher-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#publisher: Vec<PublisherProperty>,
	#[cfg(any(
		any(
			feature = "publisher-imprint-property-schema",
			feature = "bib-schema-section"
		),
		doc
	))]
	pub r#publisher_imprint: Vec<PublisherImprintProperty>,
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
			feature = "recorded-at-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#recorded_at: Vec<RecordedAtProperty>,
	#[cfg(any(
		any(
			feature = "regions-allowed-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#regions_allowed: Vec<RegionsAllowedProperty>,
	#[cfg(any(
		any(
			feature = "released-event-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#released_event: Vec<ReleasedEventProperty>,
	#[cfg(any(
		any(
			feature = "requires-subscription-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#requires_subscription: Vec<RequiresSubscriptionProperty>,
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
		any(
			feature = "schema-version-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#schema_version: Vec<SchemaVersionProperty>,
	#[cfg(any(
		any(
			feature = "sd-date-published-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#sd_date_published: Vec<SdDatePublishedProperty>,
	#[cfg(any(
		any(
			feature = "sd-license-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#sd_license: Vec<SdLicenseProperty>,
	#[cfg(any(
		any(
			feature = "sd-publisher-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#sd_publisher: Vec<SdPublisherProperty>,
	#[cfg(any(
		any(
			feature = "sha-256-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#sha_256: Vec<Sha256Property>,
	#[cfg(any(
		any(feature = "size-property-schema", feature = "pending-schema-section"),
		doc
	))]
	pub r#size: Vec<SizeProperty>,
	#[cfg(any(
		any(
			feature = "source-organization-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#source_organization: Vec<SourceOrganizationProperty>,
	#[cfg(any(
		any(
			feature = "spatial-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#spatial: Vec<SpatialProperty>,
	#[cfg(any(
		any(
			feature = "spatial-coverage-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#spatial_coverage: Vec<SpatialCoverageProperty>,
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
			feature = "start-time-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#start_time: Vec<StartTimeProperty>,
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
			feature = "teaches-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#teaches: Vec<TeachesProperty>,
	#[cfg(any(
		any(
			feature = "temporal-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#temporal: Vec<TemporalProperty>,
	#[cfg(any(
		any(
			feature = "temporal-coverage-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#temporal_coverage: Vec<TemporalCoverageProperty>,
	#[cfg(any(
		any(feature = "text-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#text: Vec<TextProperty>,
	#[cfg(any(
		any(
			feature = "thumbnail-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#thumbnail: Vec<ThumbnailProperty>,
	#[cfg(any(
		any(
			feature = "thumbnail-url-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#thumbnail_url: Vec<ThumbnailUrlProperty>,
	#[cfg(any(
		any(
			feature = "time-required-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#time_required: Vec<TimeRequiredProperty>,
	#[cfg(any(
		any(
			feature = "transcript-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#transcript: Vec<TranscriptProperty>,
	#[cfg(any(
		any(
			feature = "translation-of-work-property-schema",
			feature = "bib-schema-section"
		),
		doc
	))]
	pub r#translation_of_work: Vec<TranslationOfWorkProperty>,
	#[cfg(any(
		any(
			feature = "translator-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#translator: Vec<TranslatorProperty>,
	#[cfg(any(
		any(
			feature = "typical-age-range-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#typical_age_range: Vec<TypicalAgeRangeProperty>,
	#[cfg(any(
		any(
			feature = "upload-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#upload_date: Vec<UploadDateProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
	#[cfg(any(
		any(
			feature = "usage-info-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#usage_info: Vec<UsageInfoProperty>,
	#[cfg(any(
		any(
			feature = "version-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#version: Vec<VersionProperty>,
	#[cfg(any(
		any(feature = "video-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#video: Vec<VideoProperty>,
	#[cfg(any(
		any(
			feature = "video-frame-size-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#video_frame_size: Vec<VideoFrameSizeProperty>,
	#[cfg(any(
		any(
			feature = "video-quality-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#video_quality: Vec<VideoQualityProperty>,
	#[cfg(any(
		any(feature = "width-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#width: Vec<WidthProperty>,
	#[cfg(any(
		any(
			feature = "work-example-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#work_example: Vec<WorkExampleProperty>,
	#[cfg(any(
		any(
			feature = "work-translation-property-schema",
			feature = "bib-schema-section"
		),
		doc
	))]
	pub r#work_translation: Vec<WorkTranslationProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for VideoObjectSnapshot {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "about-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#about) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "abstract-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#abstract) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "access-mode-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#access_mode) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "access-mode-sufficient-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#access_mode_sufficient) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "accessibility-api-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#accessibility_api) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "accessibility-control-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#accessibility_control) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "accessibility-feature-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#accessibility_feature) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "accessibility-hazard-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#accessibility_hazard) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "accessibility-summary-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#accessibility_summary) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "accountable-person-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#accountable_person) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "acquire-license-page-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#acquire_license_page) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "actor-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#actor) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "actors-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#actors) as usize
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
						feature = "alternative-headline-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#alternative_headline) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "archived-at-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#archived_at) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "assesses-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#assesses) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "associated-article-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#associated_article) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "associated-media-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#associated_media) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#audience) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "audio-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#audio) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "author-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#author) as usize
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
						feature = "bitrate-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#bitrate) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "caption-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#caption) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "character-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#character) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "citation-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#citation) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "comment-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#comment) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "comment-count-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#comment_count) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "conditions-of-access-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#conditions_of_access) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "content-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#content_location) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "content-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#content_rating) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "content-reference-time-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#content_reference_time) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "content-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#content_size) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "content-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#content_url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "contributor-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#contributor) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "copyright-holder-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#copyright_holder) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "copyright-notice-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#copyright_notice) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "copyright-year-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#copyright_year) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "correction-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#correction) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#country_of_origin) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "creative-work-status-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#creative_work_status) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "creator-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#creator) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "credit-text-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#credit_text) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "date-created-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#date_created) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "date-modified-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#date_modified) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "date-published-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#date_published) as usize
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
						feature = "director-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#director) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "directors-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#directors) as usize
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
						feature = "discussion-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#discussion_url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#duration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "edit-eidr-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#edit_eidr) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "editor-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#editor) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "educational-alignment-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#educational_alignment) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "educational-level-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#educational_level) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "educational-use-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#educational_use) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "embed-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#embed_url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "embedded-text-caption-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#embedded_text_caption) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "encodes-creative-work-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#encodes_creative_work) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "encoding-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#encoding) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "encoding-format-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#encoding_format) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "encodings-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#encodings) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "end-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#end_time) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "example-of-work-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#example_of_work) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "expires-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#expires) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "file-format-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#file_format) as usize
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
						feature = "genre-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#genre) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-part-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_part) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "headline-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#headline) as usize
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
						feature = "in-language-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#in_language) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "ineligible-region-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#ineligible_region) as usize
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
						feature = "interactivity-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#interactivity_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "interpreted-as-claim-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#interpreted_as_claim) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-accessible-for-free-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_accessible_for_free) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-based-on-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_based_on) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-based-on-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_based_on_url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_family_friendly) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-part-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_part_of) as usize
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
						feature = "learning-resource-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#learning_resource_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "license-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#license) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "location-created-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#location_created) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "main-entity-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#main_entity) as usize
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
						feature = "maintainer-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#maintainer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#material) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "material-extent-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#material_extent) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "mentions-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#mentions) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "music-by-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#music_by) as usize
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
						feature = "offers-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#offers) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#pattern) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "player-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#player_type) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "position-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#position) as usize
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
						feature = "producer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#producer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "production-company-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#production_company) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "provider-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#provider) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "publication-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#publication) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "publisher-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#publisher) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "publisher-imprint-property-schema",
						feature = "bib-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#publisher_imprint) as usize
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
						feature = "recorded-at-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#recorded_at) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "regions-allowed-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#regions_allowed) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "released-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#released_event) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "requires-subscription-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#requires_subscription) as usize
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
						feature = "schema-version-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#schema_version) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sd-date-published-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sd_date_published) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sd-license-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sd_license) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sd-publisher-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sd_publisher) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sha-256-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sha_256) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "size-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#size) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "source-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#source_organization) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "spatial-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#spatial) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "spatial-coverage-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#spatial_coverage) as usize
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
						feature = "start-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#start_time) as usize
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
						feature = "teaches-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#teaches) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "temporal-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#temporal) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "temporal-coverage-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#temporal_coverage) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "text-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#text) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "thumbnail-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#thumbnail) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "thumbnail-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#thumbnail_url) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "time-required-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#time_required) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "transcript-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#transcript) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "translation-of-work-property-schema",
						feature = "bib-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#translation_of_work) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "translator-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#translator) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "typical-age-range-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#typical_age_range) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "upload-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#upload_date) as usize
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
						feature = "usage-info-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#usage_info) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "version-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#version) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "video-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#video) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "video-frame-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#video_frame_size) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "video-quality-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#video_quality) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "width-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#width) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "work-example-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#work_example) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "work-translation-property-schema",
						feature = "bib-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#work_translation) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "VideoObjectSnapshot", len)?;
			#[cfg(any(
				any(feature = "about-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "abstract-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "access-mode-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "access-mode-sufficient-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "accessibility-api-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "accessibility-control-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "accessibility-feature-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "accessibility-hazard-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "accessibility-summary-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "accountable-person-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "acquire-license-page-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "actor-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#actor) {
				serialize_struct.serialize_field("actor", {
					struct SerializeWith<'a>(&'a Vec<ActorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#actor)
				})?;
			} else {
				serialize_struct.skip_field("actor")?;
			}
			#[cfg(any(
				any(feature = "actors-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#actors) {
				serialize_struct.serialize_field("actors", {
					struct SerializeWith<'a>(&'a Vec<ActorsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#actors)
				})?;
			} else {
				serialize_struct.skip_field("actors")?;
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
				any(
					feature = "alternative-headline-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "archived-at-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "assesses-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "associated-article-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#associated_article) {
				serialize_struct.serialize_field("associatedArticle", {
					struct SerializeWith<'a>(&'a Vec<AssociatedArticleProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#associated_article)
				})?;
			} else {
				serialize_struct.skip_field("associatedArticle")?;
			}
			#[cfg(any(
				any(
					feature = "associated-media-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "audience-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "audio-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(feature = "author-property-schema", feature = "general-schema-section"),
				doc
			))]
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
					feature = "bitrate-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#bitrate) {
				serialize_struct.serialize_field("bitrate", {
					struct SerializeWith<'a>(&'a Vec<BitrateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#bitrate)
				})?;
			} else {
				serialize_struct.skip_field("bitrate")?;
			}
			#[cfg(any(
				any(
					feature = "caption-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#caption) {
				serialize_struct.serialize_field("caption", {
					struct SerializeWith<'a>(&'a Vec<CaptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#caption)
				})?;
			} else {
				serialize_struct.skip_field("caption")?;
			}
			#[cfg(any(
				any(
					feature = "character-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "citation-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "comment-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "comment-count-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "conditions-of-access-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "content-location-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "content-rating-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "content-reference-time-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "content-size-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#content_size) {
				serialize_struct.serialize_field("contentSize", {
					struct SerializeWith<'a>(&'a Vec<ContentSizeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#content_size)
				})?;
			} else {
				serialize_struct.skip_field("contentSize")?;
			}
			#[cfg(any(
				any(
					feature = "content-url-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#content_url) {
				serialize_struct.serialize_field("contentUrl", {
					struct SerializeWith<'a>(&'a Vec<ContentUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#content_url)
				})?;
			} else {
				serialize_struct.skip_field("contentUrl")?;
			}
			#[cfg(any(
				any(
					feature = "contributor-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "copyright-holder-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "copyright-notice-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "copyright-year-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "correction-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "country-of-origin-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "creative-work-status-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "creator-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "credit-text-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "date-created-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "date-modified-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "date-published-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "director-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#director) {
				serialize_struct.serialize_field("director", {
					struct SerializeWith<'a>(&'a Vec<DirectorProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#director)
				})?;
			} else {
				serialize_struct.skip_field("director")?;
			}
			#[cfg(any(
				any(
					feature = "directors-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#directors) {
				serialize_struct.serialize_field("directors", {
					struct SerializeWith<'a>(&'a Vec<DirectorsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#directors)
				})?;
			} else {
				serialize_struct.skip_field("directors")?;
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
					feature = "discussion-url-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "duration-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#duration) {
				serialize_struct.serialize_field("duration", {
					struct SerializeWith<'a>(&'a Vec<DurationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#duration)
				})?;
			} else {
				serialize_struct.skip_field("duration")?;
			}
			#[cfg(any(
				any(
					feature = "edit-eidr-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "editor-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "educational-alignment-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "educational-level-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "educational-use-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "embed-url-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#embed_url) {
				serialize_struct.serialize_field("embedUrl", {
					struct SerializeWith<'a>(&'a Vec<EmbedUrlProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#embed_url)
				})?;
			} else {
				serialize_struct.skip_field("embedUrl")?;
			}
			#[cfg(any(
				any(
					feature = "embedded-text-caption-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#embedded_text_caption) {
				serialize_struct.serialize_field("embeddedTextCaption", {
					struct SerializeWith<'a>(&'a Vec<EmbeddedTextCaptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#embedded_text_caption)
				})?;
			} else {
				serialize_struct.skip_field("embeddedTextCaption")?;
			}
			#[cfg(any(
				any(
					feature = "encodes-creative-work-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#encodes_creative_work) {
				serialize_struct.serialize_field("encodesCreativeWork", {
					struct SerializeWith<'a>(&'a Vec<EncodesCreativeWorkProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#encodes_creative_work)
				})?;
			} else {
				serialize_struct.skip_field("encodesCreativeWork")?;
			}
			#[cfg(any(
				any(
					feature = "encoding-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "encoding-format-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "encodings-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "end-time-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#end_time) {
				serialize_struct.serialize_field("endTime", {
					struct SerializeWith<'a>(&'a Vec<EndTimeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#end_time)
				})?;
			} else {
				serialize_struct.skip_field("endTime")?;
			}
			#[cfg(any(
				any(
					feature = "example-of-work-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "expires-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "file-format-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
				any(feature = "genre-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "has-part-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "headline-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "in-language-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "ineligible-region-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#ineligible_region) {
				serialize_struct.serialize_field("ineligibleRegion", {
					struct SerializeWith<'a>(&'a Vec<IneligibleRegionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#ineligible_region)
				})?;
			} else {
				serialize_struct.skip_field("ineligibleRegion")?;
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
					feature = "interactivity-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "interpreted-as-claim-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-accessible-for-free-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-based-on-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-based-on-url-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-family-friendly-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-part-of-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "learning-resource-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "license-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "location-created-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "main-entity-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "maintainer-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "material-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "material-extent-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "mentions-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "music-by-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#music_by) {
				serialize_struct.serialize_field("musicBy", {
					struct SerializeWith<'a>(&'a Vec<MusicByProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#music_by)
				})?;
			} else {
				serialize_struct.skip_field("musicBy")?;
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
				any(feature = "offers-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "pattern-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "player-type-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#player_type) {
				serialize_struct.serialize_field("playerType", {
					struct SerializeWith<'a>(&'a Vec<PlayerTypeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#player_type)
				})?;
			} else {
				serialize_struct.skip_field("playerType")?;
			}
			#[cfg(any(
				any(
					feature = "position-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "producer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "production-company-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#production_company) {
				serialize_struct.serialize_field("productionCompany", {
					struct SerializeWith<'a>(&'a Vec<ProductionCompanyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#production_company)
				})?;
			} else {
				serialize_struct.skip_field("productionCompany")?;
			}
			#[cfg(any(
				any(
					feature = "provider-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "publication-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "publisher-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "publisher-imprint-property-schema",
					feature = "bib-schema-section"
				),
				doc
			))]
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
					feature = "recorded-at-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "regions-allowed-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#regions_allowed) {
				serialize_struct.serialize_field("regionsAllowed", {
					struct SerializeWith<'a>(&'a Vec<RegionsAllowedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#regions_allowed)
				})?;
			} else {
				serialize_struct.skip_field("regionsAllowed")?;
			}
			#[cfg(any(
				any(
					feature = "released-event-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "requires-subscription-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#requires_subscription) {
				serialize_struct.serialize_field("requiresSubscription", {
					struct SerializeWith<'a>(&'a Vec<RequiresSubscriptionProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#requires_subscription)
				})?;
			} else {
				serialize_struct.skip_field("requiresSubscription")?;
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
				any(
					feature = "schema-version-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "sd-date-published-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "sd-license-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "sd-publisher-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "sha-256-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#sha_256) {
				serialize_struct.serialize_field("sha256", {
					struct SerializeWith<'a>(&'a Vec<Sha256Property>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sha_256)
				})?;
			} else {
				serialize_struct.skip_field("sha256")?;
			}
			#[cfg(any(
				any(feature = "size-property-schema", feature = "pending-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "source-organization-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "spatial-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "spatial-coverage-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "start-time-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#start_time) {
				serialize_struct.serialize_field("startTime", {
					struct SerializeWith<'a>(&'a Vec<StartTimeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#start_time)
				})?;
			} else {
				serialize_struct.skip_field("startTime")?;
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
					feature = "teaches-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "temporal-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "temporal-coverage-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "text-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "thumbnail-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "thumbnail-url-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "time-required-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "transcript-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#transcript) {
				serialize_struct.serialize_field("transcript", {
					struct SerializeWith<'a>(&'a Vec<TranscriptProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#transcript)
				})?;
			} else {
				serialize_struct.skip_field("transcript")?;
			}
			#[cfg(any(
				any(
					feature = "translation-of-work-property-schema",
					feature = "bib-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "translator-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "typical-age-range-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "upload-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#upload_date) {
				serialize_struct.serialize_field("uploadDate", {
					struct SerializeWith<'a>(&'a Vec<UploadDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#upload_date)
				})?;
			} else {
				serialize_struct.skip_field("uploadDate")?;
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
				any(
					feature = "usage-info-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "version-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "video-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "video-frame-size-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#video_frame_size) {
				serialize_struct.serialize_field("videoFrameSize", {
					struct SerializeWith<'a>(&'a Vec<VideoFrameSizeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#video_frame_size)
				})?;
			} else {
				serialize_struct.skip_field("videoFrameSize")?;
			}
			#[cfg(any(
				any(
					feature = "video-quality-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#video_quality) {
				serialize_struct.serialize_field("videoQuality", {
					struct SerializeWith<'a>(&'a Vec<VideoQualityProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#video_quality)
				})?;
			} else {
				serialize_struct.skip_field("videoQuality")?;
			}
			#[cfg(any(
				any(feature = "width-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#width) {
				serialize_struct.serialize_field("width", {
					struct SerializeWith<'a>(&'a Vec<WidthProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#width)
				})?;
			} else {
				serialize_struct.skip_field("width")?;
			}
			#[cfg(any(
				any(
					feature = "work-example-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "work-translation-property-schema",
					feature = "bib-schema-section"
				),
				doc
			))]
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
	impl<'de> Deserialize<'de> for VideoObjectSnapshot {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(feature = "about-property-schema", feature = "general-schema-section"),
					doc
				))]
				About,
				#[cfg(any(
					any(
						feature = "abstract-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Abstract,
				#[cfg(any(
					any(
						feature = "access-mode-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AccessMode,
				#[cfg(any(
					any(
						feature = "access-mode-sufficient-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AccessModeSufficient,
				#[cfg(any(
					any(
						feature = "accessibility-api-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AccessibilityApi,
				#[cfg(any(
					any(
						feature = "accessibility-control-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AccessibilityControl,
				#[cfg(any(
					any(
						feature = "accessibility-feature-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AccessibilityFeature,
				#[cfg(any(
					any(
						feature = "accessibility-hazard-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AccessibilityHazard,
				#[cfg(any(
					any(
						feature = "accessibility-summary-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AccessibilitySummary,
				#[cfg(any(
					any(
						feature = "accountable-person-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AccountablePerson,
				#[cfg(any(
					any(
						feature = "acquire-license-page-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				AcquireLicensePage,
				#[cfg(any(
					any(feature = "actor-property-schema", feature = "general-schema-section"),
					doc
				))]
				Actor,
				#[cfg(any(
					any(feature = "actors-property-schema", feature = "general-schema-section"),
					doc
				))]
				Actors,
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
					any(
						feature = "alternative-headline-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternativeHeadline,
				#[cfg(any(
					any(
						feature = "archived-at-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ArchivedAt,
				#[cfg(any(
					any(
						feature = "assesses-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Assesses,
				#[cfg(any(
					any(
						feature = "associated-article-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AssociatedArticle,
				#[cfg(any(
					any(
						feature = "associated-media-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AssociatedMedia,
				#[cfg(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Audience,
				#[cfg(any(
					any(feature = "audio-property-schema", feature = "general-schema-section"),
					doc
				))]
				Audio,
				#[cfg(any(
					any(feature = "author-property-schema", feature = "general-schema-section"),
					doc
				))]
				Author,
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
						feature = "bitrate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Bitrate,
				#[cfg(any(
					any(
						feature = "caption-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Caption,
				#[cfg(any(
					any(
						feature = "character-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Character,
				#[cfg(any(
					any(
						feature = "citation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Citation,
				#[cfg(any(
					any(
						feature = "comment-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Comment,
				#[cfg(any(
					any(
						feature = "comment-count-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				CommentCount,
				#[cfg(any(
					any(
						feature = "conditions-of-access-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ConditionsOfAccess,
				#[cfg(any(
					any(
						feature = "content-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ContentLocation,
				#[cfg(any(
					any(
						feature = "content-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ContentRating,
				#[cfg(any(
					any(
						feature = "content-reference-time-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ContentReferenceTime,
				#[cfg(any(
					any(
						feature = "content-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ContentSize,
				#[cfg(any(
					any(
						feature = "content-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ContentUrl,
				#[cfg(any(
					any(
						feature = "contributor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Contributor,
				#[cfg(any(
					any(
						feature = "copyright-holder-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				CopyrightHolder,
				#[cfg(any(
					any(
						feature = "copyright-notice-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CopyrightNotice,
				#[cfg(any(
					any(
						feature = "copyright-year-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				CopyrightYear,
				#[cfg(any(
					any(
						feature = "correction-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Correction,
				#[cfg(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				CountryOfOrigin,
				#[cfg(any(
					any(
						feature = "creative-work-status-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CreativeWorkStatus,
				#[cfg(any(
					any(
						feature = "creator-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Creator,
				#[cfg(any(
					any(
						feature = "credit-text-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CreditText,
				#[cfg(any(
					any(
						feature = "date-created-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DateCreated,
				#[cfg(any(
					any(
						feature = "date-modified-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DateModified,
				#[cfg(any(
					any(
						feature = "date-published-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DatePublished,
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
						feature = "director-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Director,
				#[cfg(any(
					any(
						feature = "directors-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Directors,
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
						feature = "discussion-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DiscussionUrl,
				#[cfg(any(
					any(
						feature = "duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Duration,
				#[cfg(any(
					any(
						feature = "edit-eidr-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EditEidr,
				#[cfg(any(
					any(feature = "editor-property-schema", feature = "general-schema-section"),
					doc
				))]
				Editor,
				#[cfg(any(
					any(
						feature = "educational-alignment-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EducationalAlignment,
				#[cfg(any(
					any(
						feature = "educational-level-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EducationalLevel,
				#[cfg(any(
					any(
						feature = "educational-use-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EducationalUse,
				#[cfg(any(
					any(
						feature = "embed-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EmbedUrl,
				#[cfg(any(
					any(
						feature = "embedded-text-caption-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EmbeddedTextCaption,
				#[cfg(any(
					any(
						feature = "encodes-creative-work-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EncodesCreativeWork,
				#[cfg(any(
					any(
						feature = "encoding-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Encoding,
				#[cfg(any(
					any(
						feature = "encoding-format-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EncodingFormat,
				#[cfg(any(
					any(
						feature = "encodings-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Encodings,
				#[cfg(any(
					any(
						feature = "end-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EndTime,
				#[cfg(any(
					any(
						feature = "example-of-work-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ExampleOfWork,
				#[cfg(any(
					any(
						feature = "expires-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Expires,
				#[cfg(any(
					any(
						feature = "file-format-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FileFormat,
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
					any(feature = "genre-property-schema", feature = "general-schema-section"),
					doc
				))]
				Genre,
				#[cfg(any(
					any(
						feature = "has-part-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				HasPart,
				#[cfg(any(
					any(
						feature = "headline-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Headline,
				#[cfg(any(
					any(feature = "height-property-schema", feature = "general-schema-section"),
					doc
				))]
				Height,
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
						feature = "in-language-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				InLanguage,
				#[cfg(any(
					any(
						feature = "ineligible-region-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				IneligibleRegion,
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
						feature = "interactivity-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				InteractivityType,
				#[cfg(any(
					any(
						feature = "interpreted-as-claim-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				InterpretedAsClaim,
				#[cfg(any(
					any(
						feature = "is-accessible-for-free-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsAccessibleForFree,
				#[cfg(any(
					any(
						feature = "is-based-on-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsBasedOn,
				#[cfg(any(
					any(
						feature = "is-based-on-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsBasedOnUrl,
				#[cfg(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsFamilyFriendly,
				#[cfg(any(
					any(
						feature = "is-part-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsPartOf,
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
						feature = "learning-resource-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				LearningResourceType,
				#[cfg(any(
					any(
						feature = "license-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				License,
				#[cfg(any(
					any(
						feature = "location-created-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				LocationCreated,
				#[cfg(any(
					any(
						feature = "main-entity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MainEntity,
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
						feature = "maintainer-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Maintainer,
				#[cfg(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Material,
				#[cfg(any(
					any(
						feature = "material-extent-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MaterialExtent,
				#[cfg(any(
					any(
						feature = "mentions-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Mentions,
				#[cfg(any(
					any(
						feature = "music-by-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MusicBy,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				Offers,
				#[cfg(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Pattern,
				#[cfg(any(
					any(
						feature = "player-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PlayerType,
				#[cfg(any(
					any(
						feature = "position-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Position,
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
						feature = "producer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Producer,
				#[cfg(any(
					any(
						feature = "production-company-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ProductionCompany,
				#[cfg(any(
					any(
						feature = "provider-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Provider,
				#[cfg(any(
					any(
						feature = "publication-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Publication,
				#[cfg(any(
					any(
						feature = "publisher-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Publisher,
				#[cfg(any(
					any(
						feature = "publisher-imprint-property-schema",
						feature = "bib-schema-section"
					),
					doc
				))]
				PublisherImprint,
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
						feature = "recorded-at-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RecordedAt,
				#[cfg(any(
					any(
						feature = "regions-allowed-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RegionsAllowed,
				#[cfg(any(
					any(
						feature = "released-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ReleasedEvent,
				#[cfg(any(
					any(
						feature = "requires-subscription-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RequiresSubscription,
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
					any(
						feature = "schema-version-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SchemaVersion,
				#[cfg(any(
					any(
						feature = "sd-date-published-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SdDatePublished,
				#[cfg(any(
					any(
						feature = "sd-license-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SdLicense,
				#[cfg(any(
					any(
						feature = "sd-publisher-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SdPublisher,
				#[cfg(any(
					any(
						feature = "sha-256-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Sha256,
				#[cfg(any(
					any(feature = "size-property-schema", feature = "pending-schema-section"),
					doc
				))]
				Size,
				#[cfg(any(
					any(
						feature = "source-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SourceOrganization,
				#[cfg(any(
					any(
						feature = "spatial-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Spatial,
				#[cfg(any(
					any(
						feature = "spatial-coverage-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SpatialCoverage,
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
						feature = "start-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				StartTime,
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
						feature = "teaches-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Teaches,
				#[cfg(any(
					any(
						feature = "temporal-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Temporal,
				#[cfg(any(
					any(
						feature = "temporal-coverage-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				TemporalCoverage,
				#[cfg(any(
					any(feature = "text-property-schema", feature = "general-schema-section"),
					doc
				))]
				Text,
				#[cfg(any(
					any(
						feature = "thumbnail-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Thumbnail,
				#[cfg(any(
					any(
						feature = "thumbnail-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ThumbnailUrl,
				#[cfg(any(
					any(
						feature = "time-required-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				TimeRequired,
				#[cfg(any(
					any(
						feature = "transcript-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Transcript,
				#[cfg(any(
					any(
						feature = "translation-of-work-property-schema",
						feature = "bib-schema-section"
					),
					doc
				))]
				TranslationOfWork,
				#[cfg(any(
					any(
						feature = "translator-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Translator,
				#[cfg(any(
					any(
						feature = "typical-age-range-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				TypicalAgeRange,
				#[cfg(any(
					any(
						feature = "upload-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				UploadDate,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
				#[cfg(any(
					any(
						feature = "usage-info-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				UsageInfo,
				#[cfg(any(
					any(
						feature = "version-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Version,
				#[cfg(any(
					any(feature = "video-property-schema", feature = "general-schema-section"),
					doc
				))]
				Video,
				#[cfg(any(
					any(
						feature = "video-frame-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VideoFrameSize,
				#[cfg(any(
					any(
						feature = "video-quality-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				VideoQuality,
				#[cfg(any(
					any(feature = "width-property-schema", feature = "general-schema-section"),
					doc
				))]
				Width,
				#[cfg(any(
					any(
						feature = "work-example-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				WorkExample,
				#[cfg(any(
					any(
						feature = "work-translation-property-schema",
						feature = "bib-schema-section"
					),
					doc
				))]
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
						#[cfg(any(
							any(
								feature = "about-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"about" => Ok(Field::About),
						#[cfg(any(
							any(
								feature = "abstract-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"abstract" => Ok(Field::Abstract),
						#[cfg(any(
							any(
								feature = "access-mode-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"accessMode" => Ok(Field::AccessMode),
						#[cfg(any(
							any(
								feature = "access-mode-sufficient-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"accessModeSufficient" => Ok(Field::AccessModeSufficient),
						#[cfg(any(
							any(
								feature = "accessibility-api-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"accessibilityAPI" => Ok(Field::AccessibilityApi),
						#[cfg(any(
							any(
								feature = "accessibility-control-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"accessibilityControl" => Ok(Field::AccessibilityControl),
						#[cfg(any(
							any(
								feature = "accessibility-feature-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"accessibilityFeature" => Ok(Field::AccessibilityFeature),
						#[cfg(any(
							any(
								feature = "accessibility-hazard-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"accessibilityHazard" => Ok(Field::AccessibilityHazard),
						#[cfg(any(
							any(
								feature = "accessibility-summary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"accessibilitySummary" => Ok(Field::AccessibilitySummary),
						#[cfg(any(
							any(
								feature = "accountable-person-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"accountablePerson" => Ok(Field::AccountablePerson),
						#[cfg(any(
							any(
								feature = "acquire-license-page-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"acquireLicensePage" => Ok(Field::AcquireLicensePage),
						#[cfg(any(
							any(
								feature = "actor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"actor" => Ok(Field::Actor),
						#[cfg(any(
							any(
								feature = "actors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"actors" => Ok(Field::Actors),
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
								feature = "alternative-headline-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alternativeHeadline" => Ok(Field::AlternativeHeadline),
						#[cfg(any(
							any(
								feature = "archived-at-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"archivedAt" => Ok(Field::ArchivedAt),
						#[cfg(any(
							any(
								feature = "assesses-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"assesses" => Ok(Field::Assesses),
						#[cfg(any(
							any(
								feature = "associated-article-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"associatedArticle" => Ok(Field::AssociatedArticle),
						#[cfg(any(
							any(
								feature = "associated-media-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"associatedMedia" => Ok(Field::AssociatedMedia),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "audio-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"audio" => Ok(Field::Audio),
						#[cfg(any(
							any(
								feature = "author-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"author" => Ok(Field::Author),
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
								feature = "bitrate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"bitrate" => Ok(Field::Bitrate),
						#[cfg(any(
							any(
								feature = "caption-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"caption" => Ok(Field::Caption),
						#[cfg(any(
							any(
								feature = "character-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"character" => Ok(Field::Character),
						#[cfg(any(
							any(
								feature = "citation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"citation" => Ok(Field::Citation),
						#[cfg(any(
							any(
								feature = "comment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"comment" => Ok(Field::Comment),
						#[cfg(any(
							any(
								feature = "comment-count-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"commentCount" => Ok(Field::CommentCount),
						#[cfg(any(
							any(
								feature = "conditions-of-access-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"conditionsOfAccess" => Ok(Field::ConditionsOfAccess),
						#[cfg(any(
							any(
								feature = "content-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contentLocation" => Ok(Field::ContentLocation),
						#[cfg(any(
							any(
								feature = "content-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contentRating" => Ok(Field::ContentRating),
						#[cfg(any(
							any(
								feature = "content-reference-time-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"contentReferenceTime" => Ok(Field::ContentReferenceTime),
						#[cfg(any(
							any(
								feature = "content-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contentSize" => Ok(Field::ContentSize),
						#[cfg(any(
							any(
								feature = "content-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contentUrl" => Ok(Field::ContentUrl),
						#[cfg(any(
							any(
								feature = "contributor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contributor" => Ok(Field::Contributor),
						#[cfg(any(
							any(
								feature = "copyright-holder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"copyrightHolder" => Ok(Field::CopyrightHolder),
						#[cfg(any(
							any(
								feature = "copyright-notice-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"copyrightNotice" => Ok(Field::CopyrightNotice),
						#[cfg(any(
							any(
								feature = "copyright-year-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"copyrightYear" => Ok(Field::CopyrightYear),
						#[cfg(any(
							any(
								feature = "correction-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"correction" => Ok(Field::Correction),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						#[cfg(any(
							any(
								feature = "creative-work-status-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"creativeWorkStatus" => Ok(Field::CreativeWorkStatus),
						#[cfg(any(
							any(
								feature = "creator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"creator" => Ok(Field::Creator),
						#[cfg(any(
							any(
								feature = "credit-text-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"creditText" => Ok(Field::CreditText),
						#[cfg(any(
							any(
								feature = "date-created-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"dateCreated" => Ok(Field::DateCreated),
						#[cfg(any(
							any(
								feature = "date-modified-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"dateModified" => Ok(Field::DateModified),
						#[cfg(any(
							any(
								feature = "date-published-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"datePublished" => Ok(Field::DatePublished),
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
								feature = "director-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"director" => Ok(Field::Director),
						#[cfg(any(
							any(
								feature = "directors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"directors" => Ok(Field::Directors),
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
								feature = "discussion-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"discussionUrl" => Ok(Field::DiscussionUrl),
						#[cfg(any(
							any(
								feature = "duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"duration" => Ok(Field::Duration),
						#[cfg(any(
							any(
								feature = "edit-eidr-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"editEIDR" => Ok(Field::EditEidr),
						#[cfg(any(
							any(
								feature = "editor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"editor" => Ok(Field::Editor),
						#[cfg(any(
							any(
								feature = "educational-alignment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"educationalAlignment" => Ok(Field::EducationalAlignment),
						#[cfg(any(
							any(
								feature = "educational-level-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"educationalLevel" => Ok(Field::EducationalLevel),
						#[cfg(any(
							any(
								feature = "educational-use-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"educationalUse" => Ok(Field::EducationalUse),
						#[cfg(any(
							any(
								feature = "embed-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"embedUrl" => Ok(Field::EmbedUrl),
						#[cfg(any(
							any(
								feature = "embedded-text-caption-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"embeddedTextCaption" => Ok(Field::EmbeddedTextCaption),
						#[cfg(any(
							any(
								feature = "encodes-creative-work-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"encodesCreativeWork" => Ok(Field::EncodesCreativeWork),
						#[cfg(any(
							any(
								feature = "encoding-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"encoding" => Ok(Field::Encoding),
						#[cfg(any(
							any(
								feature = "encoding-format-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"encodingFormat" => Ok(Field::EncodingFormat),
						#[cfg(any(
							any(
								feature = "encodings-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"encodings" => Ok(Field::Encodings),
						#[cfg(any(
							any(
								feature = "end-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"endTime" => Ok(Field::EndTime),
						#[cfg(any(
							any(
								feature = "example-of-work-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"exampleOfWork" => Ok(Field::ExampleOfWork),
						#[cfg(any(
							any(
								feature = "expires-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"expires" => Ok(Field::Expires),
						#[cfg(any(
							any(
								feature = "file-format-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"fileFormat" => Ok(Field::FileFormat),
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
								feature = "genre-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"genre" => Ok(Field::Genre),
						#[cfg(any(
							any(
								feature = "has-part-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"hasPart" => Ok(Field::HasPart),
						#[cfg(any(
							any(
								feature = "headline-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"headline" => Ok(Field::Headline),
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
								feature = "in-language-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"inLanguage" => Ok(Field::InLanguage),
						#[cfg(any(
							any(
								feature = "ineligible-region-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"ineligibleRegion" => Ok(Field::IneligibleRegion),
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
								feature = "interactivity-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"interactivityType" => Ok(Field::InteractivityType),
						#[cfg(any(
							any(
								feature = "interpreted-as-claim-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"interpretedAsClaim" => Ok(Field::InterpretedAsClaim),
						#[cfg(any(
							any(
								feature = "is-accessible-for-free-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						#[cfg(any(
							any(
								feature = "is-based-on-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isBasedOn" => Ok(Field::IsBasedOn),
						#[cfg(any(
							any(
								feature = "is-based-on-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isBasedOnUrl" => Ok(Field::IsBasedOnUrl),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						#[cfg(any(
							any(
								feature = "is-part-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isPartOf" => Ok(Field::IsPartOf),
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
								feature = "learning-resource-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"learningResourceType" => Ok(Field::LearningResourceType),
						#[cfg(any(
							any(
								feature = "license-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"license" => Ok(Field::License),
						#[cfg(any(
							any(
								feature = "location-created-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"locationCreated" => Ok(Field::LocationCreated),
						#[cfg(any(
							any(
								feature = "main-entity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mainEntity" => Ok(Field::MainEntity),
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
								feature = "maintainer-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"maintainer" => Ok(Field::Maintainer),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"material" => Ok(Field::Material),
						#[cfg(any(
							any(
								feature = "material-extent-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"materialExtent" => Ok(Field::MaterialExtent),
						#[cfg(any(
							any(
								feature = "mentions-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mentions" => Ok(Field::Mentions),
						#[cfg(any(
							any(
								feature = "music-by-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"musicBy" => Ok(Field::MusicBy),
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
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"offers" => Ok(Field::Offers),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"pattern" => Ok(Field::Pattern),
						#[cfg(any(
							any(
								feature = "player-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"playerType" => Ok(Field::PlayerType),
						#[cfg(any(
							any(
								feature = "position-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"position" => Ok(Field::Position),
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
								feature = "producer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"producer" => Ok(Field::Producer),
						#[cfg(any(
							any(
								feature = "production-company-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"productionCompany" => Ok(Field::ProductionCompany),
						#[cfg(any(
							any(
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"provider" => Ok(Field::Provider),
						#[cfg(any(
							any(
								feature = "publication-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"publication" => Ok(Field::Publication),
						#[cfg(any(
							any(
								feature = "publisher-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"publisher" => Ok(Field::Publisher),
						#[cfg(any(
							any(
								feature = "publisher-imprint-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
						"publisherImprint" => Ok(Field::PublisherImprint),
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
								feature = "recorded-at-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"recordedAt" => Ok(Field::RecordedAt),
						#[cfg(any(
							any(
								feature = "regions-allowed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"regionsAllowed" => Ok(Field::RegionsAllowed),
						#[cfg(any(
							any(
								feature = "released-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"releasedEvent" => Ok(Field::ReleasedEvent),
						#[cfg(any(
							any(
								feature = "requires-subscription-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"requiresSubscription" => Ok(Field::RequiresSubscription),
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
								feature = "schema-version-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"schemaVersion" => Ok(Field::SchemaVersion),
						#[cfg(any(
							any(
								feature = "sd-date-published-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"sdDatePublished" => Ok(Field::SdDatePublished),
						#[cfg(any(
							any(
								feature = "sd-license-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"sdLicense" => Ok(Field::SdLicense),
						#[cfg(any(
							any(
								feature = "sd-publisher-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"sdPublisher" => Ok(Field::SdPublisher),
						#[cfg(any(
							any(
								feature = "sha-256-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"sha256" => Ok(Field::Sha256),
						#[cfg(any(
							any(
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"size" => Ok(Field::Size),
						#[cfg(any(
							any(
								feature = "source-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sourceOrganization" => Ok(Field::SourceOrganization),
						#[cfg(any(
							any(
								feature = "spatial-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"spatial" => Ok(Field::Spatial),
						#[cfg(any(
							any(
								feature = "spatial-coverage-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"spatialCoverage" => Ok(Field::SpatialCoverage),
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
								feature = "start-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"startTime" => Ok(Field::StartTime),
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
								feature = "teaches-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"teaches" => Ok(Field::Teaches),
						#[cfg(any(
							any(
								feature = "temporal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"temporal" => Ok(Field::Temporal),
						#[cfg(any(
							any(
								feature = "temporal-coverage-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"temporalCoverage" => Ok(Field::TemporalCoverage),
						#[cfg(any(
							any(
								feature = "text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"text" => Ok(Field::Text),
						#[cfg(any(
							any(
								feature = "thumbnail-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"thumbnail" => Ok(Field::Thumbnail),
						#[cfg(any(
							any(
								feature = "thumbnail-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"thumbnailUrl" => Ok(Field::ThumbnailUrl),
						#[cfg(any(
							any(
								feature = "time-required-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"timeRequired" => Ok(Field::TimeRequired),
						#[cfg(any(
							any(
								feature = "transcript-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"transcript" => Ok(Field::Transcript),
						#[cfg(any(
							any(
								feature = "translation-of-work-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
						"translationOfWork" => Ok(Field::TranslationOfWork),
						#[cfg(any(
							any(
								feature = "translator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"translator" => Ok(Field::Translator),
						#[cfg(any(
							any(
								feature = "typical-age-range-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"typicalAgeRange" => Ok(Field::TypicalAgeRange),
						#[cfg(any(
							any(
								feature = "upload-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"uploadDate" => Ok(Field::UploadDate),
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
								feature = "usage-info-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"usageInfo" => Ok(Field::UsageInfo),
						#[cfg(any(
							any(
								feature = "version-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"version" => Ok(Field::Version),
						#[cfg(any(
							any(
								feature = "video-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"video" => Ok(Field::Video),
						#[cfg(any(
							any(
								feature = "video-frame-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"videoFrameSize" => Ok(Field::VideoFrameSize),
						#[cfg(any(
							any(
								feature = "video-quality-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"videoQuality" => Ok(Field::VideoQuality),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"width" => Ok(Field::Width),
						#[cfg(any(
							any(
								feature = "work-example-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"workExample" => Ok(Field::WorkExample),
						#[cfg(any(
							any(
								feature = "work-translation-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
						"workTranslation" => Ok(Field::WorkTranslation),
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
								feature = "about-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"about" => Ok(Field::About),
						#[cfg(any(
							any(
								feature = "abstract-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"abstract" => Ok(Field::Abstract),
						#[cfg(any(
							any(
								feature = "access-mode-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"accessMode" => Ok(Field::AccessMode),
						#[cfg(any(
							any(
								feature = "access-mode-sufficient-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"accessModeSufficient" => Ok(Field::AccessModeSufficient),
						#[cfg(any(
							any(
								feature = "accessibility-api-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"accessibilityAPI" => Ok(Field::AccessibilityApi),
						#[cfg(any(
							any(
								feature = "accessibility-control-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"accessibilityControl" => Ok(Field::AccessibilityControl),
						#[cfg(any(
							any(
								feature = "accessibility-feature-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"accessibilityFeature" => Ok(Field::AccessibilityFeature),
						#[cfg(any(
							any(
								feature = "accessibility-hazard-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"accessibilityHazard" => Ok(Field::AccessibilityHazard),
						#[cfg(any(
							any(
								feature = "accessibility-summary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"accessibilitySummary" => Ok(Field::AccessibilitySummary),
						#[cfg(any(
							any(
								feature = "accountable-person-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"accountablePerson" => Ok(Field::AccountablePerson),
						#[cfg(any(
							any(
								feature = "acquire-license-page-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"acquireLicensePage" => Ok(Field::AcquireLicensePage),
						#[cfg(any(
							any(
								feature = "actor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"actor" => Ok(Field::Actor),
						#[cfg(any(
							any(
								feature = "actors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"actors" => Ok(Field::Actors),
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
								feature = "alternative-headline-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alternativeHeadline" => Ok(Field::AlternativeHeadline),
						#[cfg(any(
							any(
								feature = "archived-at-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"archivedAt" => Ok(Field::ArchivedAt),
						#[cfg(any(
							any(
								feature = "assesses-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"assesses" => Ok(Field::Assesses),
						#[cfg(any(
							any(
								feature = "associated-article-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"associatedArticle" => Ok(Field::AssociatedArticle),
						#[cfg(any(
							any(
								feature = "associated-media-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"associatedMedia" => Ok(Field::AssociatedMedia),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "audio-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"audio" => Ok(Field::Audio),
						#[cfg(any(
							any(
								feature = "author-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"author" => Ok(Field::Author),
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
								feature = "bitrate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"bitrate" => Ok(Field::Bitrate),
						#[cfg(any(
							any(
								feature = "caption-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"caption" => Ok(Field::Caption),
						#[cfg(any(
							any(
								feature = "character-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"character" => Ok(Field::Character),
						#[cfg(any(
							any(
								feature = "citation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"citation" => Ok(Field::Citation),
						#[cfg(any(
							any(
								feature = "comment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"comment" => Ok(Field::Comment),
						#[cfg(any(
							any(
								feature = "comment-count-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"commentCount" => Ok(Field::CommentCount),
						#[cfg(any(
							any(
								feature = "conditions-of-access-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"conditionsOfAccess" => Ok(Field::ConditionsOfAccess),
						#[cfg(any(
							any(
								feature = "content-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contentLocation" => Ok(Field::ContentLocation),
						#[cfg(any(
							any(
								feature = "content-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contentRating" => Ok(Field::ContentRating),
						#[cfg(any(
							any(
								feature = "content-reference-time-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"contentReferenceTime" => Ok(Field::ContentReferenceTime),
						#[cfg(any(
							any(
								feature = "content-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contentSize" => Ok(Field::ContentSize),
						#[cfg(any(
							any(
								feature = "content-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contentUrl" => Ok(Field::ContentUrl),
						#[cfg(any(
							any(
								feature = "contributor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contributor" => Ok(Field::Contributor),
						#[cfg(any(
							any(
								feature = "copyright-holder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"copyrightHolder" => Ok(Field::CopyrightHolder),
						#[cfg(any(
							any(
								feature = "copyright-notice-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"copyrightNotice" => Ok(Field::CopyrightNotice),
						#[cfg(any(
							any(
								feature = "copyright-year-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"copyrightYear" => Ok(Field::CopyrightYear),
						#[cfg(any(
							any(
								feature = "correction-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"correction" => Ok(Field::Correction),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"countryOfOrigin" => Ok(Field::CountryOfOrigin),
						#[cfg(any(
							any(
								feature = "creative-work-status-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"creativeWorkStatus" => Ok(Field::CreativeWorkStatus),
						#[cfg(any(
							any(
								feature = "creator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"creator" => Ok(Field::Creator),
						#[cfg(any(
							any(
								feature = "credit-text-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"creditText" => Ok(Field::CreditText),
						#[cfg(any(
							any(
								feature = "date-created-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"dateCreated" => Ok(Field::DateCreated),
						#[cfg(any(
							any(
								feature = "date-modified-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"dateModified" => Ok(Field::DateModified),
						#[cfg(any(
							any(
								feature = "date-published-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"datePublished" => Ok(Field::DatePublished),
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
								feature = "director-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"director" => Ok(Field::Director),
						#[cfg(any(
							any(
								feature = "directors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"directors" => Ok(Field::Directors),
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
								feature = "discussion-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"discussionUrl" => Ok(Field::DiscussionUrl),
						#[cfg(any(
							any(
								feature = "duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"duration" => Ok(Field::Duration),
						#[cfg(any(
							any(
								feature = "edit-eidr-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"editEIDR" => Ok(Field::EditEidr),
						#[cfg(any(
							any(
								feature = "editor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"editor" => Ok(Field::Editor),
						#[cfg(any(
							any(
								feature = "educational-alignment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"educationalAlignment" => Ok(Field::EducationalAlignment),
						#[cfg(any(
							any(
								feature = "educational-level-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"educationalLevel" => Ok(Field::EducationalLevel),
						#[cfg(any(
							any(
								feature = "educational-use-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"educationalUse" => Ok(Field::EducationalUse),
						#[cfg(any(
							any(
								feature = "embed-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"embedUrl" => Ok(Field::EmbedUrl),
						#[cfg(any(
							any(
								feature = "embedded-text-caption-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"embeddedTextCaption" => Ok(Field::EmbeddedTextCaption),
						#[cfg(any(
							any(
								feature = "encodes-creative-work-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"encodesCreativeWork" => Ok(Field::EncodesCreativeWork),
						#[cfg(any(
							any(
								feature = "encoding-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"encoding" => Ok(Field::Encoding),
						#[cfg(any(
							any(
								feature = "encoding-format-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"encodingFormat" => Ok(Field::EncodingFormat),
						#[cfg(any(
							any(
								feature = "encodings-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"encodings" => Ok(Field::Encodings),
						#[cfg(any(
							any(
								feature = "end-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"endTime" => Ok(Field::EndTime),
						#[cfg(any(
							any(
								feature = "example-of-work-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"exampleOfWork" => Ok(Field::ExampleOfWork),
						#[cfg(any(
							any(
								feature = "expires-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"expires" => Ok(Field::Expires),
						#[cfg(any(
							any(
								feature = "file-format-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"fileFormat" => Ok(Field::FileFormat),
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
								feature = "genre-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"genre" => Ok(Field::Genre),
						#[cfg(any(
							any(
								feature = "has-part-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"hasPart" => Ok(Field::HasPart),
						#[cfg(any(
							any(
								feature = "headline-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"headline" => Ok(Field::Headline),
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
								feature = "in-language-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"inLanguage" => Ok(Field::InLanguage),
						#[cfg(any(
							any(
								feature = "ineligible-region-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"ineligibleRegion" => Ok(Field::IneligibleRegion),
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
								feature = "interactivity-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"interactivityType" => Ok(Field::InteractivityType),
						#[cfg(any(
							any(
								feature = "interpreted-as-claim-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"interpretedAsClaim" => Ok(Field::InterpretedAsClaim),
						#[cfg(any(
							any(
								feature = "is-accessible-for-free-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						#[cfg(any(
							any(
								feature = "is-based-on-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isBasedOn" => Ok(Field::IsBasedOn),
						#[cfg(any(
							any(
								feature = "is-based-on-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isBasedOnUrl" => Ok(Field::IsBasedOnUrl),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						#[cfg(any(
							any(
								feature = "is-part-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isPartOf" => Ok(Field::IsPartOf),
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
								feature = "learning-resource-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"learningResourceType" => Ok(Field::LearningResourceType),
						#[cfg(any(
							any(
								feature = "license-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"license" => Ok(Field::License),
						#[cfg(any(
							any(
								feature = "location-created-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"locationCreated" => Ok(Field::LocationCreated),
						#[cfg(any(
							any(
								feature = "main-entity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mainEntity" => Ok(Field::MainEntity),
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
								feature = "maintainer-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"maintainer" => Ok(Field::Maintainer),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"material" => Ok(Field::Material),
						#[cfg(any(
							any(
								feature = "material-extent-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"materialExtent" => Ok(Field::MaterialExtent),
						#[cfg(any(
							any(
								feature = "mentions-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mentions" => Ok(Field::Mentions),
						#[cfg(any(
							any(
								feature = "music-by-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"musicBy" => Ok(Field::MusicBy),
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
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"offers" => Ok(Field::Offers),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"pattern" => Ok(Field::Pattern),
						#[cfg(any(
							any(
								feature = "player-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"playerType" => Ok(Field::PlayerType),
						#[cfg(any(
							any(
								feature = "position-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"position" => Ok(Field::Position),
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
								feature = "producer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"producer" => Ok(Field::Producer),
						#[cfg(any(
							any(
								feature = "production-company-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"productionCompany" => Ok(Field::ProductionCompany),
						#[cfg(any(
							any(
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"provider" => Ok(Field::Provider),
						#[cfg(any(
							any(
								feature = "publication-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"publication" => Ok(Field::Publication),
						#[cfg(any(
							any(
								feature = "publisher-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"publisher" => Ok(Field::Publisher),
						#[cfg(any(
							any(
								feature = "publisher-imprint-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
						b"publisherImprint" => Ok(Field::PublisherImprint),
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
								feature = "recorded-at-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"recordedAt" => Ok(Field::RecordedAt),
						#[cfg(any(
							any(
								feature = "regions-allowed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"regionsAllowed" => Ok(Field::RegionsAllowed),
						#[cfg(any(
							any(
								feature = "released-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"releasedEvent" => Ok(Field::ReleasedEvent),
						#[cfg(any(
							any(
								feature = "requires-subscription-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"requiresSubscription" => Ok(Field::RequiresSubscription),
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
								feature = "schema-version-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"schemaVersion" => Ok(Field::SchemaVersion),
						#[cfg(any(
							any(
								feature = "sd-date-published-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"sdDatePublished" => Ok(Field::SdDatePublished),
						#[cfg(any(
							any(
								feature = "sd-license-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"sdLicense" => Ok(Field::SdLicense),
						#[cfg(any(
							any(
								feature = "sd-publisher-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"sdPublisher" => Ok(Field::SdPublisher),
						#[cfg(any(
							any(
								feature = "sha-256-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"sha256" => Ok(Field::Sha256),
						#[cfg(any(
							any(
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"size" => Ok(Field::Size),
						#[cfg(any(
							any(
								feature = "source-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sourceOrganization" => Ok(Field::SourceOrganization),
						#[cfg(any(
							any(
								feature = "spatial-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"spatial" => Ok(Field::Spatial),
						#[cfg(any(
							any(
								feature = "spatial-coverage-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"spatialCoverage" => Ok(Field::SpatialCoverage),
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
								feature = "start-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"startTime" => Ok(Field::StartTime),
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
								feature = "teaches-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"teaches" => Ok(Field::Teaches),
						#[cfg(any(
							any(
								feature = "temporal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"temporal" => Ok(Field::Temporal),
						#[cfg(any(
							any(
								feature = "temporal-coverage-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"temporalCoverage" => Ok(Field::TemporalCoverage),
						#[cfg(any(
							any(
								feature = "text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"text" => Ok(Field::Text),
						#[cfg(any(
							any(
								feature = "thumbnail-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"thumbnail" => Ok(Field::Thumbnail),
						#[cfg(any(
							any(
								feature = "thumbnail-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"thumbnailUrl" => Ok(Field::ThumbnailUrl),
						#[cfg(any(
							any(
								feature = "time-required-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"timeRequired" => Ok(Field::TimeRequired),
						#[cfg(any(
							any(
								feature = "transcript-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"transcript" => Ok(Field::Transcript),
						#[cfg(any(
							any(
								feature = "translation-of-work-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
						b"translationOfWork" => Ok(Field::TranslationOfWork),
						#[cfg(any(
							any(
								feature = "translator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"translator" => Ok(Field::Translator),
						#[cfg(any(
							any(
								feature = "typical-age-range-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"typicalAgeRange" => Ok(Field::TypicalAgeRange),
						#[cfg(any(
							any(
								feature = "upload-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"uploadDate" => Ok(Field::UploadDate),
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
								feature = "usage-info-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"usageInfo" => Ok(Field::UsageInfo),
						#[cfg(any(
							any(
								feature = "version-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"version" => Ok(Field::Version),
						#[cfg(any(
							any(
								feature = "video-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"video" => Ok(Field::Video),
						#[cfg(any(
							any(
								feature = "video-frame-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"videoFrameSize" => Ok(Field::VideoFrameSize),
						#[cfg(any(
							any(
								feature = "video-quality-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"videoQuality" => Ok(Field::VideoQuality),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"width" => Ok(Field::Width),
						#[cfg(any(
							any(
								feature = "work-example-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"workExample" => Ok(Field::WorkExample),
						#[cfg(any(
							any(
								feature = "work-translation-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
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
				type Value = VideoObjectSnapshot;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema VideoObjectSnapshot")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(feature = "about-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#about_property = None;
					#[cfg(any(
						any(
							feature = "abstract-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#abstract_property = None;
					#[cfg(any(
						any(
							feature = "access-mode-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#access_mode_property = None;
					#[cfg(any(
						any(
							feature = "access-mode-sufficient-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#access_mode_sufficient_property = None;
					#[cfg(any(
						any(
							feature = "accessibility-api-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#accessibility_api_property = None;
					#[cfg(any(
						any(
							feature = "accessibility-control-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#accessibility_control_property = None;
					#[cfg(any(
						any(
							feature = "accessibility-feature-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#accessibility_feature_property = None;
					#[cfg(any(
						any(
							feature = "accessibility-hazard-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#accessibility_hazard_property = None;
					#[cfg(any(
						any(
							feature = "accessibility-summary-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#accessibility_summary_property = None;
					#[cfg(any(
						any(
							feature = "accountable-person-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#accountable_person_property = None;
					#[cfg(any(
						any(
							feature = "acquire-license-page-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#acquire_license_page_property = None;
					#[cfg(any(
						any(feature = "actor-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#actor_property = None;
					#[cfg(any(
						any(
							feature = "actors-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#actors_property = None;
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
							feature = "alternative-headline-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alternative_headline_property = None;
					#[cfg(any(
						any(
							feature = "archived-at-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#archived_at_property = None;
					#[cfg(any(
						any(
							feature = "assesses-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#assesses_property = None;
					#[cfg(any(
						any(
							feature = "associated-article-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#associated_article_property = None;
					#[cfg(any(
						any(
							feature = "associated-media-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#associated_media_property = None;
					#[cfg(any(
						any(
							feature = "audience-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#audience_property = None;
					#[cfg(any(
						any(feature = "audio-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#audio_property = None;
					#[cfg(any(
						any(
							feature = "author-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#author_property = None;
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
							feature = "bitrate-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#bitrate_property = None;
					#[cfg(any(
						any(
							feature = "caption-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#caption_property = None;
					#[cfg(any(
						any(
							feature = "character-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#character_property = None;
					#[cfg(any(
						any(
							feature = "citation-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#citation_property = None;
					#[cfg(any(
						any(
							feature = "comment-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#comment_property = None;
					#[cfg(any(
						any(
							feature = "comment-count-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#comment_count_property = None;
					#[cfg(any(
						any(
							feature = "conditions-of-access-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#conditions_of_access_property = None;
					#[cfg(any(
						any(
							feature = "content-location-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#content_location_property = None;
					#[cfg(any(
						any(
							feature = "content-rating-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#content_rating_property = None;
					#[cfg(any(
						any(
							feature = "content-reference-time-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#content_reference_time_property = None;
					#[cfg(any(
						any(
							feature = "content-size-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#content_size_property = None;
					#[cfg(any(
						any(
							feature = "content-url-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#content_url_property = None;
					#[cfg(any(
						any(
							feature = "contributor-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#contributor_property = None;
					#[cfg(any(
						any(
							feature = "copyright-holder-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#copyright_holder_property = None;
					#[cfg(any(
						any(
							feature = "copyright-notice-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#copyright_notice_property = None;
					#[cfg(any(
						any(
							feature = "copyright-year-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#copyright_year_property = None;
					#[cfg(any(
						any(
							feature = "correction-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#correction_property = None;
					#[cfg(any(
						any(
							feature = "country-of-origin-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#country_of_origin_property = None;
					#[cfg(any(
						any(
							feature = "creative-work-status-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#creative_work_status_property = None;
					#[cfg(any(
						any(
							feature = "creator-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#creator_property = None;
					#[cfg(any(
						any(
							feature = "credit-text-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#credit_text_property = None;
					#[cfg(any(
						any(
							feature = "date-created-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#date_created_property = None;
					#[cfg(any(
						any(
							feature = "date-modified-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#date_modified_property = None;
					#[cfg(any(
						any(
							feature = "date-published-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#date_published_property = None;
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
							feature = "director-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#director_property = None;
					#[cfg(any(
						any(
							feature = "directors-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#directors_property = None;
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
							feature = "discussion-url-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#discussion_url_property = None;
					#[cfg(any(
						any(
							feature = "duration-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#duration_property = None;
					#[cfg(any(
						any(
							feature = "edit-eidr-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#edit_eidr_property = None;
					#[cfg(any(
						any(
							feature = "editor-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#editor_property = None;
					#[cfg(any(
						any(
							feature = "educational-alignment-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#educational_alignment_property = None;
					#[cfg(any(
						any(
							feature = "educational-level-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#educational_level_property = None;
					#[cfg(any(
						any(
							feature = "educational-use-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#educational_use_property = None;
					#[cfg(any(
						any(
							feature = "embed-url-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#embed_url_property = None;
					#[cfg(any(
						any(
							feature = "embedded-text-caption-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#embedded_text_caption_property = None;
					#[cfg(any(
						any(
							feature = "encodes-creative-work-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#encodes_creative_work_property = None;
					#[cfg(any(
						any(
							feature = "encoding-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#encoding_property = None;
					#[cfg(any(
						any(
							feature = "encoding-format-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#encoding_format_property = None;
					#[cfg(any(
						any(
							feature = "encodings-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#encodings_property = None;
					#[cfg(any(
						any(
							feature = "end-time-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#end_time_property = None;
					#[cfg(any(
						any(
							feature = "example-of-work-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#example_of_work_property = None;
					#[cfg(any(
						any(
							feature = "expires-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#expires_property = None;
					#[cfg(any(
						any(
							feature = "file-format-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#file_format_property = None;
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
						any(feature = "genre-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#genre_property = None;
					#[cfg(any(
						any(
							feature = "has-part-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#has_part_property = None;
					#[cfg(any(
						any(
							feature = "headline-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#headline_property = None;
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
							feature = "in-language-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#in_language_property = None;
					#[cfg(any(
						any(
							feature = "ineligible-region-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#ineligible_region_property = None;
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
							feature = "interactivity-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#interactivity_type_property = None;
					#[cfg(any(
						any(
							feature = "interpreted-as-claim-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#interpreted_as_claim_property = None;
					#[cfg(any(
						any(
							feature = "is-accessible-for-free-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_accessible_for_free_property = None;
					#[cfg(any(
						any(
							feature = "is-based-on-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_based_on_property = None;
					#[cfg(any(
						any(
							feature = "is-based-on-url-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_based_on_url_property = None;
					#[cfg(any(
						any(
							feature = "is-family-friendly-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_family_friendly_property = None;
					#[cfg(any(
						any(
							feature = "is-part-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_part_of_property = None;
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
							feature = "learning-resource-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#learning_resource_type_property = None;
					#[cfg(any(
						any(
							feature = "license-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#license_property = None;
					#[cfg(any(
						any(
							feature = "location-created-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#location_created_property = None;
					#[cfg(any(
						any(
							feature = "main-entity-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#main_entity_property = None;
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
							feature = "maintainer-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#maintainer_property = None;
					#[cfg(any(
						any(
							feature = "material-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#material_property = None;
					#[cfg(any(
						any(
							feature = "material-extent-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#material_extent_property = None;
					#[cfg(any(
						any(
							feature = "mentions-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#mentions_property = None;
					#[cfg(any(
						any(
							feature = "music-by-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#music_by_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "offers-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#offers_property = None;
					#[cfg(any(
						any(
							feature = "pattern-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#pattern_property = None;
					#[cfg(any(
						any(
							feature = "player-type-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#player_type_property = None;
					#[cfg(any(
						any(
							feature = "position-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#position_property = None;
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
							feature = "producer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#producer_property = None;
					#[cfg(any(
						any(
							feature = "production-company-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#production_company_property = None;
					#[cfg(any(
						any(
							feature = "provider-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#provider_property = None;
					#[cfg(any(
						any(
							feature = "publication-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#publication_property = None;
					#[cfg(any(
						any(
							feature = "publisher-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#publisher_property = None;
					#[cfg(any(
						any(
							feature = "publisher-imprint-property-schema",
							feature = "bib-schema-section"
						),
						doc
					))]
					let mut r#publisher_imprint_property = None;
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
							feature = "recorded-at-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#recorded_at_property = None;
					#[cfg(any(
						any(
							feature = "regions-allowed-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#regions_allowed_property = None;
					#[cfg(any(
						any(
							feature = "released-event-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#released_event_property = None;
					#[cfg(any(
						any(
							feature = "requires-subscription-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#requires_subscription_property = None;
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
						any(
							feature = "schema-version-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#schema_version_property = None;
					#[cfg(any(
						any(
							feature = "sd-date-published-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#sd_date_published_property = None;
					#[cfg(any(
						any(
							feature = "sd-license-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#sd_license_property = None;
					#[cfg(any(
						any(
							feature = "sd-publisher-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#sd_publisher_property = None;
					#[cfg(any(
						any(
							feature = "sha-256-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#sha_256_property = None;
					#[cfg(any(
						any(feature = "size-property-schema", feature = "pending-schema-section"),
						doc
					))]
					let mut r#size_property = None;
					#[cfg(any(
						any(
							feature = "source-organization-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#source_organization_property = None;
					#[cfg(any(
						any(
							feature = "spatial-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#spatial_property = None;
					#[cfg(any(
						any(
							feature = "spatial-coverage-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#spatial_coverage_property = None;
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
							feature = "start-time-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#start_time_property = None;
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
							feature = "teaches-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#teaches_property = None;
					#[cfg(any(
						any(
							feature = "temporal-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#temporal_property = None;
					#[cfg(any(
						any(
							feature = "temporal-coverage-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#temporal_coverage_property = None;
					#[cfg(any(
						any(feature = "text-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#text_property = None;
					#[cfg(any(
						any(
							feature = "thumbnail-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#thumbnail_property = None;
					#[cfg(any(
						any(
							feature = "thumbnail-url-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#thumbnail_url_property = None;
					#[cfg(any(
						any(
							feature = "time-required-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#time_required_property = None;
					#[cfg(any(
						any(
							feature = "transcript-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#transcript_property = None;
					#[cfg(any(
						any(
							feature = "translation-of-work-property-schema",
							feature = "bib-schema-section"
						),
						doc
					))]
					let mut r#translation_of_work_property = None;
					#[cfg(any(
						any(
							feature = "translator-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#translator_property = None;
					#[cfg(any(
						any(
							feature = "typical-age-range-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#typical_age_range_property = None;
					#[cfg(any(
						any(
							feature = "upload-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#upload_date_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					#[cfg(any(
						any(
							feature = "usage-info-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#usage_info_property = None;
					#[cfg(any(
						any(
							feature = "version-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#version_property = None;
					#[cfg(any(
						any(feature = "video-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#video_property = None;
					#[cfg(any(
						any(
							feature = "video-frame-size-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#video_frame_size_property = None;
					#[cfg(any(
						any(
							feature = "video-quality-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#video_quality_property = None;
					#[cfg(any(
						any(feature = "width-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#width_property = None;
					#[cfg(any(
						any(
							feature = "work-example-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#work_example_property = None;
					#[cfg(any(
						any(
							feature = "work-translation-property-schema",
							feature = "bib-schema-section"
						),
						doc
					))]
					let mut r#work_translation_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "about-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "abstract-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "access-mode-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "access-mode-sufficient-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "accessibility-api-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "accessibility-control-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "accessibility-feature-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "accessibility-hazard-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "accessibility-summary-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "accountable-person-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "acquire-license-page-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "actor-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Actor => {
								if r#actor_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("actor"));
								}
								r#actor_property = Some({
									struct DeserializeWith(Vec<ActorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "actors-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Actors => {
								if r#actors_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("actors"));
								}
								r#actors_property = Some({
									struct DeserializeWith(Vec<ActorsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "alternative-headline-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "archived-at-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "assesses-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "associated-article-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::AssociatedArticle => {
								if r#associated_article_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"associatedArticle",
									));
								}
								r#associated_article_property = Some({
									struct DeserializeWith(Vec<AssociatedArticleProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "associated-media-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "audience-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "audio-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "author-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "bitrate-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Bitrate => {
								if r#bitrate_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"bitrate",
									));
								}
								r#bitrate_property = Some({
									struct DeserializeWith(Vec<BitrateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "caption-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Caption => {
								if r#caption_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"caption",
									));
								}
								r#caption_property = Some({
									struct DeserializeWith(Vec<CaptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "character-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "citation-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "comment-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "comment-count-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "conditions-of-access-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "content-location-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "content-rating-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "content-reference-time-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "content-size-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ContentSize => {
								if r#content_size_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contentSize",
									));
								}
								r#content_size_property = Some({
									struct DeserializeWith(Vec<ContentSizeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "content-url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ContentUrl => {
								if r#content_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contentUrl",
									));
								}
								r#content_url_property = Some({
									struct DeserializeWith(Vec<ContentUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "contributor-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "copyright-holder-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "copyright-notice-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "copyright-year-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "correction-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "country-of-origin-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "creative-work-status-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "creator-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "credit-text-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "date-created-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "date-modified-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "date-published-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "director-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Director => {
								if r#director_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"director",
									));
								}
								r#director_property = Some({
									struct DeserializeWith(Vec<DirectorProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "directors-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Directors => {
								if r#directors_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"directors",
									));
								}
								r#directors_property = Some({
									struct DeserializeWith(Vec<DirectorsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "discussion-url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "duration-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Duration => {
								if r#duration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"duration",
									));
								}
								r#duration_property = Some({
									struct DeserializeWith(Vec<DurationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "edit-eidr-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "editor-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "educational-alignment-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "educational-level-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "educational-use-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "embed-url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EmbedUrl => {
								if r#embed_url_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"embedUrl",
									));
								}
								r#embed_url_property = Some({
									struct DeserializeWith(Vec<EmbedUrlProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "embedded-text-caption-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::EmbeddedTextCaption => {
								if r#embedded_text_caption_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"embeddedTextCaption",
									));
								}
								r#embedded_text_caption_property = Some({
									struct DeserializeWith(Vec<EmbeddedTextCaptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "encodes-creative-work-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EncodesCreativeWork => {
								if r#encodes_creative_work_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"encodesCreativeWork",
									));
								}
								r#encodes_creative_work_property = Some({
									struct DeserializeWith(Vec<EncodesCreativeWorkProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "encoding-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "encoding-format-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "encodings-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "end-time-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EndTime => {
								if r#end_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"endTime",
									));
								}
								r#end_time_property = Some({
									struct DeserializeWith(Vec<EndTimeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "example-of-work-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "expires-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "file-format-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "genre-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "has-part-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "headline-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "in-language-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "ineligible-region-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::IneligibleRegion => {
								if r#ineligible_region_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"ineligibleRegion",
									));
								}
								r#ineligible_region_property = Some({
									struct DeserializeWith(Vec<IneligibleRegionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "interactivity-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "interpreted-as-claim-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-accessible-for-free-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-based-on-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-based-on-url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-family-friendly-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-part-of-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "learning-resource-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "license-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "location-created-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "main-entity-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "maintainer-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "material-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "material-extent-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "mentions-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "music-by-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MusicBy => {
								if r#music_by_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"musicBy",
									));
								}
								r#music_by_property = Some({
									struct DeserializeWith(Vec<MusicByProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "offers-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "pattern-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "player-type-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::PlayerType => {
								if r#player_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"playerType",
									));
								}
								r#player_type_property = Some({
									struct DeserializeWith(Vec<PlayerTypeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "position-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "producer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "production-company-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ProductionCompany => {
								if r#production_company_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"productionCompany",
									));
								}
								r#production_company_property = Some({
									struct DeserializeWith(Vec<ProductionCompanyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "provider-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "publication-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "publisher-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "publisher-imprint-property-schema",
									feature = "bib-schema-section"
								),
								doc
							))]
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
									feature = "recorded-at-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "regions-allowed-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::RegionsAllowed => {
								if r#regions_allowed_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"regionsAllowed",
									));
								}
								r#regions_allowed_property = Some({
									struct DeserializeWith(Vec<RegionsAllowedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "released-event-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "requires-subscription-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::RequiresSubscription => {
								if r#requires_subscription_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"requiresSubscription",
									));
								}
								r#requires_subscription_property = Some({
									struct DeserializeWith(Vec<RequiresSubscriptionProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "schema-version-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "sd-date-published-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "sd-license-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "sd-publisher-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "sha-256-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Sha256 => {
								if r#sha_256_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("sha256"));
								}
								r#sha_256_property = Some({
									struct DeserializeWith(Vec<Sha256Property>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "size-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "source-organization-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "spatial-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "spatial-coverage-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "start-time-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::StartTime => {
								if r#start_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"startTime",
									));
								}
								r#start_time_property = Some({
									struct DeserializeWith(Vec<StartTimeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "teaches-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "temporal-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "temporal-coverage-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "text-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "thumbnail-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "thumbnail-url-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "time-required-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "transcript-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Transcript => {
								if r#transcript_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"transcript",
									));
								}
								r#transcript_property = Some({
									struct DeserializeWith(Vec<TranscriptProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "translation-of-work-property-schema",
									feature = "bib-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "translator-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "typical-age-range-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "upload-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::UploadDate => {
								if r#upload_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"uploadDate",
									));
								}
								r#upload_date_property = Some({
									struct DeserializeWith(Vec<UploadDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "usage-info-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "version-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "video-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "video-frame-size-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VideoFrameSize => {
								if r#video_frame_size_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"videoFrameSize",
									));
								}
								r#video_frame_size_property = Some({
									struct DeserializeWith(Vec<VideoFrameSizeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "video-quality-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::VideoQuality => {
								if r#video_quality_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"videoQuality",
									));
								}
								r#video_quality_property = Some({
									struct DeserializeWith(Vec<VideoQualityProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "width-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Width => {
								if r#width_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("width"));
								}
								r#width_property = Some({
									struct DeserializeWith(Vec<WidthProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "work-example-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "work-translation-property-schema",
									feature = "bib-schema-section"
								),
								doc
							))]
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
					Ok(VideoObjectSnapshot {
						#[cfg(any(
							any(
								feature = "about-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#about: r#about_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "abstract-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#abstract: r#abstract_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "access-mode-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#access_mode: r#access_mode_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "access-mode-sufficient-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#access_mode_sufficient: r#access_mode_sufficient_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "accessibility-api-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#accessibility_api: r#accessibility_api_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "accessibility-control-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#accessibility_control: r#accessibility_control_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "accessibility-feature-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#accessibility_feature: r#accessibility_feature_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "accessibility-hazard-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#accessibility_hazard: r#accessibility_hazard_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "accessibility-summary-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#accessibility_summary: r#accessibility_summary_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "accountable-person-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#accountable_person: r#accountable_person_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "acquire-license-page-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#acquire_license_page: r#acquire_license_page_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "actor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#actor: r#actor_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "actors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#actors: r#actors_property.unwrap_or_default(),
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
								feature = "alternative-headline-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#alternative_headline: r#alternative_headline_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "archived-at-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#archived_at: r#archived_at_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "assesses-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#assesses: r#assesses_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "associated-article-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#associated_article: r#associated_article_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "associated-media-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#associated_media: r#associated_media_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#audience: r#audience_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "audio-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#audio: r#audio_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "author-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#author: r#author_property.unwrap_or_default(),
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
								feature = "bitrate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#bitrate: r#bitrate_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "caption-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#caption: r#caption_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "character-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#character: r#character_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "citation-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#citation: r#citation_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "comment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#comment: r#comment_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "comment-count-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#comment_count: r#comment_count_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "conditions-of-access-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#conditions_of_access: r#conditions_of_access_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "content-location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#content_location: r#content_location_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "content-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#content_rating: r#content_rating_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "content-reference-time-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#content_reference_time: r#content_reference_time_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "content-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#content_size: r#content_size_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "content-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#content_url: r#content_url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "contributor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#contributor: r#contributor_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "copyright-holder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#copyright_holder: r#copyright_holder_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "copyright-notice-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#copyright_notice: r#copyright_notice_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "copyright-year-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#copyright_year: r#copyright_year_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "correction-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#correction: r#correction_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "country-of-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#country_of_origin: r#country_of_origin_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "creative-work-status-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#creative_work_status: r#creative_work_status_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "creator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#creator: r#creator_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "credit-text-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#credit_text: r#credit_text_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "date-created-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#date_created: r#date_created_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "date-modified-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#date_modified: r#date_modified_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "date-published-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#date_published: r#date_published_property.unwrap_or_default(),
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
								feature = "director-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#director: r#director_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "directors-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#directors: r#directors_property.unwrap_or_default(),
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
								feature = "discussion-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#discussion_url: r#discussion_url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#duration: r#duration_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "edit-eidr-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#edit_eidr: r#edit_eidr_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "editor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#editor: r#editor_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "educational-alignment-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#educational_alignment: r#educational_alignment_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "educational-level-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#educational_level: r#educational_level_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "educational-use-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#educational_use: r#educational_use_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "embed-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#embed_url: r#embed_url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "embedded-text-caption-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#embedded_text_caption: r#embedded_text_caption_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "encodes-creative-work-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#encodes_creative_work: r#encodes_creative_work_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "encoding-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#encoding: r#encoding_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "encoding-format-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#encoding_format: r#encoding_format_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "encodings-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#encodings: r#encodings_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "end-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#end_time: r#end_time_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "example-of-work-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#example_of_work: r#example_of_work_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "expires-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#expires: r#expires_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "file-format-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#file_format: r#file_format_property.unwrap_or_default(),
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
								feature = "genre-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#genre: r#genre_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-part-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#has_part: r#has_part_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "headline-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#headline: r#headline_property.unwrap_or_default(),
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
								feature = "in-language-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#in_language: r#in_language_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "ineligible-region-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#ineligible_region: r#ineligible_region_property.unwrap_or_default(),
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
								feature = "interactivity-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#interactivity_type: r#interactivity_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "interpreted-as-claim-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#interpreted_as_claim: r#interpreted_as_claim_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-accessible-for-free-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_accessible_for_free: r#is_accessible_for_free_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-based-on-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_based_on: r#is_based_on_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-based-on-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_based_on_url: r#is_based_on_url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-family-friendly-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_family_friendly: r#is_family_friendly_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-part-of-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_part_of: r#is_part_of_property.unwrap_or_default(),
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
								feature = "learning-resource-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#learning_resource_type: r#learning_resource_type_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "license-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#license: r#license_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "location-created-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#location_created: r#location_created_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "main-entity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#main_entity: r#main_entity_property.unwrap_or_default(),
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
								feature = "maintainer-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#maintainer: r#maintainer_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "material-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#material: r#material_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "material-extent-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#material_extent: r#material_extent_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "mentions-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#mentions: r#mentions_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "music-by-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#music_by: r#music_by_property.unwrap_or_default(),
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
								feature = "offers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#offers: r#offers_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "pattern-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#pattern: r#pattern_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "player-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#player_type: r#player_type_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "position-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#position: r#position_property.unwrap_or_default(),
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
								feature = "producer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#producer: r#producer_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "production-company-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#production_company: r#production_company_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#provider: r#provider_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "publication-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#publication: r#publication_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "publisher-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#publisher: r#publisher_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "publisher-imprint-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
						r#publisher_imprint: r#publisher_imprint_property.unwrap_or_default(),
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
								feature = "recorded-at-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#recorded_at: r#recorded_at_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "regions-allowed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#regions_allowed: r#regions_allowed_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "released-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#released_event: r#released_event_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "requires-subscription-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#requires_subscription: r#requires_subscription_property
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
								feature = "schema-version-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#schema_version: r#schema_version_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sd-date-published-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#sd_date_published: r#sd_date_published_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sd-license-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#sd_license: r#sd_license_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sd-publisher-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#sd_publisher: r#sd_publisher_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sha-256-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#sha_256: r#sha_256_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "size-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#size: r#size_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "source-organization-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#source_organization: r#source_organization_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "spatial-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#spatial: r#spatial_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "spatial-coverage-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#spatial_coverage: r#spatial_coverage_property.unwrap_or_default(),
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
								feature = "start-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#start_time: r#start_time_property.unwrap_or_default(),
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
								feature = "teaches-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#teaches: r#teaches_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "temporal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#temporal: r#temporal_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "temporal-coverage-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#temporal_coverage: r#temporal_coverage_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "text-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#text: r#text_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "thumbnail-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#thumbnail: r#thumbnail_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "thumbnail-url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#thumbnail_url: r#thumbnail_url_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "time-required-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#time_required: r#time_required_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "transcript-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#transcript: r#transcript_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "translation-of-work-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
						r#translation_of_work: r#translation_of_work_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "translator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#translator: r#translator_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "typical-age-range-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#typical_age_range: r#typical_age_range_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "upload-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#upload_date: r#upload_date_property.unwrap_or_default(),
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
								feature = "usage-info-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#usage_info: r#usage_info_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "version-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#version: r#version_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "video-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#video: r#video_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "video-frame-size-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#video_frame_size: r#video_frame_size_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "video-quality-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#video_quality: r#video_quality_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "width-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#width: r#width_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "work-example-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#work_example: r#work_example_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "work-translation-property-schema",
								feature = "bib-schema-section"
							),
							doc
						))]
						r#work_translation: r#work_translation_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(feature = "about-property-schema", feature = "general-schema-section"),
					doc
				))]
				"about",
				#[cfg(any(
					any(
						feature = "abstract-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"abstract",
				#[cfg(any(
					any(
						feature = "access-mode-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"accessMode",
				#[cfg(any(
					any(
						feature = "access-mode-sufficient-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"accessModeSufficient",
				#[cfg(any(
					any(
						feature = "accessibility-api-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"accessibilityAPI",
				#[cfg(any(
					any(
						feature = "accessibility-control-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"accessibilityControl",
				#[cfg(any(
					any(
						feature = "accessibility-feature-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"accessibilityFeature",
				#[cfg(any(
					any(
						feature = "accessibility-hazard-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"accessibilityHazard",
				#[cfg(any(
					any(
						feature = "accessibility-summary-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"accessibilitySummary",
				#[cfg(any(
					any(
						feature = "accountable-person-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"accountablePerson",
				#[cfg(any(
					any(
						feature = "acquire-license-page-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"acquireLicensePage",
				#[cfg(any(
					any(feature = "actor-property-schema", feature = "general-schema-section"),
					doc
				))]
				"actor",
				#[cfg(any(
					any(feature = "actors-property-schema", feature = "general-schema-section"),
					doc
				))]
				"actors",
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
					any(
						feature = "alternative-headline-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternativeHeadline",
				#[cfg(any(
					any(
						feature = "archived-at-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"archivedAt",
				#[cfg(any(
					any(
						feature = "assesses-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"assesses",
				#[cfg(any(
					any(
						feature = "associated-article-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"associatedArticle",
				#[cfg(any(
					any(
						feature = "associated-media-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"associatedMedia",
				#[cfg(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"audience",
				#[cfg(any(
					any(feature = "audio-property-schema", feature = "general-schema-section"),
					doc
				))]
				"audio",
				#[cfg(any(
					any(feature = "author-property-schema", feature = "general-schema-section"),
					doc
				))]
				"author",
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
						feature = "bitrate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"bitrate",
				#[cfg(any(
					any(
						feature = "caption-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"caption",
				#[cfg(any(
					any(
						feature = "character-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"character",
				#[cfg(any(
					any(
						feature = "citation-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"citation",
				#[cfg(any(
					any(
						feature = "comment-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"comment",
				#[cfg(any(
					any(
						feature = "comment-count-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"commentCount",
				#[cfg(any(
					any(
						feature = "conditions-of-access-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"conditionsOfAccess",
				#[cfg(any(
					any(
						feature = "content-location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contentLocation",
				#[cfg(any(
					any(
						feature = "content-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contentRating",
				#[cfg(any(
					any(
						feature = "content-reference-time-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"contentReferenceTime",
				#[cfg(any(
					any(
						feature = "content-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contentSize",
				#[cfg(any(
					any(
						feature = "content-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contentUrl",
				#[cfg(any(
					any(
						feature = "contributor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contributor",
				#[cfg(any(
					any(
						feature = "copyright-holder-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"copyrightHolder",
				#[cfg(any(
					any(
						feature = "copyright-notice-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"copyrightNotice",
				#[cfg(any(
					any(
						feature = "copyright-year-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"copyrightYear",
				#[cfg(any(
					any(
						feature = "correction-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"correction",
				#[cfg(any(
					any(
						feature = "country-of-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"countryOfOrigin",
				#[cfg(any(
					any(
						feature = "creative-work-status-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"creativeWorkStatus",
				#[cfg(any(
					any(
						feature = "creator-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"creator",
				#[cfg(any(
					any(
						feature = "credit-text-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"creditText",
				#[cfg(any(
					any(
						feature = "date-created-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"dateCreated",
				#[cfg(any(
					any(
						feature = "date-modified-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"dateModified",
				#[cfg(any(
					any(
						feature = "date-published-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"datePublished",
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
						feature = "director-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"director",
				#[cfg(any(
					any(
						feature = "directors-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"directors",
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
						feature = "discussion-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"discussionUrl",
				#[cfg(any(
					any(
						feature = "duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"duration",
				#[cfg(any(
					any(
						feature = "edit-eidr-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"editEIDR",
				#[cfg(any(
					any(feature = "editor-property-schema", feature = "general-schema-section"),
					doc
				))]
				"editor",
				#[cfg(any(
					any(
						feature = "educational-alignment-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"educationalAlignment",
				#[cfg(any(
					any(
						feature = "educational-level-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"educationalLevel",
				#[cfg(any(
					any(
						feature = "educational-use-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"educationalUse",
				#[cfg(any(
					any(
						feature = "embed-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"embedUrl",
				#[cfg(any(
					any(
						feature = "embedded-text-caption-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"embeddedTextCaption",
				#[cfg(any(
					any(
						feature = "encodes-creative-work-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"encodesCreativeWork",
				#[cfg(any(
					any(
						feature = "encoding-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"encoding",
				#[cfg(any(
					any(
						feature = "encoding-format-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"encodingFormat",
				#[cfg(any(
					any(
						feature = "encodings-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"encodings",
				#[cfg(any(
					any(
						feature = "end-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"endTime",
				#[cfg(any(
					any(
						feature = "example-of-work-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"exampleOfWork",
				#[cfg(any(
					any(
						feature = "expires-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"expires",
				#[cfg(any(
					any(
						feature = "file-format-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"fileFormat",
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
					any(feature = "genre-property-schema", feature = "general-schema-section"),
					doc
				))]
				"genre",
				#[cfg(any(
					any(
						feature = "has-part-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"hasPart",
				#[cfg(any(
					any(
						feature = "headline-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"headline",
				#[cfg(any(
					any(feature = "height-property-schema", feature = "general-schema-section"),
					doc
				))]
				"height",
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
						feature = "in-language-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"inLanguage",
				#[cfg(any(
					any(
						feature = "ineligible-region-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"ineligibleRegion",
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
						feature = "interactivity-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"interactivityType",
				#[cfg(any(
					any(
						feature = "interpreted-as-claim-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"interpretedAsClaim",
				#[cfg(any(
					any(
						feature = "is-accessible-for-free-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isAccessibleForFree",
				#[cfg(any(
					any(
						feature = "is-based-on-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isBasedOn",
				#[cfg(any(
					any(
						feature = "is-based-on-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isBasedOnUrl",
				#[cfg(any(
					any(
						feature = "is-family-friendly-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isFamilyFriendly",
				#[cfg(any(
					any(
						feature = "is-part-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isPartOf",
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
						feature = "learning-resource-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"learningResourceType",
				#[cfg(any(
					any(
						feature = "license-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"license",
				#[cfg(any(
					any(
						feature = "location-created-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"locationCreated",
				#[cfg(any(
					any(
						feature = "main-entity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mainEntity",
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
						feature = "maintainer-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"maintainer",
				#[cfg(any(
					any(
						feature = "material-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"material",
				#[cfg(any(
					any(
						feature = "material-extent-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"materialExtent",
				#[cfg(any(
					any(
						feature = "mentions-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mentions",
				#[cfg(any(
					any(
						feature = "music-by-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"musicBy",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				"offers",
				#[cfg(any(
					any(
						feature = "pattern-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"pattern",
				#[cfg(any(
					any(
						feature = "player-type-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"playerType",
				#[cfg(any(
					any(
						feature = "position-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"position",
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
						feature = "producer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"producer",
				#[cfg(any(
					any(
						feature = "production-company-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"productionCompany",
				#[cfg(any(
					any(
						feature = "provider-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"provider",
				#[cfg(any(
					any(
						feature = "publication-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"publication",
				#[cfg(any(
					any(
						feature = "publisher-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"publisher",
				#[cfg(any(
					any(
						feature = "publisher-imprint-property-schema",
						feature = "bib-schema-section"
					),
					doc
				))]
				"publisherImprint",
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
						feature = "recorded-at-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"recordedAt",
				#[cfg(any(
					any(
						feature = "regions-allowed-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"regionsAllowed",
				#[cfg(any(
					any(
						feature = "released-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"releasedEvent",
				#[cfg(any(
					any(
						feature = "requires-subscription-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"requiresSubscription",
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
					any(
						feature = "schema-version-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"schemaVersion",
				#[cfg(any(
					any(
						feature = "sd-date-published-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"sdDatePublished",
				#[cfg(any(
					any(
						feature = "sd-license-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"sdLicense",
				#[cfg(any(
					any(
						feature = "sd-publisher-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"sdPublisher",
				#[cfg(any(
					any(
						feature = "sha-256-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"sha256",
				#[cfg(any(
					any(feature = "size-property-schema", feature = "pending-schema-section"),
					doc
				))]
				"size",
				#[cfg(any(
					any(
						feature = "source-organization-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sourceOrganization",
				#[cfg(any(
					any(
						feature = "spatial-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"spatial",
				#[cfg(any(
					any(
						feature = "spatial-coverage-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"spatialCoverage",
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
						feature = "start-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"startTime",
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
						feature = "teaches-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"teaches",
				#[cfg(any(
					any(
						feature = "temporal-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"temporal",
				#[cfg(any(
					any(
						feature = "temporal-coverage-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"temporalCoverage",
				#[cfg(any(
					any(feature = "text-property-schema", feature = "general-schema-section"),
					doc
				))]
				"text",
				#[cfg(any(
					any(
						feature = "thumbnail-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"thumbnail",
				#[cfg(any(
					any(
						feature = "thumbnail-url-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"thumbnailUrl",
				#[cfg(any(
					any(
						feature = "time-required-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"timeRequired",
				#[cfg(any(
					any(
						feature = "transcript-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"transcript",
				#[cfg(any(
					any(
						feature = "translation-of-work-property-schema",
						feature = "bib-schema-section"
					),
					doc
				))]
				"translationOfWork",
				#[cfg(any(
					any(
						feature = "translator-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"translator",
				#[cfg(any(
					any(
						feature = "typical-age-range-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"typicalAgeRange",
				#[cfg(any(
					any(
						feature = "upload-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"uploadDate",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
				#[cfg(any(
					any(
						feature = "usage-info-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"usageInfo",
				#[cfg(any(
					any(
						feature = "version-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"version",
				#[cfg(any(
					any(feature = "video-property-schema", feature = "general-schema-section"),
					doc
				))]
				"video",
				#[cfg(any(
					any(
						feature = "video-frame-size-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"videoFrameSize",
				#[cfg(any(
					any(
						feature = "video-quality-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"videoQuality",
				#[cfg(any(
					any(feature = "width-property-schema", feature = "general-schema-section"),
					doc
				))]
				"width",
				#[cfg(any(
					any(
						feature = "work-example-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"workExample",
				#[cfg(any(
					any(
						feature = "work-translation-property-schema",
						feature = "bib-schema-section"
					),
					doc
				))]
				"workTranslation",
			];
			deserializer.deserialize_struct("VideoObjectSnapshot", FIELDS, ClassVisitor)
		}
	}
}
