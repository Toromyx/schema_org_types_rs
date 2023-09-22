use super::*;
/// A specific object or file containing a Legislation. Note that the same Legislation can be published in multiple files. For example, a digitally signed PDF, a plain PDF and an HTML version.
///
/// https://schema.org/LegislationObject
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LegislationObject {
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
        feature = "associated-article-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "associatedArticle"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#associated_article: Vec<AssociatedArticleProperty>,
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
        feature = "bitrate-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "bitrate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#bitrate: Vec<BitrateProperty>,
    #[cfg(any(
        feature = "character-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "character"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#character: Vec<CharacterProperty>,
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
        feature = "content-size-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contentSize"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#content_size: Vec<ContentSizeProperty>,
    #[cfg(any(
        feature = "content-url-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contentUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#content_url: Vec<ContentUrlProperty>,
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
        feature = "duration-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "duration"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#duration: Vec<DurationProperty>,
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
        feature = "embed-url-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "embedUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#embed_url: Vec<EmbedUrlProperty>,
    #[cfg(any(
        feature = "encodes-creative-work-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "encodesCreativeWork"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#encodes_creative_work: Vec<EncodesCreativeWorkProperty>,
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
        feature = "end-time-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "endTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#end_time: Vec<EndTimeProperty>,
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
        feature = "file-format-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "fileFormat"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#file_format: Vec<FileFormatProperty>,
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
    #[cfg(any(feature = "height-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "height"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#height: Vec<HeightProperty>,
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
        feature = "ineligible-region-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "ineligibleRegion"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#ineligible_region: Vec<IneligibleRegionProperty>,
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
        feature = "jurisdiction-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "jurisdiction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#jurisdiction: Vec<JurisdictionProperty>,
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
        feature = "legislation-applies-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationApplies"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_applies: Vec<LegislationAppliesProperty>,
    #[cfg(any(
        feature = "legislation-changes-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationChanges"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_changes: Vec<LegislationChangesProperty>,
    #[cfg(any(
        feature = "legislation-consolidates-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationConsolidates"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_consolidates: Vec<LegislationConsolidatesProperty>,
    #[cfg(any(
        feature = "legislation-date-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_date: Vec<LegislationDateProperty>,
    #[cfg(any(
        feature = "legislation-date-version-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationDateVersion"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_date_version: Vec<LegislationDateVersionProperty>,
    #[cfg(any(
        feature = "legislation-identifier-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationIdentifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_identifier: Vec<LegislationIdentifierProperty>,
    #[cfg(any(
        feature = "legislation-jurisdiction-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationJurisdiction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_jurisdiction: Vec<LegislationJurisdictionProperty>,
    #[cfg(any(
        feature = "legislation-legal-force-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationLegalForce"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_legal_force: Vec<LegislationLegalForceProperty>,
    #[cfg(any(
        feature = "legislation-legal-value-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationLegalValue"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_legal_value: Vec<LegislationLegalValueProperty>,
    #[cfg(any(
        feature = "legislation-passed-by-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationPassedBy"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_passed_by: Vec<LegislationPassedByProperty>,
    #[cfg(any(
        feature = "legislation-responsible-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationResponsible"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_responsible: Vec<LegislationResponsibleProperty>,
    #[cfg(any(
        feature = "legislation-transposes-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationTransposes"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_transposes: Vec<LegislationTransposesProperty>,
    #[cfg(any(
        feature = "legislation-type-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "legislationType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#legislation_type: Vec<LegislationTypeProperty>,
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
        feature = "mentions-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mentions"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#mentions: Vec<MentionsProperty>,
    #[cfg(any(feature = "name-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(feature = "offers-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "offers"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#offers: Vec<OffersProperty>,
    #[cfg(any(
        feature = "pattern-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "pattern"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#pattern: Vec<PatternProperty>,
    #[cfg(any(
        feature = "player-type-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "playerType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#player_type: Vec<PlayerTypeProperty>,
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
        feature = "producer-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "producer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#producer: Vec<ProducerProperty>,
    #[cfg(any(
        feature = "production-company-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "productionCompany"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#production_company: Vec<ProductionCompanyProperty>,
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
    #[cfg(any(
        feature = "recorded-at-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "recordedAt"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#recorded_at: Vec<RecordedAtProperty>,
    #[cfg(any(
        feature = "regions-allowed-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "regionsAllowed"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#regions_allowed: Vec<RegionsAllowedProperty>,
    #[cfg(any(
        feature = "released-event-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "releasedEvent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#released_event: Vec<ReleasedEventProperty>,
    #[cfg(any(
        feature = "requires-subscription-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "requiresSubscription"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#requires_subscription: Vec<RequiresSubscriptionProperty>,
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
    #[cfg(any(
        feature = "sha-256-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sha256"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sha_256: Vec<Sha256Property>,
    #[cfg(any(feature = "size-property-schema", feature = "pending-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "size"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#size: Vec<SizeProperty>,
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
        feature = "start-time-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "startTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#start_time: Vec<StartTimeProperty>,
    #[cfg(any(
        feature = "subject-of-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#subject_of: Vec<SubjectOfProperty>,
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
    #[cfg(any(
        feature = "upload-date-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "uploadDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#upload_date: Vec<UploadDateProperty>,
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
    #[cfg(any(feature = "width-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "width"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#width: Vec<WidthProperty>,
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