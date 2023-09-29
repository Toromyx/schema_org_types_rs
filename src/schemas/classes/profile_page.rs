use super::*;
/// <https://schema.org/ProfilePage>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProfilePage {
    #[cfg(any(
        any(feature = "about-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "about"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/about"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/about"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#about: Vec<AboutProperty>,
    #[cfg(any(
        any(
            feature = "abstract-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "abstract"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/abstract"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/abstract"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#abstract: Vec<AbstractProperty>,
    #[cfg(any(
        any(
            feature = "access-mode-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessMode"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/accessMode"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/accessMode"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#access_mode: Vec<AccessModeProperty>,
    #[cfg(any(
        any(
            feature = "access-mode-sufficient-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessModeSufficient"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/accessModeSufficient")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/accessModeSufficient")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#access_mode_sufficient: Vec<AccessModeSufficientProperty>,
    #[cfg(any(
        any(
            feature = "accessibility-api-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilityAPI"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/accessibilityAPI")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/accessibilityAPI"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#accessibility_api: Vec<AccessibilityApiProperty>,
    #[cfg(any(
        any(
            feature = "accessibility-control-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilityControl"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/accessibilityControl")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/accessibilityControl")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#accessibility_control: Vec<AccessibilityControlProperty>,
    #[cfg(any(
        any(
            feature = "accessibility-feature-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilityFeature"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/accessibilityFeature")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/accessibilityFeature")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#accessibility_feature: Vec<AccessibilityFeatureProperty>,
    #[cfg(any(
        any(
            feature = "accessibility-hazard-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilityHazard"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/accessibilityHazard")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/accessibilityHazard")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#accessibility_hazard: Vec<AccessibilityHazardProperty>,
    #[cfg(any(
        any(
            feature = "accessibility-summary-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accessibilitySummary"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/accessibilitySummary")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/accessibilitySummary")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#accessibility_summary: Vec<AccessibilitySummaryProperty>,
    #[cfg(any(
        any(
            feature = "accountable-person-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "accountablePerson"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/accountablePerson")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/accountablePerson")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#accountable_person: Vec<AccountablePersonProperty>,
    #[cfg(any(
        any(
            feature = "acquire-license-page-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "acquireLicensePage"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/acquireLicensePage")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/acquireLicensePage")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#acquire_license_page: Vec<AcquireLicensePageProperty>,
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/additionalType"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "aggregate-rating-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "aggregateRating"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/aggregateRating"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/aggregateRating"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#aggregate_rating: Vec<AggregateRatingProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/alternateName"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "alternative-headline-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternativeHeadline"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/alternativeHeadline")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/alternativeHeadline")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#alternative_headline: Vec<AlternativeHeadlineProperty>,
    #[cfg(any(
        any(
            feature = "archived-at-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "archivedAt"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/archivedAt"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/archivedAt"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#archived_at: Vec<ArchivedAtProperty>,
    #[cfg(any(
        any(
            feature = "assesses-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "assesses"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/assesses"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/assesses"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#assesses: Vec<AssessesProperty>,
    #[cfg(any(
        any(
            feature = "associated-media-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "associatedMedia"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/associatedMedia"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/associatedMedia"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#associated_media: Vec<AssociatedMediaProperty>,
    #[cfg(any(
        any(
            feature = "audience-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "audience"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/audience"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/audience"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#audience: Vec<AudienceProperty>,
    #[cfg(any(
        any(feature = "audio-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "audio"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/audio"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/audio"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#audio: Vec<AudioProperty>,
    #[cfg(any(
        any(feature = "author-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "author"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/author"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/author"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#author: Vec<AuthorProperty>,
    #[cfg(any(
        any(feature = "award-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "award"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/award"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/award"))]
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
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/awards"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/awards"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#awards: Vec<AwardsProperty>,
    #[cfg(any(
        any(
            feature = "breadcrumb-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "breadcrumb"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/breadcrumb"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/breadcrumb"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#breadcrumb: Vec<BreadcrumbProperty>,
    #[cfg(any(
        any(
            feature = "character-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "character"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/character"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/character"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#character: Vec<CharacterProperty>,
    #[cfg(any(
        any(
            feature = "citation-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "citation"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/citation"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/citation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#citation: Vec<CitationProperty>,
    #[cfg(any(
        any(
            feature = "comment-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "comment"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/comment"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/comment"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#comment: Vec<CommentProperty>,
    #[cfg(any(
        any(
            feature = "comment-count-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "commentCount"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/commentCount"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/commentCount"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#comment_count: Vec<CommentCountProperty>,
    #[cfg(any(
        any(
            feature = "conditions-of-access-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "conditionsOfAccess"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/conditionsOfAccess")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/conditionsOfAccess")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#conditions_of_access: Vec<ConditionsOfAccessProperty>,
    #[cfg(any(
        any(
            feature = "content-location-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contentLocation"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/contentLocation"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/contentLocation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#content_location: Vec<ContentLocationProperty>,
    #[cfg(any(
        any(
            feature = "content-rating-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contentRating"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/contentRating"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/contentRating"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#content_rating: Vec<ContentRatingProperty>,
    #[cfg(any(
        any(
            feature = "content-reference-time-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contentReferenceTime"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/contentReferenceTime")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/contentReferenceTime")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#content_reference_time: Vec<ContentReferenceTimeProperty>,
    #[cfg(any(
        any(
            feature = "contributor-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "contributor"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/contributor"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/contributor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#contributor: Vec<ContributorProperty>,
    #[cfg(any(
        any(
            feature = "copyright-holder-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "copyrightHolder"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/copyrightHolder"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/copyrightHolder"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#copyright_holder: Vec<CopyrightHolderProperty>,
    #[cfg(any(
        any(
            feature = "copyright-notice-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "copyrightNotice"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/copyrightNotice"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/copyrightNotice"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#copyright_notice: Vec<CopyrightNoticeProperty>,
    #[cfg(any(
        any(
            feature = "copyright-year-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "copyrightYear"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/copyrightYear"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/copyrightYear"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#copyright_year: Vec<CopyrightYearProperty>,
    #[cfg(any(
        any(
            feature = "correction-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "correction"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/correction"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/correction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#correction: Vec<CorrectionProperty>,
    #[cfg(any(
        any(
            feature = "country-of-origin-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "countryOfOrigin"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/countryOfOrigin"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/countryOfOrigin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#country_of_origin: Vec<CountryOfOriginProperty>,
    #[cfg(any(
        any(
            feature = "creative-work-status-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "creativeWorkStatus"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/creativeWorkStatus")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/creativeWorkStatus")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#creative_work_status: Vec<CreativeWorkStatusProperty>,
    #[cfg(any(
        any(
            feature = "creator-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "creator"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/creator"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/creator"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#creator: Vec<CreatorProperty>,
    #[cfg(any(
        any(
            feature = "credit-text-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "creditText"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/creditText"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/creditText"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#credit_text: Vec<CreditTextProperty>,
    #[cfg(any(
        any(
            feature = "date-created-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "dateCreated"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/dateCreated"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/dateCreated"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#date_created: Vec<DateCreatedProperty>,
    #[cfg(any(
        any(
            feature = "date-modified-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "dateModified"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/dateModified"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/dateModified"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#date_modified: Vec<DateModifiedProperty>,
    #[cfg(any(
        any(
            feature = "date-published-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "datePublished"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/datePublished"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/datePublished"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#date_published: Vec<DatePublishedProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/description"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/disambiguatingDescription")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/disambiguatingDescription")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "discussion-url-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "discussionUrl"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/discussionUrl"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/discussionUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#discussion_url: Vec<DiscussionUrlProperty>,
    #[cfg(any(
        any(
            feature = "edit-eidr-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "editEIDR"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/editEIDR"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/editEIDR"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#edit_eidr: Vec<EditEidrProperty>,
    #[cfg(any(
        any(feature = "editor-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "editor"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/editor"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/editor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#editor: Vec<EditorProperty>,
    #[cfg(any(
        any(
            feature = "educational-alignment-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "educationalAlignment"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/educationalAlignment")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/educationalAlignment")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#educational_alignment: Vec<EducationalAlignmentProperty>,
    #[cfg(any(
        any(
            feature = "educational-level-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "educationalLevel"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/educationalLevel")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/educationalLevel"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#educational_level: Vec<EducationalLevelProperty>,
    #[cfg(any(
        any(
            feature = "educational-use-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "educationalUse"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/educationalUse"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/educationalUse"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#educational_use: Vec<EducationalUseProperty>,
    #[cfg(any(
        any(
            feature = "encoding-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "encoding"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/encoding"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/encoding"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#encoding: Vec<EncodingProperty>,
    #[cfg(any(
        any(
            feature = "encoding-format-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "encodingFormat"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/encodingFormat"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/encodingFormat"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#encoding_format: Vec<EncodingFormatProperty>,
    #[cfg(any(
        any(
            feature = "encodings-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "encodings"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/encodings"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/encodings"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#encodings: Vec<EncodingsProperty>,
    #[cfg(any(
        any(
            feature = "example-of-work-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "exampleOfWork"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/exampleOfWork"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/exampleOfWork"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#example_of_work: Vec<ExampleOfWorkProperty>,
    #[cfg(any(
        any(
            feature = "expires-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "expires"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/expires"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/expires"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#expires: Vec<ExpiresProperty>,
    #[cfg(any(
        any(
            feature = "file-format-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "fileFormat"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/fileFormat"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/fileFormat"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#file_format: Vec<FileFormatProperty>,
    #[cfg(any(
        any(feature = "funder-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "funder"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/funder"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/funder"))]
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
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/funding"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/funding"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#funding: Vec<FundingProperty>,
    #[cfg(any(
        any(feature = "genre-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "genre"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/genre"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/genre"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#genre: Vec<GenreProperty>,
    #[cfg(any(
        any(
            feature = "has-part-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasPart"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/hasPart"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/hasPart"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_part: Vec<HasPartProperty>,
    #[cfg(any(
        any(
            feature = "headline-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "headline"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/headline"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/headline"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#headline: Vec<HeadlineProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/identifier"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/identifier"))]
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
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/image"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "in-language-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "inLanguage"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/inLanguage"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/inLanguage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#in_language: Vec<InLanguageProperty>,
    #[cfg(any(
        any(
            feature = "interaction-statistic-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "interactionStatistic"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/interactionStatistic")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/interactionStatistic")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#interaction_statistic: Vec<InteractionStatisticProperty>,
    #[cfg(any(
        any(
            feature = "interactivity-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "interactivityType"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/interactivityType")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/interactivityType")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#interactivity_type: Vec<InteractivityTypeProperty>,
    #[cfg(any(
        any(
            feature = "interpreted-as-claim-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "interpretedAsClaim"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/interpretedAsClaim")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/interpretedAsClaim")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#interpreted_as_claim: Vec<InterpretedAsClaimProperty>,
    #[cfg(any(
        any(
            feature = "is-accessible-for-free-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isAccessibleForFree"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/isAccessibleForFree")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/isAccessibleForFree")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
    #[cfg(any(
        any(
            feature = "is-based-on-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isBasedOn"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/isBasedOn"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isBasedOn"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_based_on: Vec<IsBasedOnProperty>,
    #[cfg(any(
        any(
            feature = "is-based-on-url-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isBasedOnUrl"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/isBasedOnUrl"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isBasedOnUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_based_on_url: Vec<IsBasedOnUrlProperty>,
    #[cfg(any(
        any(
            feature = "is-family-friendly-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isFamilyFriendly"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/isFamilyFriendly")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isFamilyFriendly"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_family_friendly: Vec<IsFamilyFriendlyProperty>,
    #[cfg(any(
        any(
            feature = "is-part-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isPartOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/isPartOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isPartOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#is_part_of: Vec<IsPartOfProperty>,
    #[cfg(any(
        any(
            feature = "keywords-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "keywords"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/keywords"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/keywords"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#keywords: Vec<KeywordsProperty>,
    #[cfg(any(
        any(
            feature = "last-reviewed-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "lastReviewed"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/lastReviewed"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/lastReviewed"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#last_reviewed: Vec<LastReviewedProperty>,
    #[cfg(any(
        any(
            feature = "learning-resource-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "learningResourceType"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/learningResourceType")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/learningResourceType")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#learning_resource_type: Vec<LearningResourceTypeProperty>,
    #[cfg(any(
        any(
            feature = "license-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "license"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/license"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/license"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#license: Vec<LicenseProperty>,
    #[cfg(any(
        any(
            feature = "location-created-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "locationCreated"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/locationCreated"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/locationCreated"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#location_created: Vec<LocationCreatedProperty>,
    #[cfg(any(
        any(
            feature = "main-content-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainContentOfPage"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/mainContentOfPage")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/mainContentOfPage")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#main_content_of_page: Vec<MainContentOfPageProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntity"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/mainEntity"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/mainEntity"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#main_entity: Vec<MainEntityProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/mainEntityOfPage")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(
            feature = "maintainer-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "maintainer"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/maintainer"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/maintainer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#maintainer: Vec<MaintainerProperty>,
    #[cfg(any(
        any(
            feature = "material-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "material"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/material"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/material"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#material: Vec<MaterialProperty>,
    #[cfg(any(
        any(
            feature = "material-extent-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "materialExtent"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/materialExtent"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/materialExtent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#material_extent: Vec<MaterialExtentProperty>,
    #[cfg(any(
        any(
            feature = "mentions-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mentions"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/mentions"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/mentions"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#mentions: Vec<MentionsProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/name"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(feature = "offers-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "offers"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/offers"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/offers"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#offers: Vec<OffersProperty>,
    #[cfg(any(
        any(
            feature = "pattern-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "pattern"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/pattern"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/pattern"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#pattern: Vec<PatternProperty>,
    #[cfg(any(
        any(
            feature = "position-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "position"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/position"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/position"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#position: Vec<PositionProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/potentialAction"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "primary-image-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "primaryImageOfPage"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/primaryImageOfPage")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/primaryImageOfPage")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#primary_image_of_page: Vec<PrimaryImageOfPageProperty>,
    #[cfg(any(
        any(
            feature = "producer-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "producer"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/producer"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/producer"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#producer: Vec<ProducerProperty>,
    #[cfg(any(
        any(
            feature = "provider-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "provider"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/provider"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/provider"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#provider: Vec<ProviderProperty>,
    #[cfg(any(
        any(
            feature = "publication-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publication"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/publication"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/publication"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#publication: Vec<PublicationProperty>,
    #[cfg(any(
        any(
            feature = "publisher-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publisher"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/publisher"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/publisher"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#publisher: Vec<PublisherProperty>,
    #[cfg(any(
        any(
            feature = "publisher-imprint-property-schema",
            feature = "bib-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publisherImprint"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/publisherImprint")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/publisherImprint"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#publisher_imprint: Vec<PublisherImprintProperty>,
    #[cfg(any(
        any(
            feature = "publishing-principles-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publishingPrinciples"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/publishingPrinciples")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/publishingPrinciples")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#publishing_principles: Vec<PublishingPrinciplesProperty>,
    #[cfg(any(
        any(
            feature = "recorded-at-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "recordedAt"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/recordedAt"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/recordedAt"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#recorded_at: Vec<RecordedAtProperty>,
    #[cfg(any(
        any(
            feature = "related-link-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "relatedLink"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/relatedLink"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/relatedLink"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#related_link: Vec<RelatedLinkProperty>,
    #[cfg(any(
        any(
            feature = "released-event-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "releasedEvent"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/releasedEvent"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/releasedEvent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#released_event: Vec<ReleasedEventProperty>,
    #[cfg(any(
        any(feature = "review-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "review"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/review"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/review"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#review: Vec<ReviewProperty>,
    #[cfg(any(
        any(
            feature = "reviewed-by-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "reviewedBy"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/reviewedBy"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/reviewedBy"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#reviewed_by: Vec<ReviewedByProperty>,
    #[cfg(any(
        any(
            feature = "reviews-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "reviews"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/reviews"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/reviews"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#reviews: Vec<ReviewsProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sameAs"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(
            feature = "schema-version-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "schemaVersion"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/schemaVersion"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/schemaVersion"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#schema_version: Vec<SchemaVersionProperty>,
    #[cfg(any(
        any(
            feature = "sd-date-published-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sdDatePublished"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sdDatePublished"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sdDatePublished"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#sd_date_published: Vec<SdDatePublishedProperty>,
    #[cfg(any(
        any(
            feature = "sd-license-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sdLicense"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sdLicense"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sdLicense"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#sd_license: Vec<SdLicenseProperty>,
    #[cfg(any(
        any(
            feature = "sd-publisher-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sdPublisher"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sdPublisher"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sdPublisher"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#sd_publisher: Vec<SdPublisherProperty>,
    #[cfg(any(
        any(
            feature = "significant-link-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "significantLink"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/significantLink"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/significantLink"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#significant_link: Vec<SignificantLinkProperty>,
    #[cfg(any(
        any(
            feature = "significant-links-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "significantLinks"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/significantLinks")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/significantLinks"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#significant_links: Vec<SignificantLinksProperty>,
    #[cfg(any(
        any(feature = "size-property-schema", feature = "pending-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "size"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/size"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/size"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#size: Vec<SizeProperty>,
    #[cfg(any(
        any(
            feature = "source-organization-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sourceOrganization"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/sourceOrganization")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/sourceOrganization")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#source_organization: Vec<SourceOrganizationProperty>,
    #[cfg(any(
        any(
            feature = "spatial-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "spatial"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/spatial"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/spatial"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#spatial: Vec<SpatialProperty>,
    #[cfg(any(
        any(
            feature = "spatial-coverage-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "spatialCoverage"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/spatialCoverage"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/spatialCoverage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#spatial_coverage: Vec<SpatialCoverageProperty>,
    #[cfg(any(
        any(
            feature = "speakable-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "speakable"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/speakable"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/speakable"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#speakable: Vec<SpeakableProperty>,
    #[cfg(any(
        any(
            feature = "specialty-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "specialty"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/specialty"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/specialty"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#specialty: Vec<SpecialtyProperty>,
    #[cfg(any(
        any(
            feature = "sponsor-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sponsor"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/sponsor"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/sponsor"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#sponsor: Vec<SponsorProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/subjectOf"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(
            feature = "teaches-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "teaches"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/teaches"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/teaches"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#teaches: Vec<TeachesProperty>,
    #[cfg(any(
        any(
            feature = "temporal-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "temporal"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/temporal"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/temporal"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#temporal: Vec<TemporalProperty>,
    #[cfg(any(
        any(
            feature = "temporal-coverage-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "temporalCoverage"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/temporalCoverage")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/temporalCoverage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#temporal_coverage: Vec<TemporalCoverageProperty>,
    #[cfg(any(
        any(feature = "text-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "text"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/text"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/text"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#text: Vec<TextProperty>,
    #[cfg(any(
        any(
            feature = "thumbnail-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "thumbnail"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/thumbnail"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/thumbnail"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#thumbnail: Vec<ThumbnailProperty>,
    #[cfg(any(
        any(
            feature = "thumbnail-url-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "thumbnailUrl"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/thumbnailUrl"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/thumbnailUrl"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#thumbnail_url: Vec<ThumbnailUrlProperty>,
    #[cfg(any(
        any(
            feature = "time-required-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "timeRequired"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/timeRequired"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/timeRequired"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#time_required: Vec<TimeRequiredProperty>,
    #[cfg(any(
        any(
            feature = "translation-of-work-property-schema",
            feature = "bib-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "translationOfWork"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/translationOfWork")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/translationOfWork")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#translation_of_work: Vec<TranslationOfWorkProperty>,
    #[cfg(any(
        any(
            feature = "translator-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "translator"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/translator"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/translator"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#translator: Vec<TranslatorProperty>,
    #[cfg(any(
        any(
            feature = "typical-age-range-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "typicalAgeRange"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/typicalAgeRange"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/typicalAgeRange"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#typical_age_range: Vec<TypicalAgeRangeProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/url"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        any(
            feature = "usage-info-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "usageInfo"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/usageInfo"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/usageInfo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#usage_info: Vec<UsageInfoProperty>,
    #[cfg(any(
        any(
            feature = "version-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "version"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/version"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/version"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#version: Vec<VersionProperty>,
    #[cfg(any(
        any(feature = "video-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "video"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/video"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/video"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#video: Vec<VideoProperty>,
    #[cfg(any(
        any(
            feature = "work-example-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "workExample"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/workExample"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/workExample"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#work_example: Vec<WorkExampleProperty>,
    #[cfg(any(
        any(
            feature = "work-translation-property-schema",
            feature = "bib-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "workTranslation"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/workTranslation"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/workTranslation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#work_translation: Vec<WorkTranslationProperty>,
}
