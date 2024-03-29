use super::*;
/// <https://schema.org/Flight>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Flight {
	/// <https://schema.org/aircraft>
	pub r#aircraft: Vec<AircraftProperty>,
	/// <https://schema.org/arrivalAirport>
	pub r#arrival_airport: Vec<ArrivalAirportProperty>,
	/// <https://schema.org/arrivalGate>
	pub r#arrival_gate: Vec<ArrivalGateProperty>,
	/// <https://schema.org/arrivalTerminal>
	pub r#arrival_terminal: Vec<ArrivalTerminalProperty>,
	/// <https://schema.org/boardingPolicy>
	pub r#boarding_policy: Vec<BoardingPolicyProperty>,
	/// <https://schema.org/carrier>
	#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
	pub r#carrier: Vec<CarrierProperty>,
	/// <https://schema.org/departureAirport>
	pub r#departure_airport: Vec<DepartureAirportProperty>,
	/// <https://schema.org/departureGate>
	pub r#departure_gate: Vec<DepartureGateProperty>,
	/// <https://schema.org/departureTerminal>
	pub r#departure_terminal: Vec<DepartureTerminalProperty>,
	/// <https://schema.org/estimatedFlightDuration>
	pub r#estimated_flight_duration: Vec<EstimatedFlightDurationProperty>,
	/// <https://schema.org/flightDistance>
	pub r#flight_distance: Vec<FlightDistanceProperty>,
	/// <https://schema.org/flightNumber>
	pub r#flight_number: Vec<FlightNumberProperty>,
	/// <https://schema.org/mealService>
	pub r#meal_service: Vec<MealServiceProperty>,
	/// <https://schema.org/seller>
	pub r#seller: Vec<SellerProperty>,
	/// <https://schema.org/webCheckinTime>
	pub r#web_checkin_time: Vec<WebCheckinTimeProperty>,
	/// <https://schema.org/additionalType>
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	/// <https://schema.org/alternateName>
	pub r#alternate_name: Vec<AlternateNameProperty>,
	/// <https://schema.org/description>
	pub r#description: Vec<DescriptionProperty>,
	/// <https://schema.org/disambiguatingDescription>
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	/// <https://schema.org/identifier>
	pub r#identifier: Vec<IdentifierProperty>,
	/// <https://schema.org/image>
	pub r#image: Vec<ImageProperty>,
	/// <https://schema.org/mainEntityOfPage>
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	/// <https://schema.org/name>
	pub r#name: Vec<NameProperty>,
	/// <https://schema.org/potentialAction>
	pub r#potential_action: Vec<PotentialActionProperty>,
	/// <https://schema.org/sameAs>
	pub r#same_as: Vec<SameAsProperty>,
	/// <https://schema.org/subjectOf>
	pub r#subject_of: Vec<SubjectOfProperty>,
	/// <https://schema.org/url>
	pub r#url: Vec<UrlProperty>,
	/// <https://schema.org/arrivalTime>
	pub r#arrival_time: Vec<ArrivalTimeProperty>,
	/// <https://schema.org/departureTime>
	pub r#departure_time: Vec<DepartureTimeProperty>,
	/// <https://schema.org/itinerary>
	pub r#itinerary: Vec<ItineraryProperty>,
	/// <https://schema.org/offers>
	pub r#offers: Vec<OffersProperty>,
	/// <https://schema.org/partOfTrip>
	pub r#part_of_trip: Vec<PartOfTripProperty>,
	/// <https://schema.org/provider>
	pub r#provider: Vec<ProviderProperty>,
	/// <https://schema.org/subTrip>
	pub r#sub_trip: Vec<SubTripProperty>,
	/// <https://schema.org/tripOrigin>
	pub r#trip_origin: Vec<TripOriginProperty>,
}
/// This trait is for properties from <https://schema.org/Flight>.
pub trait FlightTrait {
	/// Get <https://schema.org/aircraft> from [`Self`] as borrowed slice.
	fn get_aircraft(&self) -> &[AircraftProperty];
	/// Take <https://schema.org/aircraft> from [`Self`] as owned vector.
	fn take_aircraft(&mut self) -> Vec<AircraftProperty>;
	/// Get <https://schema.org/arrivalAirport> from [`Self`] as borrowed slice.
	fn get_arrival_airport(&self) -> &[ArrivalAirportProperty];
	/// Take <https://schema.org/arrivalAirport> from [`Self`] as owned vector.
	fn take_arrival_airport(&mut self) -> Vec<ArrivalAirportProperty>;
	/// Get <https://schema.org/arrivalGate> from [`Self`] as borrowed slice.
	fn get_arrival_gate(&self) -> &[ArrivalGateProperty];
	/// Take <https://schema.org/arrivalGate> from [`Self`] as owned vector.
	fn take_arrival_gate(&mut self) -> Vec<ArrivalGateProperty>;
	/// Get <https://schema.org/arrivalTerminal> from [`Self`] as borrowed slice.
	fn get_arrival_terminal(&self) -> &[ArrivalTerminalProperty];
	/// Take <https://schema.org/arrivalTerminal> from [`Self`] as owned vector.
	fn take_arrival_terminal(&mut self) -> Vec<ArrivalTerminalProperty>;
	/// Get <https://schema.org/boardingPolicy> from [`Self`] as borrowed slice.
	fn get_boarding_policy(&self) -> &[BoardingPolicyProperty];
	/// Take <https://schema.org/boardingPolicy> from [`Self`] as owned vector.
	fn take_boarding_policy(&mut self) -> Vec<BoardingPolicyProperty>;
	/// Get <https://schema.org/carrier> from [`Self`] as borrowed slice.
	#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
	fn get_carrier(&self) -> &[CarrierProperty];
	/// Take <https://schema.org/carrier> from [`Self`] as owned vector.
	#[deprecated = "This schema is superseded by <https://schema.org/provider>."]
	fn take_carrier(&mut self) -> Vec<CarrierProperty>;
	/// Get <https://schema.org/departureAirport> from [`Self`] as borrowed slice.
	fn get_departure_airport(&self) -> &[DepartureAirportProperty];
	/// Take <https://schema.org/departureAirport> from [`Self`] as owned vector.
	fn take_departure_airport(&mut self) -> Vec<DepartureAirportProperty>;
	/// Get <https://schema.org/departureGate> from [`Self`] as borrowed slice.
	fn get_departure_gate(&self) -> &[DepartureGateProperty];
	/// Take <https://schema.org/departureGate> from [`Self`] as owned vector.
	fn take_departure_gate(&mut self) -> Vec<DepartureGateProperty>;
	/// Get <https://schema.org/departureTerminal> from [`Self`] as borrowed slice.
	fn get_departure_terminal(&self) -> &[DepartureTerminalProperty];
	/// Take <https://schema.org/departureTerminal> from [`Self`] as owned vector.
	fn take_departure_terminal(&mut self) -> Vec<DepartureTerminalProperty>;
	/// Get <https://schema.org/estimatedFlightDuration> from [`Self`] as borrowed slice.
	fn get_estimated_flight_duration(&self) -> &[EstimatedFlightDurationProperty];
	/// Take <https://schema.org/estimatedFlightDuration> from [`Self`] as owned vector.
	fn take_estimated_flight_duration(&mut self) -> Vec<EstimatedFlightDurationProperty>;
	/// Get <https://schema.org/flightDistance> from [`Self`] as borrowed slice.
	fn get_flight_distance(&self) -> &[FlightDistanceProperty];
	/// Take <https://schema.org/flightDistance> from [`Self`] as owned vector.
	fn take_flight_distance(&mut self) -> Vec<FlightDistanceProperty>;
	/// Get <https://schema.org/flightNumber> from [`Self`] as borrowed slice.
	fn get_flight_number(&self) -> &[FlightNumberProperty];
	/// Take <https://schema.org/flightNumber> from [`Self`] as owned vector.
	fn take_flight_number(&mut self) -> Vec<FlightNumberProperty>;
	/// Get <https://schema.org/mealService> from [`Self`] as borrowed slice.
	fn get_meal_service(&self) -> &[MealServiceProperty];
	/// Take <https://schema.org/mealService> from [`Self`] as owned vector.
	fn take_meal_service(&mut self) -> Vec<MealServiceProperty>;
	/// Get <https://schema.org/seller> from [`Self`] as borrowed slice.
	fn get_seller(&self) -> &[SellerProperty];
	/// Take <https://schema.org/seller> from [`Self`] as owned vector.
	fn take_seller(&mut self) -> Vec<SellerProperty>;
	/// Get <https://schema.org/webCheckinTime> from [`Self`] as borrowed slice.
	fn get_web_checkin_time(&self) -> &[WebCheckinTimeProperty];
	/// Take <https://schema.org/webCheckinTime> from [`Self`] as owned vector.
	fn take_web_checkin_time(&mut self) -> Vec<WebCheckinTimeProperty>;
}
impl FlightTrait for Flight {
	fn get_aircraft(&self) -> &[AircraftProperty] {
		self.r#aircraft.as_slice()
	}
	fn take_aircraft(&mut self) -> Vec<AircraftProperty> {
		std::mem::take(&mut self.r#aircraft)
	}
	fn get_arrival_airport(&self) -> &[ArrivalAirportProperty] {
		self.r#arrival_airport.as_slice()
	}
	fn take_arrival_airport(&mut self) -> Vec<ArrivalAirportProperty> {
		std::mem::take(&mut self.r#arrival_airport)
	}
	fn get_arrival_gate(&self) -> &[ArrivalGateProperty] {
		self.r#arrival_gate.as_slice()
	}
	fn take_arrival_gate(&mut self) -> Vec<ArrivalGateProperty> {
		std::mem::take(&mut self.r#arrival_gate)
	}
	fn get_arrival_terminal(&self) -> &[ArrivalTerminalProperty] {
		self.r#arrival_terminal.as_slice()
	}
	fn take_arrival_terminal(&mut self) -> Vec<ArrivalTerminalProperty> {
		std::mem::take(&mut self.r#arrival_terminal)
	}
	fn get_boarding_policy(&self) -> &[BoardingPolicyProperty] {
		self.r#boarding_policy.as_slice()
	}
	fn take_boarding_policy(&mut self) -> Vec<BoardingPolicyProperty> {
		std::mem::take(&mut self.r#boarding_policy)
	}
	fn get_carrier(&self) -> &[CarrierProperty] {
		self.r#carrier.as_slice()
	}
	fn take_carrier(&mut self) -> Vec<CarrierProperty> {
		std::mem::take(&mut self.r#carrier)
	}
	fn get_departure_airport(&self) -> &[DepartureAirportProperty] {
		self.r#departure_airport.as_slice()
	}
	fn take_departure_airport(&mut self) -> Vec<DepartureAirportProperty> {
		std::mem::take(&mut self.r#departure_airport)
	}
	fn get_departure_gate(&self) -> &[DepartureGateProperty] {
		self.r#departure_gate.as_slice()
	}
	fn take_departure_gate(&mut self) -> Vec<DepartureGateProperty> {
		std::mem::take(&mut self.r#departure_gate)
	}
	fn get_departure_terminal(&self) -> &[DepartureTerminalProperty] {
		self.r#departure_terminal.as_slice()
	}
	fn take_departure_terminal(&mut self) -> Vec<DepartureTerminalProperty> {
		std::mem::take(&mut self.r#departure_terminal)
	}
	fn get_estimated_flight_duration(&self) -> &[EstimatedFlightDurationProperty] {
		self.r#estimated_flight_duration.as_slice()
	}
	fn take_estimated_flight_duration(&mut self) -> Vec<EstimatedFlightDurationProperty> {
		std::mem::take(&mut self.r#estimated_flight_duration)
	}
	fn get_flight_distance(&self) -> &[FlightDistanceProperty] {
		self.r#flight_distance.as_slice()
	}
	fn take_flight_distance(&mut self) -> Vec<FlightDistanceProperty> {
		std::mem::take(&mut self.r#flight_distance)
	}
	fn get_flight_number(&self) -> &[FlightNumberProperty] {
		self.r#flight_number.as_slice()
	}
	fn take_flight_number(&mut self) -> Vec<FlightNumberProperty> {
		std::mem::take(&mut self.r#flight_number)
	}
	fn get_meal_service(&self) -> &[MealServiceProperty] {
		self.r#meal_service.as_slice()
	}
	fn take_meal_service(&mut self) -> Vec<MealServiceProperty> {
		std::mem::take(&mut self.r#meal_service)
	}
	fn get_seller(&self) -> &[SellerProperty] {
		self.r#seller.as_slice()
	}
	fn take_seller(&mut self) -> Vec<SellerProperty> {
		std::mem::take(&mut self.r#seller)
	}
	fn get_web_checkin_time(&self) -> &[WebCheckinTimeProperty] {
		self.r#web_checkin_time.as_slice()
	}
	fn take_web_checkin_time(&mut self) -> Vec<WebCheckinTimeProperty> {
		std::mem::take(&mut self.r#web_checkin_time)
	}
}
impl ThingTrait for Flight {
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
impl TripTrait for Flight {
	fn get_arrival_time(&self) -> &[ArrivalTimeProperty] {
		self.r#arrival_time.as_slice()
	}
	fn take_arrival_time(&mut self) -> Vec<ArrivalTimeProperty> {
		std::mem::take(&mut self.r#arrival_time)
	}
	fn get_departure_time(&self) -> &[DepartureTimeProperty] {
		self.r#departure_time.as_slice()
	}
	fn take_departure_time(&mut self) -> Vec<DepartureTimeProperty> {
		std::mem::take(&mut self.r#departure_time)
	}
	fn get_itinerary(&self) -> &[ItineraryProperty] {
		self.r#itinerary.as_slice()
	}
	fn take_itinerary(&mut self) -> Vec<ItineraryProperty> {
		std::mem::take(&mut self.r#itinerary)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_part_of_trip(&self) -> &[PartOfTripProperty] {
		self.r#part_of_trip.as_slice()
	}
	fn take_part_of_trip(&mut self) -> Vec<PartOfTripProperty> {
		std::mem::take(&mut self.r#part_of_trip)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_sub_trip(&self) -> &[SubTripProperty] {
		self.r#sub_trip.as_slice()
	}
	fn take_sub_trip(&mut self) -> Vec<SubTripProperty> {
		std::mem::take(&mut self.r#sub_trip)
	}
	fn get_trip_origin(&self) -> &[TripOriginProperty] {
		self.r#trip_origin.as_slice()
	}
	fn take_trip_origin(&mut self) -> Vec<TripOriginProperty> {
		std::mem::take(&mut self.r#trip_origin)
	}
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
				!Vec::is_empty(&self.r#aircraft) as usize,
				!Vec::is_empty(&self.r#arrival_airport) as usize,
				!Vec::is_empty(&self.r#arrival_gate) as usize,
				!Vec::is_empty(&self.r#arrival_terminal) as usize,
				!Vec::is_empty(&self.r#boarding_policy) as usize,
				!Vec::is_empty(&self.r#carrier) as usize,
				!Vec::is_empty(&self.r#departure_airport) as usize,
				!Vec::is_empty(&self.r#departure_gate) as usize,
				!Vec::is_empty(&self.r#departure_terminal) as usize,
				!Vec::is_empty(&self.r#estimated_flight_duration) as usize,
				!Vec::is_empty(&self.r#flight_distance) as usize,
				!Vec::is_empty(&self.r#flight_number) as usize,
				!Vec::is_empty(&self.r#meal_service) as usize,
				!Vec::is_empty(&self.r#seller) as usize,
				!Vec::is_empty(&self.r#web_checkin_time) as usize,
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
				!Vec::is_empty(&self.r#arrival_time) as usize,
				!Vec::is_empty(&self.r#departure_time) as usize,
				!Vec::is_empty(&self.r#itinerary) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#part_of_trip) as usize,
				!Vec::is_empty(&self.r#provider) as usize,
				!Vec::is_empty(&self.r#sub_trip) as usize,
				!Vec::is_empty(&self.r#trip_origin) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct = Serializer::serialize_struct(serializer, "Flight", len)?;
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
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for Flight {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				Aircraft,
				ArrivalAirport,
				ArrivalGate,
				ArrivalTerminal,
				BoardingPolicy,
				Carrier,
				DepartureAirport,
				DepartureGate,
				DepartureTerminal,
				EstimatedFlightDuration,
				FlightDistance,
				FlightNumber,
				MealService,
				Seller,
				WebCheckinTime,
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
				ArrivalTime,
				DepartureTime,
				Itinerary,
				Offers,
				PartOfTrip,
				Provider,
				SubTrip,
				TripOrigin,
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
						"aircraft" => Ok(Field::Aircraft),
						"arrivalAirport" => Ok(Field::ArrivalAirport),
						"arrivalGate" => Ok(Field::ArrivalGate),
						"arrivalTerminal" => Ok(Field::ArrivalTerminal),
						"boardingPolicy" => Ok(Field::BoardingPolicy),
						"carrier" => Ok(Field::Carrier),
						"departureAirport" => Ok(Field::DepartureAirport),
						"departureGate" => Ok(Field::DepartureGate),
						"departureTerminal" => Ok(Field::DepartureTerminal),
						"estimatedFlightDuration" => Ok(Field::EstimatedFlightDuration),
						"flightDistance" => Ok(Field::FlightDistance),
						"flightNumber" => Ok(Field::FlightNumber),
						"mealService" => Ok(Field::MealService),
						"seller" => Ok(Field::Seller),
						"webCheckinTime" => Ok(Field::WebCheckinTime),
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
						"arrivalTime" => Ok(Field::ArrivalTime),
						"departureTime" => Ok(Field::DepartureTime),
						"itinerary" => Ok(Field::Itinerary),
						"offers" => Ok(Field::Offers),
						"partOfTrip" => Ok(Field::PartOfTrip),
						"provider" => Ok(Field::Provider),
						"subTrip" => Ok(Field::SubTrip),
						"tripOrigin" => Ok(Field::TripOrigin),
						"id" | "type" => Ok(Field::Ignore),
						_ => Err(de::Error::unknown_field(value, FIELDS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"aircraft" => Ok(Field::Aircraft),
						b"arrivalAirport" => Ok(Field::ArrivalAirport),
						b"arrivalGate" => Ok(Field::ArrivalGate),
						b"arrivalTerminal" => Ok(Field::ArrivalTerminal),
						b"boardingPolicy" => Ok(Field::BoardingPolicy),
						b"carrier" => Ok(Field::Carrier),
						b"departureAirport" => Ok(Field::DepartureAirport),
						b"departureGate" => Ok(Field::DepartureGate),
						b"departureTerminal" => Ok(Field::DepartureTerminal),
						b"estimatedFlightDuration" => Ok(Field::EstimatedFlightDuration),
						b"flightDistance" => Ok(Field::FlightDistance),
						b"flightNumber" => Ok(Field::FlightNumber),
						b"mealService" => Ok(Field::MealService),
						b"seller" => Ok(Field::Seller),
						b"webCheckinTime" => Ok(Field::WebCheckinTime),
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
						b"arrivalTime" => Ok(Field::ArrivalTime),
						b"departureTime" => Ok(Field::DepartureTime),
						b"itinerary" => Ok(Field::Itinerary),
						b"offers" => Ok(Field::Offers),
						b"partOfTrip" => Ok(Field::PartOfTrip),
						b"provider" => Ok(Field::Provider),
						b"subTrip" => Ok(Field::SubTrip),
						b"tripOrigin" => Ok(Field::TripOrigin),
						b"id" | b"type" => Ok(Field::Ignore),
						_ => {
							let value = &String::from_utf8_lossy(value);
							Err(de::Error::unknown_field(value, FIELDS))
						}
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
					let mut r#aircraft_property = None;
					let mut r#arrival_airport_property = None;
					let mut r#arrival_gate_property = None;
					let mut r#arrival_terminal_property = None;
					let mut r#boarding_policy_property = None;
					let mut r#carrier_property = None;
					let mut r#departure_airport_property = None;
					let mut r#departure_gate_property = None;
					let mut r#departure_terminal_property = None;
					let mut r#estimated_flight_duration_property = None;
					let mut r#flight_distance_property = None;
					let mut r#flight_number_property = None;
					let mut r#meal_service_property = None;
					let mut r#seller_property = None;
					let mut r#web_checkin_time_property = None;
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
					let mut r#arrival_time_property = None;
					let mut r#departure_time_property = None;
					let mut r#itinerary_property = None;
					let mut r#offers_property = None;
					let mut r#part_of_trip_property = None;
					let mut r#provider_property = None;
					let mut r#sub_trip_property = None;
					let mut r#trip_origin_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
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
							Field::Ignore => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(Flight {
						r#aircraft: r#aircraft_property.unwrap_or_default(),
						r#arrival_airport: r#arrival_airport_property.unwrap_or_default(),
						r#arrival_gate: r#arrival_gate_property.unwrap_or_default(),
						r#arrival_terminal: r#arrival_terminal_property.unwrap_or_default(),
						r#boarding_policy: r#boarding_policy_property.unwrap_or_default(),
						r#carrier: r#carrier_property.unwrap_or_default(),
						r#departure_airport: r#departure_airport_property.unwrap_or_default(),
						r#departure_gate: r#departure_gate_property.unwrap_or_default(),
						r#departure_terminal: r#departure_terminal_property.unwrap_or_default(),
						r#estimated_flight_duration: r#estimated_flight_duration_property
							.unwrap_or_default(),
						r#flight_distance: r#flight_distance_property.unwrap_or_default(),
						r#flight_number: r#flight_number_property.unwrap_or_default(),
						r#meal_service: r#meal_service_property.unwrap_or_default(),
						r#seller: r#seller_property.unwrap_or_default(),
						r#web_checkin_time: r#web_checkin_time_property.unwrap_or_default(),
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
						r#arrival_time: r#arrival_time_property.unwrap_or_default(),
						r#departure_time: r#departure_time_property.unwrap_or_default(),
						r#itinerary: r#itinerary_property.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#part_of_trip: r#part_of_trip_property.unwrap_or_default(),
						r#provider: r#provider_property.unwrap_or_default(),
						r#sub_trip: r#sub_trip_property.unwrap_or_default(),
						r#trip_origin: r#trip_origin_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"aircraft",
				"arrivalAirport",
				"arrivalGate",
				"arrivalTerminal",
				"boardingPolicy",
				"carrier",
				"departureAirport",
				"departureGate",
				"departureTerminal",
				"estimatedFlightDuration",
				"flightDistance",
				"flightNumber",
				"mealService",
				"seller",
				"webCheckinTime",
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
				"arrivalTime",
				"departureTime",
				"itinerary",
				"offers",
				"partOfTrip",
				"provider",
				"subTrip",
				"tripOrigin",
			];
			deserializer.deserialize_struct("Flight", FIELDS, ClassVisitor)
		}
	}
}
