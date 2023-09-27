use super::*;
/// <https://schema.org/ParcelDelivery>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct ParcelDelivery {
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "additionalType"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "alternateName"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "carrier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "carrier"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#carrier: Vec<CarrierProperty>,
    #[cfg(any(
        any(
            feature = "delivery-address-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "deliveryAddress"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#delivery_address: Vec<DeliveryAddressProperty>,
    #[cfg(any(
        any(
            feature = "delivery-status-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "deliveryStatus"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#delivery_status: Vec<DeliveryStatusProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "description"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "disambiguatingDescription")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "expected-arrival-from-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "expectedArrivalFrom"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#expected_arrival_from: Vec<ExpectedArrivalFromProperty>,
    #[cfg(any(
        any(
            feature = "expected-arrival-until-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "expectedArrivalUntil"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#expected_arrival_until: Vec<ExpectedArrivalUntilProperty>,
    #[cfg(any(
        any(
            feature = "has-delivery-method-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "hasDeliveryMethod"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#has_delivery_method: Vec<HasDeliveryMethodProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "identifier"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "image"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "item-shipped-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "itemShipped"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#item_shipped: Vec<ItemShippedProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "name"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "origin-address-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "originAddress"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#origin_address: Vec<OriginAddressProperty>,
    #[cfg(any(
        any(
            feature = "part-of-order-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "partOfOrder"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#part_of_order: Vec<PartOfOrderProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "potentialAction"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "provider-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "provider"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#provider: Vec<ProviderProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "sameAs"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "subjectOf"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(
            feature = "tracking-number-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "trackingNumber"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#tracking_number: Vec<TrackingNumberProperty>,
    #[cfg(any(
        any(
            feature = "tracking-url-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "trackingUrl"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#tracking_url: Vec<TrackingUrlProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "url"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#url: Vec<UrlProperty>,
}
