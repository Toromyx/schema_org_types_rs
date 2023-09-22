use super::*;
/// A video game is an electronic game that involves human interaction with a user interface to generate visual feedback on a video device.
///
/// https://schema.org/VideoGame
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VideoGame {
    #[cfg(any(feature = "about-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "about"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#about: Vec<AboutProperty>,
    #[cfg(any(
        feature = "abstract-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "abstract"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#abstract: Vec<AbstractProperty>,
    #[cfg(any(
        feature = "access-mode-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessMode"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#access_mode: Vec<AccessModeProperty>,
    #[cfg(any(
        feature = "access-mode-sufficient-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessModeSufficient"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#access_mode_sufficient: Vec<AccessModeSufficientProperty>,
    #[cfg(any(
        feature = "accessibility-api-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilityAPI"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#accessibility_api: Vec<AccessibilityApiProperty>,
    #[cfg(any(
        feature = "accessibility-control-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilityControl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#accessibility_control: Vec<AccessibilityControlProperty>,
    #[cfg(any(
        feature = "accessibility-feature-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilityFeature"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#accessibility_feature: Vec<AccessibilityFeatureProperty>,
    #[cfg(any(
        feature = "accessibility-hazard-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilityHazard"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#accessibility_hazard: Vec<AccessibilityHazardProperty>,
    #[cfg(any(
        feature = "accessibility-summary-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilitySummary"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#accessibility_summary: Vec<AccessibilitySummaryProperty>,
    #[cfg(any(
        feature = "accountable-person-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accountablePerson"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#accountable_person: Vec<AccountablePersonProperty>,
    #[cfg(any(
        feature = "acquire-license-page-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "acquireLicensePage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#acquire_license_page: Vec<AcquireLicensePageProperty>,
    #[cfg(any(feature = "actor-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "actor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#actor: Vec<ActorProperty>,
    #[cfg(any(feature = "actors-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "actors"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#actors: Vec<ActorsProperty>,
    #[cfg(any(
        feature = "additional-type-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        feature = "aggregate-rating-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "aggregateRating"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#aggregate_rating: Vec<AggregateRatingProperty>,
    #[cfg(any(
        feature = "alternate-name-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        feature = "alternative-headline-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternativeHeadline"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#alternative_headline: Vec<AlternativeHeadlineProperty>,
    #[cfg(any(
        feature = "application-category-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "applicationCategory"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#application_category: Vec<ApplicationCategoryProperty>,
    #[cfg(any(
        feature = "application-sub-category-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "applicationSubCategory"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#application_sub_category: Vec<ApplicationSubCategoryProperty>,
    #[cfg(any(
        feature = "application-suite-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "applicationSuite"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#application_suite: Vec<ApplicationSuiteProperty>,
    #[cfg(any(
        feature = "archived-at-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "archivedAt"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#archived_at: Vec<ArchivedAtProperty>,
    #[cfg(any(
        feature = "assesses-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "assesses"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#assesses: Vec<AssessesProperty>,
    #[cfg(any(
        feature = "associated-media-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "associatedMedia"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#associated_media: Vec<AssociatedMediaProperty>,
    #[cfg(any(
        feature = "audience-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "audience"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#audience: Vec<AudienceProperty>,
    #[cfg(any(feature = "audio-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "audio"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#audio: Vec<AudioProperty>,
    #[cfg(any(feature = "author-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "author"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#author: Vec<AuthorProperty>,
    #[cfg(any(
        feature = "available-on-device-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "availableOnDevice"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#available_on_device: Vec<AvailableOnDeviceProperty>,
    #[cfg(any(feature = "award-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "award"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#award: Vec<AwardProperty>,
    #[cfg(any(feature = "awards-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "awards"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#awards: Vec<AwardsProperty>,
    #[cfg(any(
        feature = "character-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "character"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#character: Vec<CharacterProperty>,
    #[cfg(any(
        feature = "character-attribute-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "characterAttribute"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#character_attribute: Vec<CharacterAttributeProperty>,
    #[cfg(any(
        feature = "cheat-code-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cheatCode"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cheat_code: Vec<CheatCodeProperty>,
    #[cfg(any(
        feature = "citation-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "citation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#citation: Vec<CitationProperty>,
    #[cfg(any(
        feature = "comment-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "comment"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#comment: Vec<CommentProperty>,
    #[cfg(any(
        feature = "comment-count-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "commentCount"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#comment_count: Vec<CommentCountProperty>,
    #[cfg(any(
        feature = "conditions-of-access-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "conditionsOfAccess"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#conditions_of_access: Vec<ConditionsOfAccessProperty>,
    #[cfg(any(
        feature = "content-location-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contentLocation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#content_location: Vec<ContentLocationProperty>,
    #[cfg(any(
        feature = "content-rating-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contentRating"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#content_rating: Vec<ContentRatingProperty>,
    #[cfg(any(
        feature = "content-reference-time-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contentReferenceTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#content_reference_time: Vec<ContentReferenceTimeProperty>,
    #[cfg(any(
        feature = "contributor-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contributor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#contributor: Vec<ContributorProperty>,
    #[cfg(any(
        feature = "copyright-holder-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "copyrightHolder"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#copyright_holder: Vec<CopyrightHolderProperty>,
    #[cfg(any(
        feature = "copyright-notice-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "copyrightNotice"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#copyright_notice: Vec<CopyrightNoticeProperty>,
    #[cfg(any(
        feature = "copyright-year-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "copyrightYear"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#copyright_year: Vec<CopyrightYearProperty>,
    #[cfg(any(
        feature = "correction-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "correction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#correction: Vec<CorrectionProperty>,
    #[cfg(any(
        feature = "countries-not-supported-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countriesNotSupported"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#countries_not_supported: Vec<CountriesNotSupportedProperty>,
    #[cfg(any(
        feature = "countries-supported-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countriesSupported"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#countries_supported: Vec<CountriesSupportedProperty>,
    #[cfg(any(
        feature = "country-of-origin-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countryOfOrigin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#country_of_origin: Vec<CountryOfOriginProperty>,
    #[cfg(any(
        feature = "creative-work-status-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "creativeWorkStatus"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#creative_work_status: Vec<CreativeWorkStatusProperty>,
    #[cfg(any(
        feature = "creator-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "creator"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#creator: Vec<CreatorProperty>,
    #[cfg(any(
        feature = "credit-text-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "creditText"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#credit_text: Vec<CreditTextProperty>,
    #[cfg(any(
        feature = "date-created-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "dateCreated"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#date_created: Vec<DateCreatedProperty>,
    #[cfg(any(
        feature = "date-modified-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "dateModified"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#date_modified: Vec<DateModifiedProperty>,
    #[cfg(any(
        feature = "date-published-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "datePublished"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#date_published: Vec<DatePublishedProperty>,
    #[cfg(any(
        feature = "description-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(feature = "device-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "device"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#device: Vec<DeviceProperty>,
    #[cfg(any(
        feature = "director-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "director"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#director: Vec<DirectorProperty>,
    #[cfg(any(
        feature = "directors-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "directors"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#directors: Vec<DirectorsProperty>,
    #[cfg(any(
        feature = "disambiguating-description-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        feature = "discussion-url-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "discussionUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#discussion_url: Vec<DiscussionUrlProperty>,
    #[cfg(any(
        feature = "download-url-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "downloadUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#download_url: Vec<DownloadUrlProperty>,
    #[cfg(any(
        feature = "edit-eidr-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "editEIDR"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#edit_eidr: Vec<EditEidrProperty>,
    #[cfg(any(feature = "editor-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "editor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#editor: Vec<EditorProperty>,
    #[cfg(any(
        feature = "educational-alignment-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "educationalAlignment"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#educational_alignment: Vec<EducationalAlignmentProperty>,
    #[cfg(any(
        feature = "educational-level-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "educationalLevel"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#educational_level: Vec<EducationalLevelProperty>,
    #[cfg(any(
        feature = "educational-use-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "educationalUse"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#educational_use: Vec<EducationalUseProperty>,
    #[cfg(any(
        feature = "encoding-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "encoding"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#encoding: Vec<EncodingProperty>,
    #[cfg(any(
        feature = "encoding-format-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "encodingFormat"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#encoding_format: Vec<EncodingFormatProperty>,
    #[cfg(any(
        feature = "encodings-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "encodings"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#encodings: Vec<EncodingsProperty>,
    #[cfg(any(
        feature = "example-of-work-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "exampleOfWork"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#example_of_work: Vec<ExampleOfWorkProperty>,
    #[cfg(any(
        feature = "expires-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "expires"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#expires: Vec<ExpiresProperty>,
    #[cfg(any(
        feature = "feature-list-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "featureList"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#feature_list: Vec<FeatureListProperty>,
    #[cfg(any(
        feature = "file-format-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "fileFormat"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#file_format: Vec<FileFormatProperty>,
    #[cfg(any(
        feature = "file-size-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "fileSize"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#file_size: Vec<FileSizeProperty>,
    #[cfg(any(feature = "funder-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "funder"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#funder: Vec<FunderProperty>,
    #[cfg(any(
        feature = "funding-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "funding"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#funding: Vec<FundingProperty>,
    #[cfg(any(
        feature = "game-edition-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gameEdition"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#game_edition: Vec<GameEditionProperty>,
    #[cfg(any(
        feature = "game-item-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gameItem"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#game_item: Vec<GameItemProperty>,
    #[cfg(any(
        feature = "game-location-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gameLocation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#game_location: Vec<GameLocationProperty>,
    #[cfg(any(
        feature = "game-platform-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gamePlatform"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#game_platform: Vec<GamePlatformProperty>,
    #[cfg(any(
        feature = "game-server-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gameServer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#game_server: Vec<GameServerProperty>,
    #[cfg(any(
        feature = "game-tip-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "gameTip"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#game_tip: Vec<GameTipProperty>,
    #[cfg(any(feature = "genre-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "genre"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#genre: Vec<GenreProperty>,
    #[cfg(any(
        feature = "has-part-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasPart"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#has_part: Vec<HasPartProperty>,
    #[cfg(any(
        feature = "headline-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "headline"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#headline: Vec<HeadlineProperty>,
    #[cfg(any(
        feature = "identifier-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(feature = "image-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        feature = "in-language-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "inLanguage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#in_language: Vec<InLanguageProperty>,
    #[cfg(any(
        feature = "install-url-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "installUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#install_url: Vec<InstallUrlProperty>,
    #[cfg(any(
        feature = "interaction-statistic-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "interactionStatistic"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
    #[cfg(any(
        feature = "interactivity-type-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "interactivityType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#interactivity_type: Vec<InteractivityTypeProperty>,
    #[cfg(any(
        feature = "interpreted-as-claim-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "interpretedAsClaim"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#interpreted_as_claim: Vec<InterpretedAsClaimProperty>,
    #[cfg(any(
        feature = "is-accessible-for-free-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isAccessibleForFree"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
    #[cfg(any(
        feature = "is-based-on-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isBasedOn"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_based_on: Vec<IsBasedOnProperty>,
    #[cfg(any(
        feature = "is-based-on-url-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isBasedOnUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_based_on_url: Vec<IsBasedOnUrlProperty>,
    #[cfg(any(
        feature = "is-family-friendly-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isFamilyFriendly"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
    #[cfg(any(
        feature = "is-part-of-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isPartOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#is_part_of: Vec<IsPartOfProperty>,
    #[cfg(any(
        feature = "keywords-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "keywords"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#keywords: Vec<KeywordsProperty>,
    #[cfg(any(
        feature = "learning-resource-type-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "learningResourceType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#learning_resource_type: Vec<LearningResourceTypeProperty>,
    #[cfg(any(
        feature = "license-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "license"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#license: Vec<LicenseProperty>,
    #[cfg(any(
        feature = "location-created-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "locationCreated"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#location_created: Vec<LocationCreatedProperty>,
    #[cfg(any(
        feature = "main-entity-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntity"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#main_entity: Vec<MainEntityProperty>,
    #[cfg(any(
        feature = "main-entity-of-page-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        feature = "maintainer-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "maintainer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#maintainer: Vec<MaintainerProperty>,
    #[cfg(any(
        feature = "material-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#material: Vec<MaterialProperty>,
    #[cfg(any(
        feature = "material-extent-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "materialExtent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#material_extent: Vec<MaterialExtentProperty>,
    #[cfg(any(
        feature = "memory-requirements-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "memoryRequirements"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#memory_requirements: Vec<MemoryRequirementsProperty>,
    #[cfg(any(
        feature = "mentions-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mentions"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#mentions: Vec<MentionsProperty>,
    #[cfg(any(
        feature = "music-by-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "musicBy"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#music_by: Vec<MusicByProperty>,
    #[cfg(any(feature = "name-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        feature = "number-of-players-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "numberOfPlayers"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#number_of_players: Vec<NumberOfPlayersProperty>,
    #[cfg(any(feature = "offers-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "offers"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#offers: Vec<OffersProperty>,
    #[cfg(any(
        feature = "operating-system-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "operatingSystem"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#operating_system: Vec<OperatingSystemProperty>,
    #[cfg(any(
        feature = "pattern-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "pattern"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#pattern: Vec<PatternProperty>,
    #[cfg(any(
        feature = "permissions-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "permissions"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#permissions: Vec<PermissionsProperty>,
    #[cfg(any(
        feature = "play-mode-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "playMode"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#play_mode: Vec<PlayModeProperty>,
    #[cfg(any(
        feature = "position-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "position"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#position: Vec<PositionProperty>,
    #[cfg(any(
        feature = "potential-action-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        feature = "processor-requirements-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "processorRequirements"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#processor_requirements: Vec<ProcessorRequirementsProperty>,
    #[cfg(any(
        feature = "producer-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "producer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#producer: Vec<ProducerProperty>,
    #[cfg(any(
        feature = "provider-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "provider"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#provider: Vec<ProviderProperty>,
    #[cfg(any(
        feature = "publication-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publication"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#publication: Vec<PublicationProperty>,
    #[cfg(any(
        feature = "publisher-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publisher"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#publisher: Vec<PublisherProperty>,
    #[cfg(any(
        feature = "publisher-imprint-property-schema",
        feature = "bib-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publisherImprint"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#publisher_imprint: Vec<PublisherImprintProperty>,
    #[cfg(any(
        feature = "publishing-principles-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publishingPrinciples"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
    #[cfg(any(feature = "quest-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "quest"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#quest: Vec<QuestProperty>,
    #[cfg(any(
        feature = "recorded-at-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "recordedAt"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#recorded_at: Vec<RecordedAtProperty>,
    #[cfg(any(
        feature = "release-notes-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "releaseNotes"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#release_notes: Vec<ReleaseNotesProperty>,
    #[cfg(any(
        feature = "released-event-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "releasedEvent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#released_event: Vec<ReleasedEventProperty>,
    #[cfg(any(
        feature = "requirements-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "requirements"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#requirements: Vec<RequirementsProperty>,
    #[cfg(any(feature = "review-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "review"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#review: Vec<ReviewProperty>,
    #[cfg(any(
        feature = "reviews-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "reviews"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#reviews: Vec<ReviewsProperty>,
    #[cfg(any(
        feature = "same-as-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        feature = "schema-version-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "schemaVersion"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#schema_version: Vec<SchemaVersionProperty>,
    #[cfg(any(
        feature = "screenshot-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "screenshot"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#screenshot: Vec<ScreenshotProperty>,
    #[cfg(any(
        feature = "sd-date-published-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sdDatePublished"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sd_date_published: Vec<SdDatePublishedProperty>,
    #[cfg(any(
        feature = "sd-license-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sdLicense"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sd_license: Vec<SdLicenseProperty>,
    #[cfg(any(
        feature = "sd-publisher-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sdPublisher"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sd_publisher: Vec<SdPublisherProperty>,
    #[cfg(any(feature = "size-property-schema", feature = "pending-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "size"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#size: Vec<SizeProperty>,
    #[cfg(any(
        feature = "software-add-on-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "softwareAddOn"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#software_add_on: Vec<SoftwareAddOnProperty>,
    #[cfg(any(
        feature = "software-help-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "softwareHelp"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#software_help: Vec<SoftwareHelpProperty>,
    #[cfg(any(
        feature = "software-requirements-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "softwareRequirements"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#software_requirements: Vec<SoftwareRequirementsProperty>,
    #[cfg(any(
        feature = "software-version-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "softwareVersion"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#software_version: Vec<SoftwareVersionProperty>,
    #[cfg(any(
        feature = "source-organization-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sourceOrganization"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#source_organization: Vec<SourceOrganizationProperty>,
    #[cfg(any(
        feature = "spatial-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "spatial"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#spatial: Vec<SpatialProperty>,
    #[cfg(any(
        feature = "spatial-coverage-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "spatialCoverage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#spatial_coverage: Vec<SpatialCoverageProperty>,
    #[cfg(any(
        feature = "sponsor-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sponsor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sponsor: Vec<SponsorProperty>,
    #[cfg(any(
        feature = "storage-requirements-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "storageRequirements"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#storage_requirements: Vec<StorageRequirementsProperty>,
    #[cfg(any(
        feature = "subject-of-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        feature = "supporting-data-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "supportingData"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#supporting_data: Vec<SupportingDataProperty>,
    #[cfg(any(
        feature = "teaches-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "teaches"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#teaches: Vec<TeachesProperty>,
    #[cfg(any(
        feature = "temporal-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "temporal"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#temporal: Vec<TemporalProperty>,
    #[cfg(any(
        feature = "temporal-coverage-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "temporalCoverage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#temporal_coverage: Vec<TemporalCoverageProperty>,
    #[cfg(any(feature = "text-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "text"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#text: Vec<TextProperty>,
    #[cfg(any(
        feature = "thumbnail-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "thumbnail"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#thumbnail: Vec<ThumbnailProperty>,
    #[cfg(any(
        feature = "thumbnail-url-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "thumbnailUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#thumbnail_url: Vec<ThumbnailUrlProperty>,
    #[cfg(any(
        feature = "time-required-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "timeRequired"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#time_required: Vec<TimeRequiredProperty>,
    #[cfg(any(
        feature = "trailer-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "trailer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#trailer: Vec<TrailerProperty>,
    #[cfg(any(
        feature = "translation-of-work-property-schema",
        feature = "bib-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "translationOfWork"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#translation_of_work: Vec<TranslationOfWorkProperty>,
    #[cfg(any(
        feature = "translator-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "translator"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#translator: Vec<TranslatorProperty>,
    #[cfg(any(
        feature = "typical-age-range-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "typicalAgeRange"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#typical_age_range: Vec<TypicalAgeRangeProperty>,
    #[cfg(any(feature = "url-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        feature = "usage-info-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "usageInfo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#usage_info: Vec<UsageInfoProperty>,
    #[cfg(any(
        feature = "version-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "version"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#version: Vec<VersionProperty>,
    #[cfg(any(feature = "video-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "video"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#video: Vec<VideoProperty>,
    #[cfg(any(
        feature = "work-example-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "workExample"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#work_example: Vec<WorkExampleProperty>,
    #[cfg(any(
        feature = "work-translation-property-schema",
        feature = "bib-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "workTranslation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#work_translation: Vec<WorkTranslationProperty>,
}