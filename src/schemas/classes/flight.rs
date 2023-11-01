use super::*;
/// <https://schema.org/Flight>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Flight {
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
			feature = "aircraft-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#aircraft: Vec<AircraftProperty>,
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
			feature = "arrival-airport-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#arrival_airport: Vec<ArrivalAirportProperty>,
	#[cfg(any(
		any(
			feature = "arrival-gate-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#arrival_gate: Vec<ArrivalGateProperty>,
	#[cfg(any(
		any(
			feature = "arrival-terminal-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#arrival_terminal: Vec<ArrivalTerminalProperty>,
	#[cfg(any(
		any(
			feature = "arrival-time-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#arrival_time: Vec<ArrivalTimeProperty>,
	#[cfg(any(
		any(
			feature = "boarding-policy-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#boarding_policy: Vec<BoardingPolicyProperty>,
	#[cfg(any(
		any(
			feature = "carrier-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#carrier: Vec<CarrierProperty>,
	#[cfg(any(
		any(
			feature = "departure-airport-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#departure_airport: Vec<DepartureAirportProperty>,
	#[cfg(any(
		any(
			feature = "departure-gate-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#departure_gate: Vec<DepartureGateProperty>,
	#[cfg(any(
		any(
			feature = "departure-terminal-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#departure_terminal: Vec<DepartureTerminalProperty>,
	#[cfg(any(
		any(
			feature = "departure-time-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#departure_time: Vec<DepartureTimeProperty>,
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
			feature = "disambiguating-description-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	#[cfg(any(
		any(
			feature = "estimated-flight-duration-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#estimated_flight_duration: Vec<EstimatedFlightDurationProperty>,
	#[cfg(any(
		any(
			feature = "flight-distance-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#flight_distance: Vec<FlightDistanceProperty>,
	#[cfg(any(
		any(
			feature = "flight-number-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#flight_number: Vec<FlightNumberProperty>,
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
			feature = "itinerary-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#itinerary: Vec<ItineraryProperty>,
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
			feature = "meal-service-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#meal_service: Vec<MealServiceProperty>,
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
			feature = "part-of-trip-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#part_of_trip: Vec<PartOfTripProperty>,
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
			feature = "provider-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#provider: Vec<ProviderProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(feature = "seller-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#seller: Vec<SellerProperty>,
	#[cfg(any(
		any(
			feature = "sub-trip-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#sub_trip: Vec<SubTripProperty>,
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
			feature = "trip-origin-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#trip_origin: Vec<TripOriginProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
	#[cfg(any(
		any(
			feature = "web-checkin-time-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#web_checkin_time: Vec<WebCheckinTimeProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for Flight {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
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
						feature = "aircraft-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#aircraft) as usize
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
						feature = "arrival-airport-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#arrival_airport) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "arrival-gate-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#arrival_gate) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "arrival-terminal-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#arrival_terminal) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "arrival-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#arrival_time) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "boarding-policy-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#boarding_policy) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "carrier-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#carrier) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "departure-airport-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#departure_airport) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "departure-gate-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#departure_gate) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "departure-terminal-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#departure_terminal) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "departure-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#departure_time) as usize
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
						feature = "estimated-flight-duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#estimated_flight_duration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "flight-distance-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#flight_distance) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "flight-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#flight_number) as usize
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
						feature = "itinerary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#itinerary) as usize
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
						feature = "meal-service-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#meal_service) as usize
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
						feature = "part-of-trip-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#part_of_trip) as usize
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
						feature = "seller-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#seller) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sub-trip-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sub_trip) as usize
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
						feature = "trip-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#trip_origin) as usize
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
						feature = "web-checkin-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#web_checkin_time) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Flight", len)?;
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
					feature = "aircraft-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#aircraft) {
				serialize_struct.serialize_field("aircraft", {
					struct SerializeWith<'a>(&'a Vec<AircraftProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#aircraft)
				})?;
			} else {
				serialize_struct.skip_field("aircraft")?;
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
					feature = "arrival-airport-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#arrival_airport) {
				serialize_struct.serialize_field("arrivalAirport", {
					struct SerializeWith<'a>(&'a Vec<ArrivalAirportProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#arrival_airport)
				})?;
			} else {
				serialize_struct.skip_field("arrivalAirport")?;
			}
			#[cfg(any(
				any(
					feature = "arrival-gate-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#arrival_gate) {
				serialize_struct.serialize_field("arrivalGate", {
					struct SerializeWith<'a>(&'a Vec<ArrivalGateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#arrival_gate)
				})?;
			} else {
				serialize_struct.skip_field("arrivalGate")?;
			}
			#[cfg(any(
				any(
					feature = "arrival-terminal-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#arrival_terminal) {
				serialize_struct.serialize_field("arrivalTerminal", {
					struct SerializeWith<'a>(&'a Vec<ArrivalTerminalProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#arrival_terminal)
				})?;
			} else {
				serialize_struct.skip_field("arrivalTerminal")?;
			}
			#[cfg(any(
				any(
					feature = "arrival-time-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#arrival_time) {
				serialize_struct.serialize_field("arrivalTime", {
					struct SerializeWith<'a>(&'a Vec<ArrivalTimeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#arrival_time)
				})?;
			} else {
				serialize_struct.skip_field("arrivalTime")?;
			}
			#[cfg(any(
				any(
					feature = "boarding-policy-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#boarding_policy) {
				serialize_struct.serialize_field("boardingPolicy", {
					struct SerializeWith<'a>(&'a Vec<BoardingPolicyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#boarding_policy)
				})?;
			} else {
				serialize_struct.skip_field("boardingPolicy")?;
			}
			#[cfg(any(
				any(
					feature = "carrier-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#carrier) {
				serialize_struct.serialize_field("carrier", {
					struct SerializeWith<'a>(&'a Vec<CarrierProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#carrier)
				})?;
			} else {
				serialize_struct.skip_field("carrier")?;
			}
			#[cfg(any(
				any(
					feature = "departure-airport-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#departure_airport) {
				serialize_struct.serialize_field("departureAirport", {
					struct SerializeWith<'a>(&'a Vec<DepartureAirportProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#departure_airport)
				})?;
			} else {
				serialize_struct.skip_field("departureAirport")?;
			}
			#[cfg(any(
				any(
					feature = "departure-gate-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#departure_gate) {
				serialize_struct.serialize_field("departureGate", {
					struct SerializeWith<'a>(&'a Vec<DepartureGateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#departure_gate)
				})?;
			} else {
				serialize_struct.skip_field("departureGate")?;
			}
			#[cfg(any(
				any(
					feature = "departure-terminal-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#departure_terminal) {
				serialize_struct.serialize_field("departureTerminal", {
					struct SerializeWith<'a>(&'a Vec<DepartureTerminalProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#departure_terminal)
				})?;
			} else {
				serialize_struct.skip_field("departureTerminal")?;
			}
			#[cfg(any(
				any(
					feature = "departure-time-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#departure_time) {
				serialize_struct.serialize_field("departureTime", {
					struct SerializeWith<'a>(&'a Vec<DepartureTimeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#departure_time)
				})?;
			} else {
				serialize_struct.skip_field("departureTime")?;
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
					feature = "estimated-flight-duration-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#estimated_flight_duration) {
				serialize_struct.serialize_field("estimatedFlightDuration", {
					struct SerializeWith<'a>(&'a Vec<EstimatedFlightDurationProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#estimated_flight_duration)
				})?;
			} else {
				serialize_struct.skip_field("estimatedFlightDuration")?;
			}
			#[cfg(any(
				any(
					feature = "flight-distance-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#flight_distance) {
				serialize_struct.serialize_field("flightDistance", {
					struct SerializeWith<'a>(&'a Vec<FlightDistanceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#flight_distance)
				})?;
			} else {
				serialize_struct.skip_field("flightDistance")?;
			}
			#[cfg(any(
				any(
					feature = "flight-number-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#flight_number) {
				serialize_struct.serialize_field("flightNumber", {
					struct SerializeWith<'a>(&'a Vec<FlightNumberProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#flight_number)
				})?;
			} else {
				serialize_struct.skip_field("flightNumber")?;
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
					feature = "itinerary-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#itinerary) {
				serialize_struct.serialize_field("itinerary", {
					struct SerializeWith<'a>(&'a Vec<ItineraryProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#itinerary)
				})?;
			} else {
				serialize_struct.skip_field("itinerary")?;
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
					feature = "meal-service-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#meal_service) {
				serialize_struct.serialize_field("mealService", {
					struct SerializeWith<'a>(&'a Vec<MealServiceProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#meal_service)
				})?;
			} else {
				serialize_struct.skip_field("mealService")?;
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
					feature = "part-of-trip-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#part_of_trip) {
				serialize_struct.serialize_field("partOfTrip", {
					struct SerializeWith<'a>(&'a Vec<PartOfTripProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#part_of_trip)
				})?;
			} else {
				serialize_struct.skip_field("partOfTrip")?;
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
				any(feature = "seller-property-schema", feature = "general-schema-section"),
				doc
			))]
			if !Vec::is_empty(&self.r#seller) {
				serialize_struct.serialize_field("seller", {
					struct SerializeWith<'a>(&'a Vec<SellerProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#seller)
				})?;
			} else {
				serialize_struct.skip_field("seller")?;
			}
			#[cfg(any(
				any(
					feature = "sub-trip-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#sub_trip) {
				serialize_struct.serialize_field("subTrip", {
					struct SerializeWith<'a>(&'a Vec<SubTripProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#sub_trip)
				})?;
			} else {
				serialize_struct.skip_field("subTrip")?;
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
					feature = "trip-origin-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#trip_origin) {
				serialize_struct.serialize_field("tripOrigin", {
					struct SerializeWith<'a>(&'a Vec<TripOriginProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#trip_origin)
				})?;
			} else {
				serialize_struct.skip_field("tripOrigin")?;
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
					feature = "web-checkin-time-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#web_checkin_time) {
				serialize_struct.serialize_field("webCheckinTime", {
					struct SerializeWith<'a>(&'a Vec<WebCheckinTimeProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#web_checkin_time)
				})?;
			} else {
				serialize_struct.skip_field("webCheckinTime")?;
			}
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Flight {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
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
						feature = "aircraft-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Aircraft,
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
						feature = "arrival-airport-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ArrivalAirport,
				#[cfg(any(
					any(
						feature = "arrival-gate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ArrivalGate,
				#[cfg(any(
					any(
						feature = "arrival-terminal-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ArrivalTerminal,
				#[cfg(any(
					any(
						feature = "arrival-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				ArrivalTime,
				#[cfg(any(
					any(
						feature = "boarding-policy-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				BoardingPolicy,
				#[cfg(any(
					any(
						feature = "carrier-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Carrier,
				#[cfg(any(
					any(
						feature = "departure-airport-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DepartureAirport,
				#[cfg(any(
					any(
						feature = "departure-gate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DepartureGate,
				#[cfg(any(
					any(
						feature = "departure-terminal-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DepartureTerminal,
				#[cfg(any(
					any(
						feature = "departure-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DepartureTime,
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
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DisambiguatingDescription,
				#[cfg(any(
					any(
						feature = "estimated-flight-duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EstimatedFlightDuration,
				#[cfg(any(
					any(
						feature = "flight-distance-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FlightDistance,
				#[cfg(any(
					any(
						feature = "flight-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				FlightNumber,
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
						feature = "itinerary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Itinerary,
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
						feature = "meal-service-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MealService,
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
						feature = "part-of-trip-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				PartOfTrip,
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
						feature = "provider-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Provider,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(feature = "seller-property-schema", feature = "general-schema-section"),
					doc
				))]
				Seller,
				#[cfg(any(
					any(
						feature = "sub-trip-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SubTrip,
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
						feature = "trip-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				TripOrigin,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
				#[cfg(any(
					any(
						feature = "web-checkin-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				WebCheckinTime,
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
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"additionalType" => Ok(Field::AdditionalType),
						#[cfg(any(
							any(
								feature = "aircraft-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"aircraft" => Ok(Field::Aircraft),
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
								feature = "arrival-airport-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"arrivalAirport" => Ok(Field::ArrivalAirport),
						#[cfg(any(
							any(
								feature = "arrival-gate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"arrivalGate" => Ok(Field::ArrivalGate),
						#[cfg(any(
							any(
								feature = "arrival-terminal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"arrivalTerminal" => Ok(Field::ArrivalTerminal),
						#[cfg(any(
							any(
								feature = "arrival-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"arrivalTime" => Ok(Field::ArrivalTime),
						#[cfg(any(
							any(
								feature = "boarding-policy-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"boardingPolicy" => Ok(Field::BoardingPolicy),
						#[cfg(any(
							any(
								feature = "carrier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"carrier" => Ok(Field::Carrier),
						#[cfg(any(
							any(
								feature = "departure-airport-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"departureAirport" => Ok(Field::DepartureAirport),
						#[cfg(any(
							any(
								feature = "departure-gate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"departureGate" => Ok(Field::DepartureGate),
						#[cfg(any(
							any(
								feature = "departure-terminal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"departureTerminal" => Ok(Field::DepartureTerminal),
						#[cfg(any(
							any(
								feature = "departure-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"departureTime" => Ok(Field::DepartureTime),
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
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						#[cfg(any(
							any(
								feature = "estimated-flight-duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"estimatedFlightDuration" => Ok(Field::EstimatedFlightDuration),
						#[cfg(any(
							any(
								feature = "flight-distance-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"flightDistance" => Ok(Field::FlightDistance),
						#[cfg(any(
							any(
								feature = "flight-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"flightNumber" => Ok(Field::FlightNumber),
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
								feature = "itinerary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"itinerary" => Ok(Field::Itinerary),
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
								feature = "meal-service-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mealService" => Ok(Field::MealService),
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
								feature = "part-of-trip-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"partOfTrip" => Ok(Field::PartOfTrip),
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
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"provider" => Ok(Field::Provider),
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
								feature = "seller-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"seller" => Ok(Field::Seller),
						#[cfg(any(
							any(
								feature = "sub-trip-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"subTrip" => Ok(Field::SubTrip),
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
								feature = "trip-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"tripOrigin" => Ok(Field::TripOrigin),
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
								feature = "web-checkin-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"webCheckinTime" => Ok(Field::WebCheckinTime),
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
								feature = "additional-type-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"additionalType" => Ok(Field::AdditionalType),
						#[cfg(any(
							any(
								feature = "aircraft-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"aircraft" => Ok(Field::Aircraft),
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
								feature = "arrival-airport-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"arrivalAirport" => Ok(Field::ArrivalAirport),
						#[cfg(any(
							any(
								feature = "arrival-gate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"arrivalGate" => Ok(Field::ArrivalGate),
						#[cfg(any(
							any(
								feature = "arrival-terminal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"arrivalTerminal" => Ok(Field::ArrivalTerminal),
						#[cfg(any(
							any(
								feature = "arrival-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"arrivalTime" => Ok(Field::ArrivalTime),
						#[cfg(any(
							any(
								feature = "boarding-policy-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"boardingPolicy" => Ok(Field::BoardingPolicy),
						#[cfg(any(
							any(
								feature = "carrier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"carrier" => Ok(Field::Carrier),
						#[cfg(any(
							any(
								feature = "departure-airport-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"departureAirport" => Ok(Field::DepartureAirport),
						#[cfg(any(
							any(
								feature = "departure-gate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"departureGate" => Ok(Field::DepartureGate),
						#[cfg(any(
							any(
								feature = "departure-terminal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"departureTerminal" => Ok(Field::DepartureTerminal),
						#[cfg(any(
							any(
								feature = "departure-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"departureTime" => Ok(Field::DepartureTime),
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
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						#[cfg(any(
							any(
								feature = "estimated-flight-duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"estimatedFlightDuration" => Ok(Field::EstimatedFlightDuration),
						#[cfg(any(
							any(
								feature = "flight-distance-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"flightDistance" => Ok(Field::FlightDistance),
						#[cfg(any(
							any(
								feature = "flight-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"flightNumber" => Ok(Field::FlightNumber),
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
								feature = "itinerary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"itinerary" => Ok(Field::Itinerary),
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
								feature = "meal-service-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mealService" => Ok(Field::MealService),
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
								feature = "part-of-trip-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"partOfTrip" => Ok(Field::PartOfTrip),
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
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"provider" => Ok(Field::Provider),
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
								feature = "seller-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"seller" => Ok(Field::Seller),
						#[cfg(any(
							any(
								feature = "sub-trip-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"subTrip" => Ok(Field::SubTrip),
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
								feature = "trip-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"tripOrigin" => Ok(Field::TripOrigin),
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
								feature = "web-checkin-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"webCheckinTime" => Ok(Field::WebCheckinTime),
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
				type Value = Flight;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema Flight")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
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
							feature = "aircraft-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#aircraft_property = None;
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
							feature = "arrival-airport-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#arrival_airport_property = None;
					#[cfg(any(
						any(
							feature = "arrival-gate-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#arrival_gate_property = None;
					#[cfg(any(
						any(
							feature = "arrival-terminal-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#arrival_terminal_property = None;
					#[cfg(any(
						any(
							feature = "arrival-time-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#arrival_time_property = None;
					#[cfg(any(
						any(
							feature = "boarding-policy-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#boarding_policy_property = None;
					#[cfg(any(
						any(
							feature = "carrier-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#carrier_property = None;
					#[cfg(any(
						any(
							feature = "departure-airport-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#departure_airport_property = None;
					#[cfg(any(
						any(
							feature = "departure-gate-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#departure_gate_property = None;
					#[cfg(any(
						any(
							feature = "departure-terminal-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#departure_terminal_property = None;
					#[cfg(any(
						any(
							feature = "departure-time-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#departure_time_property = None;
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
							feature = "disambiguating-description-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#disambiguating_description_property = None;
					#[cfg(any(
						any(
							feature = "estimated-flight-duration-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#estimated_flight_duration_property = None;
					#[cfg(any(
						any(
							feature = "flight-distance-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#flight_distance_property = None;
					#[cfg(any(
						any(
							feature = "flight-number-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#flight_number_property = None;
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
							feature = "itinerary-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#itinerary_property = None;
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
							feature = "meal-service-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#meal_service_property = None;
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
							feature = "part-of-trip-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#part_of_trip_property = None;
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
							feature = "provider-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#provider_property = None;
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
							feature = "seller-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#seller_property = None;
					#[cfg(any(
						any(
							feature = "sub-trip-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#sub_trip_property = None;
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
							feature = "trip-origin-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#trip_origin_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					#[cfg(any(
						any(
							feature = "web-checkin-time-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#web_checkin_time_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
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
									feature = "aircraft-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Aircraft => {
								if r#aircraft_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"aircraft",
									));
								}
								r#aircraft_property = Some({
									struct DeserializeWith(Vec<AircraftProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "arrival-airport-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ArrivalAirport => {
								if r#arrival_airport_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"arrivalAirport",
									));
								}
								r#arrival_airport_property = Some({
									struct DeserializeWith(Vec<ArrivalAirportProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "arrival-gate-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ArrivalGate => {
								if r#arrival_gate_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"arrivalGate",
									));
								}
								r#arrival_gate_property = Some({
									struct DeserializeWith(Vec<ArrivalGateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "arrival-terminal-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ArrivalTerminal => {
								if r#arrival_terminal_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"arrivalTerminal",
									));
								}
								r#arrival_terminal_property = Some({
									struct DeserializeWith(Vec<ArrivalTerminalProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "arrival-time-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::ArrivalTime => {
								if r#arrival_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"arrivalTime",
									));
								}
								r#arrival_time_property = Some({
									struct DeserializeWith(Vec<ArrivalTimeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "boarding-policy-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::BoardingPolicy => {
								if r#boarding_policy_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"boardingPolicy",
									));
								}
								r#boarding_policy_property = Some({
									struct DeserializeWith(Vec<BoardingPolicyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "carrier-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Carrier => {
								if r#carrier_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"carrier",
									));
								}
								r#carrier_property = Some({
									struct DeserializeWith(Vec<CarrierProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "departure-airport-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DepartureAirport => {
								if r#departure_airport_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"departureAirport",
									));
								}
								r#departure_airport_property = Some({
									struct DeserializeWith(Vec<DepartureAirportProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "departure-gate-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DepartureGate => {
								if r#departure_gate_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"departureGate",
									));
								}
								r#departure_gate_property = Some({
									struct DeserializeWith(Vec<DepartureGateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "departure-terminal-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DepartureTerminal => {
								if r#departure_terminal_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"departureTerminal",
									));
								}
								r#departure_terminal_property = Some({
									struct DeserializeWith(Vec<DepartureTerminalProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "departure-time-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DepartureTime => {
								if r#departure_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"departureTime",
									));
								}
								r#departure_time_property = Some({
									struct DeserializeWith(Vec<DepartureTimeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "estimated-flight-duration-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EstimatedFlightDuration => {
								if r#estimated_flight_duration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"estimatedFlightDuration",
									));
								}
								r#estimated_flight_duration_property = Some({
									struct DeserializeWith(Vec<EstimatedFlightDurationProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "flight-distance-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FlightDistance => {
								if r#flight_distance_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"flightDistance",
									));
								}
								r#flight_distance_property = Some({
									struct DeserializeWith(Vec<FlightDistanceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "flight-number-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::FlightNumber => {
								if r#flight_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"flightNumber",
									));
								}
								r#flight_number_property = Some({
									struct DeserializeWith(Vec<FlightNumberProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "itinerary-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::Itinerary => {
								if r#itinerary_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"itinerary",
									));
								}
								r#itinerary_property = Some({
									struct DeserializeWith(Vec<ItineraryProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "meal-service-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::MealService => {
								if r#meal_service_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"mealService",
									));
								}
								r#meal_service_property = Some({
									struct DeserializeWith(Vec<MealServiceProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "part-of-trip-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::PartOfTrip => {
								if r#part_of_trip_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"partOfTrip",
									));
								}
								r#part_of_trip_property = Some({
									struct DeserializeWith(Vec<PartOfTripProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "seller-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::Seller => {
								if r#seller_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("seller"));
								}
								r#seller_property = Some({
									struct DeserializeWith(Vec<SellerProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "sub-trip-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::SubTrip => {
								if r#sub_trip_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subTrip",
									));
								}
								r#sub_trip_property = Some({
									struct DeserializeWith(Vec<SubTripProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "trip-origin-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::TripOrigin => {
								if r#trip_origin_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"tripOrigin",
									));
								}
								r#trip_origin_property = Some({
									struct DeserializeWith(Vec<TripOriginProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
									feature = "web-checkin-time-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::WebCheckinTime => {
								if r#web_checkin_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"webCheckinTime",
									));
								}
								r#web_checkin_time_property = Some({
									struct DeserializeWith(Vec<WebCheckinTimeProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(Flight {
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
								feature = "aircraft-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#aircraft: r#aircraft_property.unwrap_or_default(),
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
								feature = "arrival-airport-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#arrival_airport: r#arrival_airport_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "arrival-gate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#arrival_gate: r#arrival_gate_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "arrival-terminal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#arrival_terminal: r#arrival_terminal_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "arrival-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#arrival_time: r#arrival_time_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "boarding-policy-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#boarding_policy: r#boarding_policy_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "carrier-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#carrier: r#carrier_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "departure-airport-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#departure_airport: r#departure_airport_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "departure-gate-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#departure_gate: r#departure_gate_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "departure-terminal-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#departure_terminal: r#departure_terminal_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "departure-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#departure_time: r#departure_time_property.unwrap_or_default(),
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
								feature = "disambiguating-description-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "estimated-flight-duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#estimated_flight_duration: r#estimated_flight_duration_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "flight-distance-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#flight_distance: r#flight_distance_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "flight-number-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#flight_number: r#flight_number_property.unwrap_or_default(),
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
								feature = "itinerary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#itinerary: r#itinerary_property.unwrap_or_default(),
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
								feature = "meal-service-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#meal_service: r#meal_service_property.unwrap_or_default(),
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
								feature = "part-of-trip-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#part_of_trip: r#part_of_trip_property.unwrap_or_default(),
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
								feature = "provider-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#provider: r#provider_property.unwrap_or_default(),
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
								feature = "seller-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#seller: r#seller_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sub-trip-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#sub_trip: r#sub_trip_property.unwrap_or_default(),
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
								feature = "trip-origin-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#trip_origin: r#trip_origin_property.unwrap_or_default(),
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
								feature = "web-checkin-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#web_checkin_time: r#web_checkin_time_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
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
						feature = "aircraft-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"aircraft",
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
						feature = "arrival-airport-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"arrivalAirport",
				#[cfg(any(
					any(
						feature = "arrival-gate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"arrivalGate",
				#[cfg(any(
					any(
						feature = "arrival-terminal-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"arrivalTerminal",
				#[cfg(any(
					any(
						feature = "arrival-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"arrivalTime",
				#[cfg(any(
					any(
						feature = "boarding-policy-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"boardingPolicy",
				#[cfg(any(
					any(
						feature = "carrier-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"carrier",
				#[cfg(any(
					any(
						feature = "departure-airport-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"departureAirport",
				#[cfg(any(
					any(
						feature = "departure-gate-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"departureGate",
				#[cfg(any(
					any(
						feature = "departure-terminal-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"departureTerminal",
				#[cfg(any(
					any(
						feature = "departure-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"departureTime",
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
						feature = "disambiguating-description-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"disambiguatingDescription",
				#[cfg(any(
					any(
						feature = "estimated-flight-duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"estimatedFlightDuration",
				#[cfg(any(
					any(
						feature = "flight-distance-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"flightDistance",
				#[cfg(any(
					any(
						feature = "flight-number-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"flightNumber",
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
						feature = "itinerary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"itinerary",
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
						feature = "meal-service-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mealService",
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
						feature = "part-of-trip-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"partOfTrip",
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
						feature = "provider-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"provider",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(feature = "seller-property-schema", feature = "general-schema-section"),
					doc
				))]
				"seller",
				#[cfg(any(
					any(
						feature = "sub-trip-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"subTrip",
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
						feature = "trip-origin-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"tripOrigin",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
				#[cfg(any(
					any(
						feature = "web-checkin-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"webCheckinTime",
			];
			deserializer.deserialize_struct("Flight", FIELDS, ClassVisitor)
		}
	}
}
