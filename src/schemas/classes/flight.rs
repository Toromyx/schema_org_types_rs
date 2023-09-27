use super::*;
/// <https://schema.org/Flight>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Flight {
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "aircraft-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "aircraft"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#aircraft: Vec<AircraftProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "arrival-airport-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "arrivalAirport"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#arrival_airport: Vec<ArrivalAirportProperty>,
    #[cfg(any(
        any(
            feature = "arrival-gate-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "arrivalGate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#arrival_gate: Vec<ArrivalGateProperty>,
    #[cfg(any(
        any(
            feature = "arrival-terminal-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "arrivalTerminal"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#arrival_terminal: Vec<ArrivalTerminalProperty>,
    #[cfg(any(
        any(
            feature = "arrival-time-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "arrivalTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#arrival_time: Vec<ArrivalTimeProperty>,
    #[cfg(any(
        any(
            feature = "boarding-policy-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "boardingPolicy"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#boarding_policy: Vec<BoardingPolicyProperty>,
    #[cfg(any(
        any(
            feature = "carrier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "carrier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#carrier: Vec<CarrierProperty>,
    #[cfg(any(
        any(
            feature = "departure-airport-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "departureAirport"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#departure_airport: Vec<DepartureAirportProperty>,
    #[cfg(any(
        any(
            feature = "departure-gate-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "departureGate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#departure_gate: Vec<DepartureGateProperty>,
    #[cfg(any(
        any(
            feature = "departure-terminal-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "departureTerminal"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#departure_terminal: Vec<DepartureTerminalProperty>,
    #[cfg(any(
        any(
            feature = "departure-time-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "departureTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#departure_time: Vec<DepartureTimeProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "estimated-flight-duration-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "estimatedFlightDuration"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#estimated_flight_duration: Vec<EstimatedFlightDurationProperty>,
    #[cfg(any(
        any(
            feature = "flight-distance-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "flightDistance"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#flight_distance: Vec<FlightDistanceProperty>,
    #[cfg(any(
        any(
            feature = "flight-number-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "flightNumber"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#flight_number: Vec<FlightNumberProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "itinerary-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "itinerary"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#itinerary: Vec<ItineraryProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(
            feature = "meal-service-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mealService"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#meal_service: Vec<MealServiceProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(feature = "offers-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "offers"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#offers: Vec<OffersProperty>,
    #[cfg(any(
        any(
            feature = "part-of-trip-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "partOfTrip"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#part_of_trip: Vec<PartOfTripProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "provider-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "provider"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#provider: Vec<ProviderProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(feature = "seller-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "seller"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#seller: Vec<SellerProperty>,
    #[cfg(any(
        any(
            feature = "sub-trip-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subTrip"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sub_trip: Vec<SubTripProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(
            feature = "trip-origin-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "tripOrigin"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#trip_origin: Vec<TripOriginProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#url: Vec<UrlProperty>,
    #[cfg(any(
        any(
            feature = "web-checkin-time-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "webCheckinTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#web_checkin_time: Vec<WebCheckinTimeProperty>,
}
