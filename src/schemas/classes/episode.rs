use super::*;
/// <https://schema.org/Episode>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Episode {
	pub r#actor: Vec<ActorProperty>,
	pub r#actors: Vec<ActorsProperty>,
	pub r#director: Vec<DirectorProperty>,
	pub r#directors: Vec<DirectorsProperty>,
	pub r#duration: Vec<DurationProperty>,
	pub r#episode_number: Vec<EpisodeNumberProperty>,
	pub r#music_by: Vec<MusicByProperty>,
	pub r#part_of_season: Vec<PartOfSeasonProperty>,
	pub r#part_of_series: Vec<PartOfSeriesProperty>,
	pub r#production_company: Vec<ProductionCompanyProperty>,
	pub r#trailer: Vec<TrailerProperty>,
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
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
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
	pub r#in_language: Vec<InLanguageProperty>,
	pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
	pub r#interactivity_type: Vec<InteractivityTypeProperty>,
	pub r#interpreted_as_claim: Vec<InterpretedAsClaimProperty>,
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	pub r#is_based_on: Vec<IsBasedOnProperty>,
	pub r#is_based_on_url: Vec<IsBasedOnUrlProperty>,
	pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
	pub r#is_part_of: Vec<IsPartOfProperty>,
	pub r#keywords: Vec<KeywordsProperty>,
	pub r#learning_resource_type: Vec<LearningResourceTypeProperty>,
	pub r#license: Vec<LicenseProperty>,
	pub r#location_created: Vec<LocationCreatedProperty>,
	pub r#main_entity: Vec<MainEntityProperty>,
	pub r#maintainer: Vec<MaintainerProperty>,
	pub r#material: Vec<MaterialProperty>,
	pub r#material_extent: Vec<MaterialExtentProperty>,
	pub r#mentions: Vec<MentionsProperty>,
	pub r#offers: Vec<OffersProperty>,
	pub r#pattern: Vec<PatternProperty>,
	pub r#position: Vec<PositionProperty>,
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
	pub r#schema_version: Vec<SchemaVersionProperty>,
	pub r#sd_date_published: Vec<SdDatePublishedProperty>,
	pub r#sd_license: Vec<SdLicenseProperty>,
	pub r#sd_publisher: Vec<SdPublisherProperty>,
	pub r#size: Vec<SizeProperty>,
	pub r#source_organization: Vec<SourceOrganizationProperty>,
	pub r#spatial: Vec<SpatialProperty>,
	pub r#spatial_coverage: Vec<SpatialCoverageProperty>,
	pub r#sponsor: Vec<SponsorProperty>,
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
	pub r#usage_info: Vec<UsageInfoProperty>,
	pub r#version: Vec<VersionProperty>,
	pub r#video: Vec<VideoProperty>,
	pub r#work_example: Vec<WorkExampleProperty>,
	pub r#work_translation: Vec<WorkTranslationProperty>,
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
pub trait EpisodeTrait {
	fn get_actor(&self) -> &[ActorProperty];
	fn take_actor(&mut self) -> Vec<ActorProperty>;
	fn get_actors(&self) -> &[ActorsProperty];
	fn take_actors(&mut self) -> Vec<ActorsProperty>;
	fn get_director(&self) -> &[DirectorProperty];
	fn take_director(&mut self) -> Vec<DirectorProperty>;
	fn get_directors(&self) -> &[DirectorsProperty];
	fn take_directors(&mut self) -> Vec<DirectorsProperty>;
	fn get_duration(&self) -> &[DurationProperty];
	fn take_duration(&mut self) -> Vec<DurationProperty>;
	fn get_episode_number(&self) -> &[EpisodeNumberProperty];
	fn take_episode_number(&mut self) -> Vec<EpisodeNumberProperty>;
	fn get_music_by(&self) -> &[MusicByProperty];
	fn take_music_by(&mut self) -> Vec<MusicByProperty>;
	fn get_part_of_season(&self) -> &[PartOfSeasonProperty];
	fn take_part_of_season(&mut self) -> Vec<PartOfSeasonProperty>;
	fn get_part_of_series(&self) -> &[PartOfSeriesProperty];
	fn take_part_of_series(&mut self) -> Vec<PartOfSeriesProperty>;
	fn get_production_company(&self) -> &[ProductionCompanyProperty];
	fn take_production_company(&mut self) -> Vec<ProductionCompanyProperty>;
	fn get_trailer(&self) -> &[TrailerProperty];
	fn take_trailer(&mut self) -> Vec<TrailerProperty>;
}
impl EpisodeTrait for Episode {
	fn get_actor(&self) -> &[ActorProperty] {
		self.r#actor.as_slice()
	}
	fn take_actor(&mut self) -> Vec<ActorProperty> {
		std::mem::take(&mut self.r#actor)
	}
	fn get_actors(&self) -> &[ActorsProperty] {
		self.r#actors.as_slice()
	}
	fn take_actors(&mut self) -> Vec<ActorsProperty> {
		std::mem::take(&mut self.r#actors)
	}
	fn get_director(&self) -> &[DirectorProperty] {
		self.r#director.as_slice()
	}
	fn take_director(&mut self) -> Vec<DirectorProperty> {
		std::mem::take(&mut self.r#director)
	}
	fn get_directors(&self) -> &[DirectorsProperty] {
		self.r#directors.as_slice()
	}
	fn take_directors(&mut self) -> Vec<DirectorsProperty> {
		std::mem::take(&mut self.r#directors)
	}
	fn get_duration(&self) -> &[DurationProperty] {
		self.r#duration.as_slice()
	}
	fn take_duration(&mut self) -> Vec<DurationProperty> {
		std::mem::take(&mut self.r#duration)
	}
	fn get_episode_number(&self) -> &[EpisodeNumberProperty] {
		self.r#episode_number.as_slice()
	}
	fn take_episode_number(&mut self) -> Vec<EpisodeNumberProperty> {
		std::mem::take(&mut self.r#episode_number)
	}
	fn get_music_by(&self) -> &[MusicByProperty] {
		self.r#music_by.as_slice()
	}
	fn take_music_by(&mut self) -> Vec<MusicByProperty> {
		std::mem::take(&mut self.r#music_by)
	}
	fn get_part_of_season(&self) -> &[PartOfSeasonProperty] {
		self.r#part_of_season.as_slice()
	}
	fn take_part_of_season(&mut self) -> Vec<PartOfSeasonProperty> {
		std::mem::take(&mut self.r#part_of_season)
	}
	fn get_part_of_series(&self) -> &[PartOfSeriesProperty] {
		self.r#part_of_series.as_slice()
	}
	fn take_part_of_series(&mut self) -> Vec<PartOfSeriesProperty> {
		std::mem::take(&mut self.r#part_of_series)
	}
	fn get_production_company(&self) -> &[ProductionCompanyProperty] {
		self.r#production_company.as_slice()
	}
	fn take_production_company(&mut self) -> Vec<ProductionCompanyProperty> {
		std::mem::take(&mut self.r#production_company)
	}
	fn get_trailer(&self) -> &[TrailerProperty] {
		self.r#trailer.as_slice()
	}
	fn take_trailer(&mut self) -> Vec<TrailerProperty> {
		std::mem::take(&mut self.r#trailer)
	}
}
impl CreativeWorkTrait for Episode {
	fn get_about(&self) -> &[AboutProperty] {
		self.r#about.as_slice()
	}
	fn take_about(&mut self) -> Vec<AboutProperty> {
		std::mem::take(&mut self.r#about)
	}
	fn get_abstract(&self) -> &[AbstractProperty] {
		self.r#abstract.as_slice()
	}
	fn take_abstract(&mut self) -> Vec<AbstractProperty> {
		std::mem::take(&mut self.r#abstract)
	}
	fn get_access_mode(&self) -> &[AccessModeProperty] {
		self.r#access_mode.as_slice()
	}
	fn take_access_mode(&mut self) -> Vec<AccessModeProperty> {
		std::mem::take(&mut self.r#access_mode)
	}
	fn get_access_mode_sufficient(&self) -> &[AccessModeSufficientProperty] {
		self.r#access_mode_sufficient.as_slice()
	}
	fn take_access_mode_sufficient(&mut self) -> Vec<AccessModeSufficientProperty> {
		std::mem::take(&mut self.r#access_mode_sufficient)
	}
	fn get_accessibility_api(&self) -> &[AccessibilityApiProperty] {
		self.r#accessibility_api.as_slice()
	}
	fn take_accessibility_api(&mut self) -> Vec<AccessibilityApiProperty> {
		std::mem::take(&mut self.r#accessibility_api)
	}
	fn get_accessibility_control(&self) -> &[AccessibilityControlProperty] {
		self.r#accessibility_control.as_slice()
	}
	fn take_accessibility_control(&mut self) -> Vec<AccessibilityControlProperty> {
		std::mem::take(&mut self.r#accessibility_control)
	}
	fn get_accessibility_feature(&self) -> &[AccessibilityFeatureProperty] {
		self.r#accessibility_feature.as_slice()
	}
	fn take_accessibility_feature(&mut self) -> Vec<AccessibilityFeatureProperty> {
		std::mem::take(&mut self.r#accessibility_feature)
	}
	fn get_accessibility_hazard(&self) -> &[AccessibilityHazardProperty] {
		self.r#accessibility_hazard.as_slice()
	}
	fn take_accessibility_hazard(&mut self) -> Vec<AccessibilityHazardProperty> {
		std::mem::take(&mut self.r#accessibility_hazard)
	}
	fn get_accessibility_summary(&self) -> &[AccessibilitySummaryProperty] {
		self.r#accessibility_summary.as_slice()
	}
	fn take_accessibility_summary(&mut self) -> Vec<AccessibilitySummaryProperty> {
		std::mem::take(&mut self.r#accessibility_summary)
	}
	fn get_accountable_person(&self) -> &[AccountablePersonProperty] {
		self.r#accountable_person.as_slice()
	}
	fn take_accountable_person(&mut self) -> Vec<AccountablePersonProperty> {
		std::mem::take(&mut self.r#accountable_person)
	}
	fn get_acquire_license_page(&self) -> &[AcquireLicensePageProperty] {
		self.r#acquire_license_page.as_slice()
	}
	fn take_acquire_license_page(&mut self) -> Vec<AcquireLicensePageProperty> {
		std::mem::take(&mut self.r#acquire_license_page)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_alternative_headline(&self) -> &[AlternativeHeadlineProperty] {
		self.r#alternative_headline.as_slice()
	}
	fn take_alternative_headline(&mut self) -> Vec<AlternativeHeadlineProperty> {
		std::mem::take(&mut self.r#alternative_headline)
	}
	fn get_archived_at(&self) -> &[ArchivedAtProperty] {
		self.r#archived_at.as_slice()
	}
	fn take_archived_at(&mut self) -> Vec<ArchivedAtProperty> {
		std::mem::take(&mut self.r#archived_at)
	}
	fn get_assesses(&self) -> &[AssessesProperty] {
		self.r#assesses.as_slice()
	}
	fn take_assesses(&mut self) -> Vec<AssessesProperty> {
		std::mem::take(&mut self.r#assesses)
	}
	fn get_associated_media(&self) -> &[AssociatedMediaProperty] {
		self.r#associated_media.as_slice()
	}
	fn take_associated_media(&mut self) -> Vec<AssociatedMediaProperty> {
		std::mem::take(&mut self.r#associated_media)
	}
	fn get_audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn take_audience(&mut self) -> Vec<AudienceProperty> {
		std::mem::take(&mut self.r#audience)
	}
	fn get_audio(&self) -> &[AudioProperty] {
		self.r#audio.as_slice()
	}
	fn take_audio(&mut self) -> Vec<AudioProperty> {
		std::mem::take(&mut self.r#audio)
	}
	fn get_author(&self) -> &[AuthorProperty] {
		self.r#author.as_slice()
	}
	fn take_author(&mut self) -> Vec<AuthorProperty> {
		std::mem::take(&mut self.r#author)
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
	fn get_character(&self) -> &[CharacterProperty] {
		self.r#character.as_slice()
	}
	fn take_character(&mut self) -> Vec<CharacterProperty> {
		std::mem::take(&mut self.r#character)
	}
	fn get_citation(&self) -> &[CitationProperty] {
		self.r#citation.as_slice()
	}
	fn take_citation(&mut self) -> Vec<CitationProperty> {
		std::mem::take(&mut self.r#citation)
	}
	fn get_comment(&self) -> &[CommentProperty] {
		self.r#comment.as_slice()
	}
	fn take_comment(&mut self) -> Vec<CommentProperty> {
		std::mem::take(&mut self.r#comment)
	}
	fn get_comment_count(&self) -> &[CommentCountProperty] {
		self.r#comment_count.as_slice()
	}
	fn take_comment_count(&mut self) -> Vec<CommentCountProperty> {
		std::mem::take(&mut self.r#comment_count)
	}
	fn get_conditions_of_access(&self) -> &[ConditionsOfAccessProperty] {
		self.r#conditions_of_access.as_slice()
	}
	fn take_conditions_of_access(&mut self) -> Vec<ConditionsOfAccessProperty> {
		std::mem::take(&mut self.r#conditions_of_access)
	}
	fn get_content_location(&self) -> &[ContentLocationProperty] {
		self.r#content_location.as_slice()
	}
	fn take_content_location(&mut self) -> Vec<ContentLocationProperty> {
		std::mem::take(&mut self.r#content_location)
	}
	fn get_content_rating(&self) -> &[ContentRatingProperty] {
		self.r#content_rating.as_slice()
	}
	fn take_content_rating(&mut self) -> Vec<ContentRatingProperty> {
		std::mem::take(&mut self.r#content_rating)
	}
	fn get_content_reference_time(&self) -> &[ContentReferenceTimeProperty] {
		self.r#content_reference_time.as_slice()
	}
	fn take_content_reference_time(&mut self) -> Vec<ContentReferenceTimeProperty> {
		std::mem::take(&mut self.r#content_reference_time)
	}
	fn get_contributor(&self) -> &[ContributorProperty] {
		self.r#contributor.as_slice()
	}
	fn take_contributor(&mut self) -> Vec<ContributorProperty> {
		std::mem::take(&mut self.r#contributor)
	}
	fn get_copyright_holder(&self) -> &[CopyrightHolderProperty] {
		self.r#copyright_holder.as_slice()
	}
	fn take_copyright_holder(&mut self) -> Vec<CopyrightHolderProperty> {
		std::mem::take(&mut self.r#copyright_holder)
	}
	fn get_copyright_notice(&self) -> &[CopyrightNoticeProperty] {
		self.r#copyright_notice.as_slice()
	}
	fn take_copyright_notice(&mut self) -> Vec<CopyrightNoticeProperty> {
		std::mem::take(&mut self.r#copyright_notice)
	}
	fn get_copyright_year(&self) -> &[CopyrightYearProperty] {
		self.r#copyright_year.as_slice()
	}
	fn take_copyright_year(&mut self) -> Vec<CopyrightYearProperty> {
		std::mem::take(&mut self.r#copyright_year)
	}
	fn get_correction(&self) -> &[CorrectionProperty] {
		self.r#correction.as_slice()
	}
	fn take_correction(&mut self) -> Vec<CorrectionProperty> {
		std::mem::take(&mut self.r#correction)
	}
	fn get_country_of_origin(&self) -> &[CountryOfOriginProperty] {
		self.r#country_of_origin.as_slice()
	}
	fn take_country_of_origin(&mut self) -> Vec<CountryOfOriginProperty> {
		std::mem::take(&mut self.r#country_of_origin)
	}
	fn get_creative_work_status(&self) -> &[CreativeWorkStatusProperty] {
		self.r#creative_work_status.as_slice()
	}
	fn take_creative_work_status(&mut self) -> Vec<CreativeWorkStatusProperty> {
		std::mem::take(&mut self.r#creative_work_status)
	}
	fn get_creator(&self) -> &[CreatorProperty] {
		self.r#creator.as_slice()
	}
	fn take_creator(&mut self) -> Vec<CreatorProperty> {
		std::mem::take(&mut self.r#creator)
	}
	fn get_credit_text(&self) -> &[CreditTextProperty] {
		self.r#credit_text.as_slice()
	}
	fn take_credit_text(&mut self) -> Vec<CreditTextProperty> {
		std::mem::take(&mut self.r#credit_text)
	}
	fn get_date_created(&self) -> &[DateCreatedProperty] {
		self.r#date_created.as_slice()
	}
	fn take_date_created(&mut self) -> Vec<DateCreatedProperty> {
		std::mem::take(&mut self.r#date_created)
	}
	fn get_date_modified(&self) -> &[DateModifiedProperty] {
		self.r#date_modified.as_slice()
	}
	fn take_date_modified(&mut self) -> Vec<DateModifiedProperty> {
		std::mem::take(&mut self.r#date_modified)
	}
	fn get_date_published(&self) -> &[DatePublishedProperty] {
		self.r#date_published.as_slice()
	}
	fn take_date_published(&mut self) -> Vec<DatePublishedProperty> {
		std::mem::take(&mut self.r#date_published)
	}
	fn get_discussion_url(&self) -> &[DiscussionUrlProperty] {
		self.r#discussion_url.as_slice()
	}
	fn take_discussion_url(&mut self) -> Vec<DiscussionUrlProperty> {
		std::mem::take(&mut self.r#discussion_url)
	}
	fn get_edit_eidr(&self) -> &[EditEidrProperty] {
		self.r#edit_eidr.as_slice()
	}
	fn take_edit_eidr(&mut self) -> Vec<EditEidrProperty> {
		std::mem::take(&mut self.r#edit_eidr)
	}
	fn get_editor(&self) -> &[EditorProperty] {
		self.r#editor.as_slice()
	}
	fn take_editor(&mut self) -> Vec<EditorProperty> {
		std::mem::take(&mut self.r#editor)
	}
	fn get_educational_alignment(&self) -> &[EducationalAlignmentProperty] {
		self.r#educational_alignment.as_slice()
	}
	fn take_educational_alignment(&mut self) -> Vec<EducationalAlignmentProperty> {
		std::mem::take(&mut self.r#educational_alignment)
	}
	fn get_educational_level(&self) -> &[EducationalLevelProperty] {
		self.r#educational_level.as_slice()
	}
	fn take_educational_level(&mut self) -> Vec<EducationalLevelProperty> {
		std::mem::take(&mut self.r#educational_level)
	}
	fn get_educational_use(&self) -> &[EducationalUseProperty] {
		self.r#educational_use.as_slice()
	}
	fn take_educational_use(&mut self) -> Vec<EducationalUseProperty> {
		std::mem::take(&mut self.r#educational_use)
	}
	fn get_encoding(&self) -> &[EncodingProperty] {
		self.r#encoding.as_slice()
	}
	fn take_encoding(&mut self) -> Vec<EncodingProperty> {
		std::mem::take(&mut self.r#encoding)
	}
	fn get_encoding_format(&self) -> &[EncodingFormatProperty] {
		self.r#encoding_format.as_slice()
	}
	fn take_encoding_format(&mut self) -> Vec<EncodingFormatProperty> {
		std::mem::take(&mut self.r#encoding_format)
	}
	fn get_encodings(&self) -> &[EncodingsProperty] {
		self.r#encodings.as_slice()
	}
	fn take_encodings(&mut self) -> Vec<EncodingsProperty> {
		std::mem::take(&mut self.r#encodings)
	}
	fn get_example_of_work(&self) -> &[ExampleOfWorkProperty] {
		self.r#example_of_work.as_slice()
	}
	fn take_example_of_work(&mut self) -> Vec<ExampleOfWorkProperty> {
		std::mem::take(&mut self.r#example_of_work)
	}
	fn get_expires(&self) -> &[ExpiresProperty] {
		self.r#expires.as_slice()
	}
	fn take_expires(&mut self) -> Vec<ExpiresProperty> {
		std::mem::take(&mut self.r#expires)
	}
	fn get_file_format(&self) -> &[FileFormatProperty] {
		self.r#file_format.as_slice()
	}
	fn take_file_format(&mut self) -> Vec<FileFormatProperty> {
		std::mem::take(&mut self.r#file_format)
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
	fn get_genre(&self) -> &[GenreProperty] {
		self.r#genre.as_slice()
	}
	fn take_genre(&mut self) -> Vec<GenreProperty> {
		std::mem::take(&mut self.r#genre)
	}
	fn get_has_part(&self) -> &[HasPartProperty] {
		self.r#has_part.as_slice()
	}
	fn take_has_part(&mut self) -> Vec<HasPartProperty> {
		std::mem::take(&mut self.r#has_part)
	}
	fn get_headline(&self) -> &[HeadlineProperty] {
		self.r#headline.as_slice()
	}
	fn take_headline(&mut self) -> Vec<HeadlineProperty> {
		std::mem::take(&mut self.r#headline)
	}
	fn get_in_language(&self) -> &[InLanguageProperty] {
		self.r#in_language.as_slice()
	}
	fn take_in_language(&mut self) -> Vec<InLanguageProperty> {
		std::mem::take(&mut self.r#in_language)
	}
	fn get_interaction_statistic(&self) -> &[InteractionStatisticProperty] {
		self.r#interaction_statistic.as_slice()
	}
	fn take_interaction_statistic(&mut self) -> Vec<InteractionStatisticProperty> {
		std::mem::take(&mut self.r#interaction_statistic)
	}
	fn get_interactivity_type(&self) -> &[InteractivityTypeProperty] {
		self.r#interactivity_type.as_slice()
	}
	fn take_interactivity_type(&mut self) -> Vec<InteractivityTypeProperty> {
		std::mem::take(&mut self.r#interactivity_type)
	}
	fn get_interpreted_as_claim(&self) -> &[InterpretedAsClaimProperty] {
		self.r#interpreted_as_claim.as_slice()
	}
	fn take_interpreted_as_claim(&mut self) -> Vec<InterpretedAsClaimProperty> {
		std::mem::take(&mut self.r#interpreted_as_claim)
	}
	fn get_is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty] {
		self.r#is_accessible_for_free.as_slice()
	}
	fn take_is_accessible_for_free(&mut self) -> Vec<IsAccessibleForFreeProperty> {
		std::mem::take(&mut self.r#is_accessible_for_free)
	}
	fn get_is_based_on(&self) -> &[IsBasedOnProperty] {
		self.r#is_based_on.as_slice()
	}
	fn take_is_based_on(&mut self) -> Vec<IsBasedOnProperty> {
		std::mem::take(&mut self.r#is_based_on)
	}
	fn get_is_based_on_url(&self) -> &[IsBasedOnUrlProperty] {
		self.r#is_based_on_url.as_slice()
	}
	fn take_is_based_on_url(&mut self) -> Vec<IsBasedOnUrlProperty> {
		std::mem::take(&mut self.r#is_based_on_url)
	}
	fn get_is_family_friendly(&self) -> &[IsFamilyFriendlyProperty] {
		self.r#is_family_friendly.as_slice()
	}
	fn take_is_family_friendly(&mut self) -> Vec<IsFamilyFriendlyProperty> {
		std::mem::take(&mut self.r#is_family_friendly)
	}
	fn get_is_part_of(&self) -> &[IsPartOfProperty] {
		self.r#is_part_of.as_slice()
	}
	fn take_is_part_of(&mut self) -> Vec<IsPartOfProperty> {
		std::mem::take(&mut self.r#is_part_of)
	}
	fn get_keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn take_keywords(&mut self) -> Vec<KeywordsProperty> {
		std::mem::take(&mut self.r#keywords)
	}
	fn get_learning_resource_type(&self) -> &[LearningResourceTypeProperty] {
		self.r#learning_resource_type.as_slice()
	}
	fn take_learning_resource_type(&mut self) -> Vec<LearningResourceTypeProperty> {
		std::mem::take(&mut self.r#learning_resource_type)
	}
	fn get_license(&self) -> &[LicenseProperty] {
		self.r#license.as_slice()
	}
	fn take_license(&mut self) -> Vec<LicenseProperty> {
		std::mem::take(&mut self.r#license)
	}
	fn get_location_created(&self) -> &[LocationCreatedProperty] {
		self.r#location_created.as_slice()
	}
	fn take_location_created(&mut self) -> Vec<LocationCreatedProperty> {
		std::mem::take(&mut self.r#location_created)
	}
	fn get_main_entity(&self) -> &[MainEntityProperty] {
		self.r#main_entity.as_slice()
	}
	fn take_main_entity(&mut self) -> Vec<MainEntityProperty> {
		std::mem::take(&mut self.r#main_entity)
	}
	fn get_maintainer(&self) -> &[MaintainerProperty] {
		self.r#maintainer.as_slice()
	}
	fn take_maintainer(&mut self) -> Vec<MaintainerProperty> {
		std::mem::take(&mut self.r#maintainer)
	}
	fn get_material(&self) -> &[MaterialProperty] {
		self.r#material.as_slice()
	}
	fn take_material(&mut self) -> Vec<MaterialProperty> {
		std::mem::take(&mut self.r#material)
	}
	fn get_material_extent(&self) -> &[MaterialExtentProperty] {
		self.r#material_extent.as_slice()
	}
	fn take_material_extent(&mut self) -> Vec<MaterialExtentProperty> {
		std::mem::take(&mut self.r#material_extent)
	}
	fn get_mentions(&self) -> &[MentionsProperty] {
		self.r#mentions.as_slice()
	}
	fn take_mentions(&mut self) -> Vec<MentionsProperty> {
		std::mem::take(&mut self.r#mentions)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_pattern(&self) -> &[PatternProperty] {
		self.r#pattern.as_slice()
	}
	fn take_pattern(&mut self) -> Vec<PatternProperty> {
		std::mem::take(&mut self.r#pattern)
	}
	fn get_position(&self) -> &[PositionProperty] {
		self.r#position.as_slice()
	}
	fn take_position(&mut self) -> Vec<PositionProperty> {
		std::mem::take(&mut self.r#position)
	}
	fn get_producer(&self) -> &[ProducerProperty] {
		self.r#producer.as_slice()
	}
	fn take_producer(&mut self) -> Vec<ProducerProperty> {
		std::mem::take(&mut self.r#producer)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_publication(&self) -> &[PublicationProperty] {
		self.r#publication.as_slice()
	}
	fn take_publication(&mut self) -> Vec<PublicationProperty> {
		std::mem::take(&mut self.r#publication)
	}
	fn get_publisher(&self) -> &[PublisherProperty] {
		self.r#publisher.as_slice()
	}
	fn take_publisher(&mut self) -> Vec<PublisherProperty> {
		std::mem::take(&mut self.r#publisher)
	}
	fn get_publisher_imprint(&self) -> &[PublisherImprintProperty] {
		self.r#publisher_imprint.as_slice()
	}
	fn take_publisher_imprint(&mut self) -> Vec<PublisherImprintProperty> {
		std::mem::take(&mut self.r#publisher_imprint)
	}
	fn get_publishing_principles(&self) -> &[PublishingPrinciplesProperty] {
		self.r#publishing_principles.as_slice()
	}
	fn take_publishing_principles(&mut self) -> Vec<PublishingPrinciplesProperty> {
		std::mem::take(&mut self.r#publishing_principles)
	}
	fn get_recorded_at(&self) -> &[RecordedAtProperty] {
		self.r#recorded_at.as_slice()
	}
	fn take_recorded_at(&mut self) -> Vec<RecordedAtProperty> {
		std::mem::take(&mut self.r#recorded_at)
	}
	fn get_released_event(&self) -> &[ReleasedEventProperty] {
		self.r#released_event.as_slice()
	}
	fn take_released_event(&mut self) -> Vec<ReleasedEventProperty> {
		std::mem::take(&mut self.r#released_event)
	}
	fn get_review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn take_review(&mut self) -> Vec<ReviewProperty> {
		std::mem::take(&mut self.r#review)
	}
	fn get_reviews(&self) -> &[ReviewsProperty] {
		self.r#reviews.as_slice()
	}
	fn take_reviews(&mut self) -> Vec<ReviewsProperty> {
		std::mem::take(&mut self.r#reviews)
	}
	fn get_schema_version(&self) -> &[SchemaVersionProperty] {
		self.r#schema_version.as_slice()
	}
	fn take_schema_version(&mut self) -> Vec<SchemaVersionProperty> {
		std::mem::take(&mut self.r#schema_version)
	}
	fn get_sd_date_published(&self) -> &[SdDatePublishedProperty] {
		self.r#sd_date_published.as_slice()
	}
	fn take_sd_date_published(&mut self) -> Vec<SdDatePublishedProperty> {
		std::mem::take(&mut self.r#sd_date_published)
	}
	fn get_sd_license(&self) -> &[SdLicenseProperty] {
		self.r#sd_license.as_slice()
	}
	fn take_sd_license(&mut self) -> Vec<SdLicenseProperty> {
		std::mem::take(&mut self.r#sd_license)
	}
	fn get_sd_publisher(&self) -> &[SdPublisherProperty] {
		self.r#sd_publisher.as_slice()
	}
	fn take_sd_publisher(&mut self) -> Vec<SdPublisherProperty> {
		std::mem::take(&mut self.r#sd_publisher)
	}
	fn get_size(&self) -> &[SizeProperty] {
		self.r#size.as_slice()
	}
	fn take_size(&mut self) -> Vec<SizeProperty> {
		std::mem::take(&mut self.r#size)
	}
	fn get_source_organization(&self) -> &[SourceOrganizationProperty] {
		self.r#source_organization.as_slice()
	}
	fn take_source_organization(&mut self) -> Vec<SourceOrganizationProperty> {
		std::mem::take(&mut self.r#source_organization)
	}
	fn get_spatial(&self) -> &[SpatialProperty] {
		self.r#spatial.as_slice()
	}
	fn take_spatial(&mut self) -> Vec<SpatialProperty> {
		std::mem::take(&mut self.r#spatial)
	}
	fn get_spatial_coverage(&self) -> &[SpatialCoverageProperty] {
		self.r#spatial_coverage.as_slice()
	}
	fn take_spatial_coverage(&mut self) -> Vec<SpatialCoverageProperty> {
		std::mem::take(&mut self.r#spatial_coverage)
	}
	fn get_sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn take_sponsor(&mut self) -> Vec<SponsorProperty> {
		std::mem::take(&mut self.r#sponsor)
	}
	fn get_teaches(&self) -> &[TeachesProperty] {
		self.r#teaches.as_slice()
	}
	fn take_teaches(&mut self) -> Vec<TeachesProperty> {
		std::mem::take(&mut self.r#teaches)
	}
	fn get_temporal(&self) -> &[TemporalProperty] {
		self.r#temporal.as_slice()
	}
	fn take_temporal(&mut self) -> Vec<TemporalProperty> {
		std::mem::take(&mut self.r#temporal)
	}
	fn get_temporal_coverage(&self) -> &[TemporalCoverageProperty] {
		self.r#temporal_coverage.as_slice()
	}
	fn take_temporal_coverage(&mut self) -> Vec<TemporalCoverageProperty> {
		std::mem::take(&mut self.r#temporal_coverage)
	}
	fn get_text(&self) -> &[TextProperty] {
		self.r#text.as_slice()
	}
	fn take_text(&mut self) -> Vec<TextProperty> {
		std::mem::take(&mut self.r#text)
	}
	fn get_thumbnail(&self) -> &[ThumbnailProperty] {
		self.r#thumbnail.as_slice()
	}
	fn take_thumbnail(&mut self) -> Vec<ThumbnailProperty> {
		std::mem::take(&mut self.r#thumbnail)
	}
	fn get_thumbnail_url(&self) -> &[ThumbnailUrlProperty] {
		self.r#thumbnail_url.as_slice()
	}
	fn take_thumbnail_url(&mut self) -> Vec<ThumbnailUrlProperty> {
		std::mem::take(&mut self.r#thumbnail_url)
	}
	fn get_time_required(&self) -> &[TimeRequiredProperty] {
		self.r#time_required.as_slice()
	}
	fn take_time_required(&mut self) -> Vec<TimeRequiredProperty> {
		std::mem::take(&mut self.r#time_required)
	}
	fn get_translation_of_work(&self) -> &[TranslationOfWorkProperty] {
		self.r#translation_of_work.as_slice()
	}
	fn take_translation_of_work(&mut self) -> Vec<TranslationOfWorkProperty> {
		std::mem::take(&mut self.r#translation_of_work)
	}
	fn get_translator(&self) -> &[TranslatorProperty] {
		self.r#translator.as_slice()
	}
	fn take_translator(&mut self) -> Vec<TranslatorProperty> {
		std::mem::take(&mut self.r#translator)
	}
	fn get_typical_age_range(&self) -> &[TypicalAgeRangeProperty] {
		self.r#typical_age_range.as_slice()
	}
	fn take_typical_age_range(&mut self) -> Vec<TypicalAgeRangeProperty> {
		std::mem::take(&mut self.r#typical_age_range)
	}
	fn get_usage_info(&self) -> &[UsageInfoProperty] {
		self.r#usage_info.as_slice()
	}
	fn take_usage_info(&mut self) -> Vec<UsageInfoProperty> {
		std::mem::take(&mut self.r#usage_info)
	}
	fn get_version(&self) -> &[VersionProperty] {
		self.r#version.as_slice()
	}
	fn take_version(&mut self) -> Vec<VersionProperty> {
		std::mem::take(&mut self.r#version)
	}
	fn get_video(&self) -> &[VideoProperty] {
		self.r#video.as_slice()
	}
	fn take_video(&mut self) -> Vec<VideoProperty> {
		std::mem::take(&mut self.r#video)
	}
	fn get_work_example(&self) -> &[WorkExampleProperty] {
		self.r#work_example.as_slice()
	}
	fn take_work_example(&mut self) -> Vec<WorkExampleProperty> {
		std::mem::take(&mut self.r#work_example)
	}
	fn get_work_translation(&self) -> &[WorkTranslationProperty] {
		self.r#work_translation.as_slice()
	}
	fn take_work_translation(&mut self) -> Vec<WorkTranslationProperty> {
		std::mem::take(&mut self.r#work_translation)
	}
}
impl ThingTrait for Episode {
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
	impl Serialize for Episode {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#actor) as usize,
				!Vec::is_empty(&self.r#actors) as usize,
				!Vec::is_empty(&self.r#director) as usize,
				!Vec::is_empty(&self.r#directors) as usize,
				!Vec::is_empty(&self.r#duration) as usize,
				!Vec::is_empty(&self.r#episode_number) as usize,
				!Vec::is_empty(&self.r#music_by) as usize,
				!Vec::is_empty(&self.r#part_of_season) as usize,
				!Vec::is_empty(&self.r#part_of_series) as usize,
				!Vec::is_empty(&self.r#production_company) as usize,
				!Vec::is_empty(&self.r#trailer) as usize,
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
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
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
				!Vec::is_empty(&self.r#in_language) as usize,
				!Vec::is_empty(&self.r#interaction_statistic) as usize,
				!Vec::is_empty(&self.r#interactivity_type) as usize,
				!Vec::is_empty(&self.r#interpreted_as_claim) as usize,
				!Vec::is_empty(&self.r#is_accessible_for_free) as usize,
				!Vec::is_empty(&self.r#is_based_on) as usize,
				!Vec::is_empty(&self.r#is_based_on_url) as usize,
				!Vec::is_empty(&self.r#is_family_friendly) as usize,
				!Vec::is_empty(&self.r#is_part_of) as usize,
				!Vec::is_empty(&self.r#keywords) as usize,
				!Vec::is_empty(&self.r#learning_resource_type) as usize,
				!Vec::is_empty(&self.r#license) as usize,
				!Vec::is_empty(&self.r#location_created) as usize,
				!Vec::is_empty(&self.r#main_entity) as usize,
				!Vec::is_empty(&self.r#maintainer) as usize,
				!Vec::is_empty(&self.r#material) as usize,
				!Vec::is_empty(&self.r#material_extent) as usize,
				!Vec::is_empty(&self.r#mentions) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#pattern) as usize,
				!Vec::is_empty(&self.r#position) as usize,
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
				!Vec::is_empty(&self.r#schema_version) as usize,
				!Vec::is_empty(&self.r#sd_date_published) as usize,
				!Vec::is_empty(&self.r#sd_license) as usize,
				!Vec::is_empty(&self.r#sd_publisher) as usize,
				!Vec::is_empty(&self.r#size) as usize,
				!Vec::is_empty(&self.r#source_organization) as usize,
				!Vec::is_empty(&self.r#spatial) as usize,
				!Vec::is_empty(&self.r#spatial_coverage) as usize,
				!Vec::is_empty(&self.r#sponsor) as usize,
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
				!Vec::is_empty(&self.r#usage_info) as usize,
				!Vec::is_empty(&self.r#version) as usize,
				!Vec::is_empty(&self.r#video) as usize,
				!Vec::is_empty(&self.r#work_example) as usize,
				!Vec::is_empty(&self.r#work_translation) as usize,
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
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Episode", len)?;
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
			if !Vec::is_empty(&self.r#episode_number) {
				serialize_struct.serialize_field("episodeNumber", {
					struct SerializeWith<'a>(&'a Vec<EpisodeNumberProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#episode_number)
				})?;
			} else {
				serialize_struct.skip_field("episodeNumber")?;
			}
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
			if !Vec::is_empty(&self.r#part_of_season) {
				serialize_struct.serialize_field("partOfSeason", {
					struct SerializeWith<'a>(&'a Vec<PartOfSeasonProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#part_of_season)
				})?;
			} else {
				serialize_struct.skip_field("partOfSeason")?;
			}
			if !Vec::is_empty(&self.r#part_of_series) {
				serialize_struct.serialize_field("partOfSeries", {
					struct SerializeWith<'a>(&'a Vec<PartOfSeriesProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#part_of_series)
				})?;
			} else {
				serialize_struct.skip_field("partOfSeries")?;
			}
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
			if !Vec::is_empty(&self.r#trailer) {
				serialize_struct.serialize_field("trailer", {
					struct SerializeWith<'a>(&'a Vec<TrailerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#trailer)
				})?;
			} else {
				serialize_struct.skip_field("trailer")?;
			}
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
	impl<'de> Deserialize<'de> for Episode {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Actor,
				Actors,
				Director,
				Directors,
				Duration,
				EpisodeNumber,
				MusicBy,
				PartOfSeason,
				PartOfSeries,
				ProductionCompany,
				Trailer,
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
				AggregateRating,
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
				InLanguage,
				InteractionStatistic,
				InteractivityType,
				InterpretedAsClaim,
				IsAccessibleForFree,
				IsBasedOn,
				IsBasedOnUrl,
				IsFamilyFriendly,
				IsPartOf,
				Keywords,
				LearningResourceType,
				License,
				LocationCreated,
				MainEntity,
				Maintainer,
				Material,
				MaterialExtent,
				Mentions,
				Offers,
				Pattern,
				Position,
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
				SchemaVersion,
				SdDatePublished,
				SdLicense,
				SdPublisher,
				Size,
				SourceOrganization,
				Spatial,
				SpatialCoverage,
				Sponsor,
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
				UsageInfo,
				Version,
				Video,
				WorkExample,
				WorkTranslation,
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
						"actor" => Ok(Field::Actor),
						"actors" => Ok(Field::Actors),
						"director" => Ok(Field::Director),
						"directors" => Ok(Field::Directors),
						"duration" => Ok(Field::Duration),
						"episodeNumber" => Ok(Field::EpisodeNumber),
						"musicBy" => Ok(Field::MusicBy),
						"partOfSeason" => Ok(Field::PartOfSeason),
						"partOfSeries" => Ok(Field::PartOfSeries),
						"productionCompany" => Ok(Field::ProductionCompany),
						"trailer" => Ok(Field::Trailer),
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
						"aggregateRating" => Ok(Field::AggregateRating),
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
						"inLanguage" => Ok(Field::InLanguage),
						"interactionStatistic" => Ok(Field::InteractionStatistic),
						"interactivityType" => Ok(Field::InteractivityType),
						"interpretedAsClaim" => Ok(Field::InterpretedAsClaim),
						"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						"isBasedOn" => Ok(Field::IsBasedOn),
						"isBasedOnUrl" => Ok(Field::IsBasedOnUrl),
						"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						"isPartOf" => Ok(Field::IsPartOf),
						"keywords" => Ok(Field::Keywords),
						"learningResourceType" => Ok(Field::LearningResourceType),
						"license" => Ok(Field::License),
						"locationCreated" => Ok(Field::LocationCreated),
						"mainEntity" => Ok(Field::MainEntity),
						"maintainer" => Ok(Field::Maintainer),
						"material" => Ok(Field::Material),
						"materialExtent" => Ok(Field::MaterialExtent),
						"mentions" => Ok(Field::Mentions),
						"offers" => Ok(Field::Offers),
						"pattern" => Ok(Field::Pattern),
						"position" => Ok(Field::Position),
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
						"schemaVersion" => Ok(Field::SchemaVersion),
						"sdDatePublished" => Ok(Field::SdDatePublished),
						"sdLicense" => Ok(Field::SdLicense),
						"sdPublisher" => Ok(Field::SdPublisher),
						"size" => Ok(Field::Size),
						"sourceOrganization" => Ok(Field::SourceOrganization),
						"spatial" => Ok(Field::Spatial),
						"spatialCoverage" => Ok(Field::SpatialCoverage),
						"sponsor" => Ok(Field::Sponsor),
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
						"usageInfo" => Ok(Field::UsageInfo),
						"version" => Ok(Field::Version),
						"video" => Ok(Field::Video),
						"workExample" => Ok(Field::WorkExample),
						"workTranslation" => Ok(Field::WorkTranslation),
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
						b"actor" => Ok(Field::Actor),
						b"actors" => Ok(Field::Actors),
						b"director" => Ok(Field::Director),
						b"directors" => Ok(Field::Directors),
						b"duration" => Ok(Field::Duration),
						b"episodeNumber" => Ok(Field::EpisodeNumber),
						b"musicBy" => Ok(Field::MusicBy),
						b"partOfSeason" => Ok(Field::PartOfSeason),
						b"partOfSeries" => Ok(Field::PartOfSeries),
						b"productionCompany" => Ok(Field::ProductionCompany),
						b"trailer" => Ok(Field::Trailer),
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
						b"aggregateRating" => Ok(Field::AggregateRating),
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
						b"inLanguage" => Ok(Field::InLanguage),
						b"interactionStatistic" => Ok(Field::InteractionStatistic),
						b"interactivityType" => Ok(Field::InteractivityType),
						b"interpretedAsClaim" => Ok(Field::InterpretedAsClaim),
						b"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						b"isBasedOn" => Ok(Field::IsBasedOn),
						b"isBasedOnUrl" => Ok(Field::IsBasedOnUrl),
						b"isFamilyFriendly" => Ok(Field::IsFamilyFriendly),
						b"isPartOf" => Ok(Field::IsPartOf),
						b"keywords" => Ok(Field::Keywords),
						b"learningResourceType" => Ok(Field::LearningResourceType),
						b"license" => Ok(Field::License),
						b"locationCreated" => Ok(Field::LocationCreated),
						b"mainEntity" => Ok(Field::MainEntity),
						b"maintainer" => Ok(Field::Maintainer),
						b"material" => Ok(Field::Material),
						b"materialExtent" => Ok(Field::MaterialExtent),
						b"mentions" => Ok(Field::Mentions),
						b"offers" => Ok(Field::Offers),
						b"pattern" => Ok(Field::Pattern),
						b"position" => Ok(Field::Position),
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
						b"schemaVersion" => Ok(Field::SchemaVersion),
						b"sdDatePublished" => Ok(Field::SdDatePublished),
						b"sdLicense" => Ok(Field::SdLicense),
						b"sdPublisher" => Ok(Field::SdPublisher),
						b"size" => Ok(Field::Size),
						b"sourceOrganization" => Ok(Field::SourceOrganization),
						b"spatial" => Ok(Field::Spatial),
						b"spatialCoverage" => Ok(Field::SpatialCoverage),
						b"sponsor" => Ok(Field::Sponsor),
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
						b"usageInfo" => Ok(Field::UsageInfo),
						b"version" => Ok(Field::Version),
						b"video" => Ok(Field::Video),
						b"workExample" => Ok(Field::WorkExample),
						b"workTranslation" => Ok(Field::WorkTranslation),
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
				type Value = Episode;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Episode")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#actor_property = None;
					let mut r#actors_property = None;
					let mut r#director_property = None;
					let mut r#directors_property = None;
					let mut r#duration_property = None;
					let mut r#episode_number_property = None;
					let mut r#music_by_property = None;
					let mut r#part_of_season_property = None;
					let mut r#part_of_series_property = None;
					let mut r#production_company_property = None;
					let mut r#trailer_property = None;
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
					let mut r#aggregate_rating_property = None;
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
					let mut r#in_language_property = None;
					let mut r#interaction_statistic_property = None;
					let mut r#interactivity_type_property = None;
					let mut r#interpreted_as_claim_property = None;
					let mut r#is_accessible_for_free_property = None;
					let mut r#is_based_on_property = None;
					let mut r#is_based_on_url_property = None;
					let mut r#is_family_friendly_property = None;
					let mut r#is_part_of_property = None;
					let mut r#keywords_property = None;
					let mut r#learning_resource_type_property = None;
					let mut r#license_property = None;
					let mut r#location_created_property = None;
					let mut r#main_entity_property = None;
					let mut r#maintainer_property = None;
					let mut r#material_property = None;
					let mut r#material_extent_property = None;
					let mut r#mentions_property = None;
					let mut r#offers_property = None;
					let mut r#pattern_property = None;
					let mut r#position_property = None;
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
					let mut r#schema_version_property = None;
					let mut r#sd_date_published_property = None;
					let mut r#sd_license_property = None;
					let mut r#sd_publisher_property = None;
					let mut r#size_property = None;
					let mut r#source_organization_property = None;
					let mut r#spatial_property = None;
					let mut r#spatial_coverage_property = None;
					let mut r#sponsor_property = None;
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
					let mut r#usage_info_property = None;
					let mut r#version_property = None;
					let mut r#video_property = None;
					let mut r#work_example_property = None;
					let mut r#work_translation_property = None;
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
							Field::EpisodeNumber => {
								if r#episode_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"episodeNumber",
									));
								}
								r#episode_number_property = Some({
									struct DeserializeWith(Vec<EpisodeNumberProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::PartOfSeason => {
								if r#part_of_season_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"partOfSeason",
									));
								}
								r#part_of_season_property = Some({
									struct DeserializeWith(Vec<PartOfSeasonProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::PartOfSeries => {
								if r#part_of_series_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"partOfSeries",
									));
								}
								r#part_of_series_property = Some({
									struct DeserializeWith(Vec<PartOfSeriesProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
							Field::Trailer => {
								if r#trailer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"trailer",
									));
								}
								r#trailer_property = Some({
									struct DeserializeWith(Vec<TrailerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
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
					Ok(Episode {
						r#actor: r#actor_property.unwrap_or_default(),
						r#actors: r#actors_property.unwrap_or_default(),
						r#director: r#director_property.unwrap_or_default(),
						r#directors: r#directors_property.unwrap_or_default(),
						r#duration: r#duration_property.unwrap_or_default(),
						r#episode_number: r#episode_number_property.unwrap_or_default(),
						r#music_by: r#music_by_property.unwrap_or_default(),
						r#part_of_season: r#part_of_season_property.unwrap_or_default(),
						r#part_of_series: r#part_of_series_property.unwrap_or_default(),
						r#production_company: r#production_company_property.unwrap_or_default(),
						r#trailer: r#trailer_property.unwrap_or_default(),
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
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
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
						r#keywords: r#keywords_property.unwrap_or_default(),
						r#learning_resource_type: r#learning_resource_type_property
							.unwrap_or_default(),
						r#license: r#license_property.unwrap_or_default(),
						r#location_created: r#location_created_property.unwrap_or_default(),
						r#main_entity: r#main_entity_property.unwrap_or_default(),
						r#maintainer: r#maintainer_property.unwrap_or_default(),
						r#material: r#material_property.unwrap_or_default(),
						r#material_extent: r#material_extent_property.unwrap_or_default(),
						r#mentions: r#mentions_property.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#pattern: r#pattern_property.unwrap_or_default(),
						r#position: r#position_property.unwrap_or_default(),
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
						r#schema_version: r#schema_version_property.unwrap_or_default(),
						r#sd_date_published: r#sd_date_published_property.unwrap_or_default(),
						r#sd_license: r#sd_license_property.unwrap_or_default(),
						r#sd_publisher: r#sd_publisher_property.unwrap_or_default(),
						r#size: r#size_property.unwrap_or_default(),
						r#source_organization: r#source_organization_property.unwrap_or_default(),
						r#spatial: r#spatial_property.unwrap_or_default(),
						r#spatial_coverage: r#spatial_coverage_property.unwrap_or_default(),
						r#sponsor: r#sponsor_property.unwrap_or_default(),
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
						r#usage_info: r#usage_info_property.unwrap_or_default(),
						r#version: r#version_property.unwrap_or_default(),
						r#video: r#video_property.unwrap_or_default(),
						r#work_example: r#work_example_property.unwrap_or_default(),
						r#work_translation: r#work_translation_property.unwrap_or_default(),
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
				"actor",
				"actors",
				"director",
				"directors",
				"duration",
				"episodeNumber",
				"musicBy",
				"partOfSeason",
				"partOfSeries",
				"productionCompany",
				"trailer",
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
				"aggregateRating",
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
				"inLanguage",
				"interactionStatistic",
				"interactivityType",
				"interpretedAsClaim",
				"isAccessibleForFree",
				"isBasedOn",
				"isBasedOnUrl",
				"isFamilyFriendly",
				"isPartOf",
				"keywords",
				"learningResourceType",
				"license",
				"locationCreated",
				"mainEntity",
				"maintainer",
				"material",
				"materialExtent",
				"mentions",
				"offers",
				"pattern",
				"position",
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
				"schemaVersion",
				"sdDatePublished",
				"sdLicense",
				"sdPublisher",
				"size",
				"sourceOrganization",
				"spatial",
				"spatialCoverage",
				"sponsor",
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
				"usageInfo",
				"version",
				"video",
				"workExample",
				"workTranslation",
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
			deserializer.deserialize_struct("Episode", FIELDS, ClassVisitor)
		}
	}
}
