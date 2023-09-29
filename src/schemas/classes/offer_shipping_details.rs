use super::*;
/// <https://schema.org/OfferShippingDetails>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OfferShippingDetails {
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
            feature = "delivery-time-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "deliveryTime"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/deliveryTime"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/deliveryTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#delivery_time: Vec<DeliveryTimeProperty>,
    #[cfg(any(
        any(feature = "depth-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "depth"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/depth"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/depth"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#depth: Vec<DepthProperty>,
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
            feature = "does-not-ship-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "doesNotShip"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/doesNotShip"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/doesNotShip"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#does_not_ship: Vec<DoesNotShipProperty>,
    #[cfg(any(
        any(feature = "height-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "height"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/height"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/height"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#height: Vec<HeightProperty>,
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
            feature = "shipping-destination-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "shippingDestination"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/shippingDestination")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/shippingDestination")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#shipping_destination: Vec<ShippingDestinationProperty>,
    #[cfg(any(
        any(
            feature = "shipping-label-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "shippingLabel"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/shippingLabel"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/shippingLabel"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#shipping_label: Vec<ShippingLabelProperty>,
    #[cfg(any(
        any(
            feature = "shipping-origin-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "shippingOrigin"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/shippingOrigin"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/shippingOrigin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#shipping_origin: Vec<ShippingOriginProperty>,
    #[cfg(any(
        any(
            feature = "shipping-rate-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "shippingRate"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/shippingRate"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/shippingRate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#shipping_rate: Vec<ShippingRateProperty>,
    #[cfg(any(
        any(
            feature = "shipping-settings-link-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "shippingSettingsLink"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/shippingSettingsLink")
    )]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "http://schema.org/shippingSettingsLink")
    )]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#shipping_settings_link: Vec<ShippingSettingsLinkProperty>,
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
            feature = "transit-time-label-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "transitTimeLabel"))]
    #[cfg_attr(
        feature = "serde",
        serde(alias = "https://schema.org/transitTimeLabel")
    )]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/transitTimeLabel"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#transit_time_label: Vec<TransitTimeLabelProperty>,
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
        any(feature = "weight-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "weight"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/weight"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/weight"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#weight: Vec<WeightProperty>,
    #[cfg(any(
        any(feature = "width-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "width"))]
    #[cfg_attr(feature = "serde", serde(alias = "https://schema.org/width"))]
    #[cfg_attr(feature = "serde", serde(alias = "http://schema.org/width"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    #[cfg_attr(
        feature = "serde",
        serde(with = "serde_with::As::<serde_with::OneOrMany<serde_with::Same>>")
    )]
    pub r#width: Vec<WidthProperty>,
}
