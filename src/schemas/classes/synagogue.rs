use super::*;
/// <https://schema.org/Synagogue>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Synagogue {
    #[cfg(any(
        any(
            feature = "additional-property-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalProperty"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/additionalProperty")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/additionalProperty")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#additional_property: Vec<AdditionalPropertyProperty>,
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
            feature = "address-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "address"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/address"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/address"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#address: Vec<AddressProperty>,
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
            feature = "amenity-feature-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "amenityFeature"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/amenityFeature"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/amenityFeature"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#amenity_feature: Vec<AmenityFeatureProperty>,
    #[cfg(any(
        any(
            feature = "branch-code-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "branchCode"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/branchCode"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/branchCode"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#branch_code: Vec<BranchCodeProperty>,
    #[cfg(any(
        any(
            feature = "contained-in-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "containedIn"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/containedIn"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/containedIn"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#contained_in: Vec<ContainedInProperty>,
    #[cfg(any(
        any(
            feature = "contained-in-place-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "containedInPlace"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/containedInPlace")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/containedInPlace"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#contained_in_place: Vec<ContainedInPlaceProperty>,
    #[cfg(any(
        any(
            feature = "contains-place-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "containsPlace"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/containsPlace"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/containsPlace"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#contains_place: Vec<ContainsPlaceProperty>,
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
        any(feature = "event-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "event"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/event"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/event"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#event: Vec<EventProperty>,
    #[cfg(any(
        any(feature = "events-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "events"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/events"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/events"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#events: Vec<EventsProperty>,
    #[cfg(any(
        any(
            feature = "fax-number-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "faxNumber"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/faxNumber"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/faxNumber"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#fax_number: Vec<FaxNumberProperty>,
    #[cfg(any(
        any(feature = "geo-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geo"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geo"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo: Vec<GeoProperty>,
    #[cfg(any(
        any(
            feature = "geo-contains-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoContains"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoContains"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoContains"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_contains: Vec<GeoContainsProperty>,
    #[cfg(any(
        any(
            feature = "geo-covered-by-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoCoveredBy"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoCoveredBy"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoCoveredBy"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_covered_by: Vec<GeoCoveredByProperty>,
    #[cfg(any(
        any(
            feature = "geo-covers-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoCovers"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoCovers"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoCovers"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_covers: Vec<GeoCoversProperty>,
    #[cfg(any(
        any(
            feature = "geo-crosses-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoCrosses"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoCrosses"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoCrosses"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_crosses: Vec<GeoCrossesProperty>,
    #[cfg(any(
        any(
            feature = "geo-disjoint-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoDisjoint"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoDisjoint"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoDisjoint"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_disjoint: Vec<GeoDisjointProperty>,
    #[cfg(any(
        any(
            feature = "geo-equals-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoEquals"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoEquals"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoEquals"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_equals: Vec<GeoEqualsProperty>,
    #[cfg(any(
        any(
            feature = "geo-intersects-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoIntersects"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoIntersects"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoIntersects"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_intersects: Vec<GeoIntersectsProperty>,
    #[cfg(any(
        any(
            feature = "geo-overlaps-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoOverlaps"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoOverlaps"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoOverlaps"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_overlaps: Vec<GeoOverlapsProperty>,
    #[cfg(any(
        any(
            feature = "geo-touches-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoTouches"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoTouches"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoTouches"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_touches: Vec<GeoTouchesProperty>,
    #[cfg(any(
        any(
            feature = "geo-within-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "geoWithin"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/geoWithin"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/geoWithin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#geo_within: Vec<GeoWithinProperty>,
    #[cfg(any(
        any(
            feature = "global-location-number-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "globalLocationNumber"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/globalLocationNumber")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/globalLocationNumber")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#global_location_number: Vec<GlobalLocationNumberProperty>,
    #[cfg(any(
        any(
            feature = "has-drive-through-service-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasDriveThroughService"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/hasDriveThroughService")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/hasDriveThroughService")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_drive_through_service: Vec<HasDriveThroughServiceProperty>,
    #[cfg(any(
        any(
            feature = "has-map-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "hasMap"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/hasMap"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/hasMap"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#has_map: Vec<HasMapProperty>,
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
            feature = "isic-v-4-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "isicV4"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/isicV4"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/isicV4"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#isic_v_4: Vec<IsicV4Property>,
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
            feature = "latitude-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "latitude"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/latitude"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/latitude"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#latitude: Vec<LatitudeProperty>,
    #[cfg(any(
        any(feature = "logo-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "logo"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/logo"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/logo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#logo: Vec<LogoProperty>,
    #[cfg(any(
        any(
            feature = "longitude-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "longitude"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/longitude"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/longitude"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#longitude: Vec<LongitudeProperty>,
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
        any(feature = "map-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "map"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/map"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/map"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#map: Vec<MapProperty>,
    #[cfg(any(
        any(feature = "maps-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "maps"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/maps"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/maps"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#maps: Vec<MapsProperty>,
    #[cfg(any(
        any(
            feature = "maximum-attendee-capacity-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "maximumAttendeeCapacity"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/maximumAttendeeCapacity")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/maximumAttendeeCapacity")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#maximum_attendee_capacity: Vec<MaximumAttendeeCapacityProperty>,
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
        any(
            feature = "opening-hours-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "openingHours"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/openingHours"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/openingHours"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#opening_hours: Vec<OpeningHoursProperty>,
    #[cfg(any(
        any(
            feature = "opening-hours-specification-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "openingHoursSpecification"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/openingHoursSpecification")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/openingHoursSpecification")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#opening_hours_specification: Vec<OpeningHoursSpecificationProperty>,
    #[cfg(any(
        any(feature = "photo-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "photo"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/photo"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/photo"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#photo: Vec<PhotoProperty>,
    #[cfg(any(
        any(feature = "photos-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "photos"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/photos"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/photos"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#photos: Vec<PhotosProperty>,
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
            feature = "public-access-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "publicAccess"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/publicAccess"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/publicAccess"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#public_access: Vec<PublicAccessProperty>,
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
        any(feature = "slogan-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "slogan"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/slogan"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/slogan"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#slogan: Vec<SloganProperty>,
    #[cfg(any(
        any(
            feature = "smoking-allowed-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "smokingAllowed"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/smokingAllowed"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/smokingAllowed"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#smoking_allowed: Vec<SmokingAllowedProperty>,
    #[cfg(any(
        any(
            feature = "special-opening-hours-specification-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "specialOpeningHoursSpecification"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/specialOpeningHoursSpecification")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/specialOpeningHoursSpecification")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#special_opening_hours_specification: Vec<SpecialOpeningHoursSpecificationProperty>,
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
            feature = "telephone-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "telephone"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/telephone"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/telephone"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#telephone: Vec<TelephoneProperty>,
    #[cfg(any(
        any(
            feature = "tour-booking-page-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "tourBookingPage"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/tourBookingPage"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/tourBookingPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#tour_booking_page: Vec<TourBookingPageProperty>,
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
}
