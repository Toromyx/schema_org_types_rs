use super::*;
/// <https://schema.org/DefenceEstablishment>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct DefenceEstablishment {
	/// <https://schema.org/openingHours>
	pub r#opening_hours: Vec<OpeningHoursProperty>,
	/// <https://schema.org/additionalProperty>
	pub r#additional_property: Vec<AdditionalPropertyProperty>,
	/// <https://schema.org/address>
	pub r#address: Vec<AddressProperty>,
	/// <https://schema.org/aggregateRating>
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	/// <https://schema.org/amenityFeature>
	pub r#amenity_feature: Vec<AmenityFeatureProperty>,
	/// <https://schema.org/branchCode>
	pub r#branch_code: Vec<BranchCodeProperty>,
	/// <https://schema.org/containedIn>
	#[deprecated = "This schema is superseded by <https://schema.org/containedInPlace>."]
	pub r#contained_in: Vec<ContainedInProperty>,
	/// <https://schema.org/containedInPlace>
	pub r#contained_in_place: Vec<ContainedInPlaceProperty>,
	/// <https://schema.org/containsPlace>
	pub r#contains_place: Vec<ContainsPlaceProperty>,
	/// <https://schema.org/event>
	pub r#event: Vec<EventProperty>,
	/// <https://schema.org/events>
	#[deprecated = "This schema is superseded by <https://schema.org/event>."]
	pub r#events: Vec<EventsProperty>,
	/// <https://schema.org/faxNumber>
	pub r#fax_number: Vec<FaxNumberProperty>,
	/// <https://schema.org/geo>
	pub r#geo: Vec<GeoProperty>,
	/// <https://schema.org/geoContains>
	pub r#geo_contains: Vec<GeoContainsProperty>,
	/// <https://schema.org/geoCoveredBy>
	pub r#geo_covered_by: Vec<GeoCoveredByProperty>,
	/// <https://schema.org/geoCovers>
	pub r#geo_covers: Vec<GeoCoversProperty>,
	/// <https://schema.org/geoCrosses>
	pub r#geo_crosses: Vec<GeoCrossesProperty>,
	/// <https://schema.org/geoDisjoint>
	pub r#geo_disjoint: Vec<GeoDisjointProperty>,
	/// <https://schema.org/geoEquals>
	pub r#geo_equals: Vec<GeoEqualsProperty>,
	/// <https://schema.org/geoIntersects>
	pub r#geo_intersects: Vec<GeoIntersectsProperty>,
	/// <https://schema.org/geoOverlaps>
	pub r#geo_overlaps: Vec<GeoOverlapsProperty>,
	/// <https://schema.org/geoTouches>
	pub r#geo_touches: Vec<GeoTouchesProperty>,
	/// <https://schema.org/geoWithin>
	pub r#geo_within: Vec<GeoWithinProperty>,
	/// <https://schema.org/globalLocationNumber>
	pub r#global_location_number: Vec<GlobalLocationNumberProperty>,
	/// <https://schema.org/hasDriveThroughService>
	pub r#has_drive_through_service: Vec<HasDriveThroughServiceProperty>,
	/// <https://schema.org/hasMap>
	pub r#has_map: Vec<HasMapProperty>,
	/// <https://schema.org/isAccessibleForFree>
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	/// <https://schema.org/isicV4>
	pub r#isic_v_4: Vec<IsicV4Property>,
	/// <https://schema.org/keywords>
	pub r#keywords: Vec<KeywordsProperty>,
	/// <https://schema.org/latitude>
	pub r#latitude: Vec<LatitudeProperty>,
	/// <https://schema.org/logo>
	pub r#logo: Vec<LogoProperty>,
	/// <https://schema.org/longitude>
	pub r#longitude: Vec<LongitudeProperty>,
	/// <https://schema.org/map>
	#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
	pub r#map: Vec<MapProperty>,
	/// <https://schema.org/maps>
	#[deprecated = "This schema is superseded by <https://schema.org/hasMap>."]
	pub r#maps: Vec<MapsProperty>,
	/// <https://schema.org/maximumAttendeeCapacity>
	pub r#maximum_attendee_capacity: Vec<MaximumAttendeeCapacityProperty>,
	/// <https://schema.org/openingHoursSpecification>
	pub r#opening_hours_specification: Vec<OpeningHoursSpecificationProperty>,
	/// <https://schema.org/photo>
	pub r#photo: Vec<PhotoProperty>,
	/// <https://schema.org/photos>
	#[deprecated = "This schema is superseded by <https://schema.org/photo>."]
	pub r#photos: Vec<PhotosProperty>,
	/// <https://schema.org/publicAccess>
	pub r#public_access: Vec<PublicAccessProperty>,
	/// <https://schema.org/review>
	pub r#review: Vec<ReviewProperty>,
	/// <https://schema.org/reviews>
	#[deprecated = "This schema is superseded by <https://schema.org/review>."]
	pub r#reviews: Vec<ReviewsProperty>,
	/// <https://schema.org/slogan>
	pub r#slogan: Vec<SloganProperty>,
	/// <https://schema.org/smokingAllowed>
	pub r#smoking_allowed: Vec<SmokingAllowedProperty>,
	/// <https://schema.org/specialOpeningHoursSpecification>
	pub r#special_opening_hours_specification: Vec<SpecialOpeningHoursSpecificationProperty>,
	/// <https://schema.org/telephone>
	pub r#telephone: Vec<TelephoneProperty>,
	/// <https://schema.org/tourBookingPage>
	pub r#tour_booking_page: Vec<TourBookingPageProperty>,
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
}
/// This trait is for properties from <https://schema.org/DefenceEstablishment>.
pub trait DefenceEstablishmentTrait {}
impl DefenceEstablishmentTrait for DefenceEstablishment {}
impl CivicStructureTrait for DefenceEstablishment {
	fn get_opening_hours(&self) -> &[OpeningHoursProperty] {
		self.r#opening_hours.as_slice()
	}
	fn take_opening_hours(&mut self) -> Vec<OpeningHoursProperty> {
		std::mem::take(&mut self.r#opening_hours)
	}
}
impl GovernmentBuildingTrait for DefenceEstablishment {}
impl PlaceTrait for DefenceEstablishment {
	fn get_additional_property(&self) -> &[AdditionalPropertyProperty] {
		self.r#additional_property.as_slice()
	}
	fn take_additional_property(&mut self) -> Vec<AdditionalPropertyProperty> {
		std::mem::take(&mut self.r#additional_property)
	}
	fn get_address(&self) -> &[AddressProperty] {
		self.r#address.as_slice()
	}
	fn take_address(&mut self) -> Vec<AddressProperty> {
		std::mem::take(&mut self.r#address)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_amenity_feature(&self) -> &[AmenityFeatureProperty] {
		self.r#amenity_feature.as_slice()
	}
	fn take_amenity_feature(&mut self) -> Vec<AmenityFeatureProperty> {
		std::mem::take(&mut self.r#amenity_feature)
	}
	fn get_branch_code(&self) -> &[BranchCodeProperty] {
		self.r#branch_code.as_slice()
	}
	fn take_branch_code(&mut self) -> Vec<BranchCodeProperty> {
		std::mem::take(&mut self.r#branch_code)
	}
	fn get_contained_in(&self) -> &[ContainedInProperty] {
		self.r#contained_in.as_slice()
	}
	fn take_contained_in(&mut self) -> Vec<ContainedInProperty> {
		std::mem::take(&mut self.r#contained_in)
	}
	fn get_contained_in_place(&self) -> &[ContainedInPlaceProperty] {
		self.r#contained_in_place.as_slice()
	}
	fn take_contained_in_place(&mut self) -> Vec<ContainedInPlaceProperty> {
		std::mem::take(&mut self.r#contained_in_place)
	}
	fn get_contains_place(&self) -> &[ContainsPlaceProperty] {
		self.r#contains_place.as_slice()
	}
	fn take_contains_place(&mut self) -> Vec<ContainsPlaceProperty> {
		std::mem::take(&mut self.r#contains_place)
	}
	fn get_event(&self) -> &[EventProperty] {
		self.r#event.as_slice()
	}
	fn take_event(&mut self) -> Vec<EventProperty> {
		std::mem::take(&mut self.r#event)
	}
	fn get_events(&self) -> &[EventsProperty] {
		self.r#events.as_slice()
	}
	fn take_events(&mut self) -> Vec<EventsProperty> {
		std::mem::take(&mut self.r#events)
	}
	fn get_fax_number(&self) -> &[FaxNumberProperty] {
		self.r#fax_number.as_slice()
	}
	fn take_fax_number(&mut self) -> Vec<FaxNumberProperty> {
		std::mem::take(&mut self.r#fax_number)
	}
	fn get_geo(&self) -> &[GeoProperty] {
		self.r#geo.as_slice()
	}
	fn take_geo(&mut self) -> Vec<GeoProperty> {
		std::mem::take(&mut self.r#geo)
	}
	fn get_geo_contains(&self) -> &[GeoContainsProperty] {
		self.r#geo_contains.as_slice()
	}
	fn take_geo_contains(&mut self) -> Vec<GeoContainsProperty> {
		std::mem::take(&mut self.r#geo_contains)
	}
	fn get_geo_covered_by(&self) -> &[GeoCoveredByProperty] {
		self.r#geo_covered_by.as_slice()
	}
	fn take_geo_covered_by(&mut self) -> Vec<GeoCoveredByProperty> {
		std::mem::take(&mut self.r#geo_covered_by)
	}
	fn get_geo_covers(&self) -> &[GeoCoversProperty] {
		self.r#geo_covers.as_slice()
	}
	fn take_geo_covers(&mut self) -> Vec<GeoCoversProperty> {
		std::mem::take(&mut self.r#geo_covers)
	}
	fn get_geo_crosses(&self) -> &[GeoCrossesProperty] {
		self.r#geo_crosses.as_slice()
	}
	fn take_geo_crosses(&mut self) -> Vec<GeoCrossesProperty> {
		std::mem::take(&mut self.r#geo_crosses)
	}
	fn get_geo_disjoint(&self) -> &[GeoDisjointProperty] {
		self.r#geo_disjoint.as_slice()
	}
	fn take_geo_disjoint(&mut self) -> Vec<GeoDisjointProperty> {
		std::mem::take(&mut self.r#geo_disjoint)
	}
	fn get_geo_equals(&self) -> &[GeoEqualsProperty] {
		self.r#geo_equals.as_slice()
	}
	fn take_geo_equals(&mut self) -> Vec<GeoEqualsProperty> {
		std::mem::take(&mut self.r#geo_equals)
	}
	fn get_geo_intersects(&self) -> &[GeoIntersectsProperty] {
		self.r#geo_intersects.as_slice()
	}
	fn take_geo_intersects(&mut self) -> Vec<GeoIntersectsProperty> {
		std::mem::take(&mut self.r#geo_intersects)
	}
	fn get_geo_overlaps(&self) -> &[GeoOverlapsProperty] {
		self.r#geo_overlaps.as_slice()
	}
	fn take_geo_overlaps(&mut self) -> Vec<GeoOverlapsProperty> {
		std::mem::take(&mut self.r#geo_overlaps)
	}
	fn get_geo_touches(&self) -> &[GeoTouchesProperty] {
		self.r#geo_touches.as_slice()
	}
	fn take_geo_touches(&mut self) -> Vec<GeoTouchesProperty> {
		std::mem::take(&mut self.r#geo_touches)
	}
	fn get_geo_within(&self) -> &[GeoWithinProperty] {
		self.r#geo_within.as_slice()
	}
	fn take_geo_within(&mut self) -> Vec<GeoWithinProperty> {
		std::mem::take(&mut self.r#geo_within)
	}
	fn get_global_location_number(&self) -> &[GlobalLocationNumberProperty] {
		self.r#global_location_number.as_slice()
	}
	fn take_global_location_number(&mut self) -> Vec<GlobalLocationNumberProperty> {
		std::mem::take(&mut self.r#global_location_number)
	}
	fn get_has_drive_through_service(&self) -> &[HasDriveThroughServiceProperty] {
		self.r#has_drive_through_service.as_slice()
	}
	fn take_has_drive_through_service(&mut self) -> Vec<HasDriveThroughServiceProperty> {
		std::mem::take(&mut self.r#has_drive_through_service)
	}
	fn get_has_map(&self) -> &[HasMapProperty] {
		self.r#has_map.as_slice()
	}
	fn take_has_map(&mut self) -> Vec<HasMapProperty> {
		std::mem::take(&mut self.r#has_map)
	}
	fn get_is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty] {
		self.r#is_accessible_for_free.as_slice()
	}
	fn take_is_accessible_for_free(&mut self) -> Vec<IsAccessibleForFreeProperty> {
		std::mem::take(&mut self.r#is_accessible_for_free)
	}
	fn get_isic_v_4(&self) -> &[IsicV4Property] {
		self.r#isic_v_4.as_slice()
	}
	fn take_isic_v_4(&mut self) -> Vec<IsicV4Property> {
		std::mem::take(&mut self.r#isic_v_4)
	}
	fn get_keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn take_keywords(&mut self) -> Vec<KeywordsProperty> {
		std::mem::take(&mut self.r#keywords)
	}
	fn get_latitude(&self) -> &[LatitudeProperty] {
		self.r#latitude.as_slice()
	}
	fn take_latitude(&mut self) -> Vec<LatitudeProperty> {
		std::mem::take(&mut self.r#latitude)
	}
	fn get_logo(&self) -> &[LogoProperty] {
		self.r#logo.as_slice()
	}
	fn take_logo(&mut self) -> Vec<LogoProperty> {
		std::mem::take(&mut self.r#logo)
	}
	fn get_longitude(&self) -> &[LongitudeProperty] {
		self.r#longitude.as_slice()
	}
	fn take_longitude(&mut self) -> Vec<LongitudeProperty> {
		std::mem::take(&mut self.r#longitude)
	}
	fn get_map(&self) -> &[MapProperty] {
		self.r#map.as_slice()
	}
	fn take_map(&mut self) -> Vec<MapProperty> {
		std::mem::take(&mut self.r#map)
	}
	fn get_maps(&self) -> &[MapsProperty] {
		self.r#maps.as_slice()
	}
	fn take_maps(&mut self) -> Vec<MapsProperty> {
		std::mem::take(&mut self.r#maps)
	}
	fn get_maximum_attendee_capacity(&self) -> &[MaximumAttendeeCapacityProperty] {
		self.r#maximum_attendee_capacity.as_slice()
	}
	fn take_maximum_attendee_capacity(&mut self) -> Vec<MaximumAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#maximum_attendee_capacity)
	}
	fn get_opening_hours_specification(&self) -> &[OpeningHoursSpecificationProperty] {
		self.r#opening_hours_specification.as_slice()
	}
	fn take_opening_hours_specification(&mut self) -> Vec<OpeningHoursSpecificationProperty> {
		std::mem::take(&mut self.r#opening_hours_specification)
	}
	fn get_photo(&self) -> &[PhotoProperty] {
		self.r#photo.as_slice()
	}
	fn take_photo(&mut self) -> Vec<PhotoProperty> {
		std::mem::take(&mut self.r#photo)
	}
	fn get_photos(&self) -> &[PhotosProperty] {
		self.r#photos.as_slice()
	}
	fn take_photos(&mut self) -> Vec<PhotosProperty> {
		std::mem::take(&mut self.r#photos)
	}
	fn get_public_access(&self) -> &[PublicAccessProperty] {
		self.r#public_access.as_slice()
	}
	fn take_public_access(&mut self) -> Vec<PublicAccessProperty> {
		std::mem::take(&mut self.r#public_access)
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
	fn get_slogan(&self) -> &[SloganProperty] {
		self.r#slogan.as_slice()
	}
	fn take_slogan(&mut self) -> Vec<SloganProperty> {
		std::mem::take(&mut self.r#slogan)
	}
	fn get_smoking_allowed(&self) -> &[SmokingAllowedProperty] {
		self.r#smoking_allowed.as_slice()
	}
	fn take_smoking_allowed(&mut self) -> Vec<SmokingAllowedProperty> {
		std::mem::take(&mut self.r#smoking_allowed)
	}
	fn get_special_opening_hours_specification(
		&self,
	) -> &[SpecialOpeningHoursSpecificationProperty] {
		self.r#special_opening_hours_specification.as_slice()
	}
	fn take_special_opening_hours_specification(
		&mut self,
	) -> Vec<SpecialOpeningHoursSpecificationProperty> {
		std::mem::take(&mut self.r#special_opening_hours_specification)
	}
	fn get_telephone(&self) -> &[TelephoneProperty] {
		self.r#telephone.as_slice()
	}
	fn take_telephone(&mut self) -> Vec<TelephoneProperty> {
		std::mem::take(&mut self.r#telephone)
	}
	fn get_tour_booking_page(&self) -> &[TourBookingPageProperty] {
		self.r#tour_booking_page.as_slice()
	}
	fn take_tour_booking_page(&mut self) -> Vec<TourBookingPageProperty> {
		std::mem::take(&mut self.r#tour_booking_page)
	}
}
impl ThingTrait for DefenceEstablishment {
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
	impl Serialize for DefenceEstablishment {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#opening_hours) as usize,
				!Vec::is_empty(&self.r#additional_property) as usize,
				!Vec::is_empty(&self.r#address) as usize,
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
				!Vec::is_empty(&self.r#amenity_feature) as usize,
				!Vec::is_empty(&self.r#branch_code) as usize,
				!Vec::is_empty(&self.r#contained_in) as usize,
				!Vec::is_empty(&self.r#contained_in_place) as usize,
				!Vec::is_empty(&self.r#contains_place) as usize,
				!Vec::is_empty(&self.r#event) as usize,
				!Vec::is_empty(&self.r#events) as usize,
				!Vec::is_empty(&self.r#fax_number) as usize,
				!Vec::is_empty(&self.r#geo) as usize,
				!Vec::is_empty(&self.r#geo_contains) as usize,
				!Vec::is_empty(&self.r#geo_covered_by) as usize,
				!Vec::is_empty(&self.r#geo_covers) as usize,
				!Vec::is_empty(&self.r#geo_crosses) as usize,
				!Vec::is_empty(&self.r#geo_disjoint) as usize,
				!Vec::is_empty(&self.r#geo_equals) as usize,
				!Vec::is_empty(&self.r#geo_intersects) as usize,
				!Vec::is_empty(&self.r#geo_overlaps) as usize,
				!Vec::is_empty(&self.r#geo_touches) as usize,
				!Vec::is_empty(&self.r#geo_within) as usize,
				!Vec::is_empty(&self.r#global_location_number) as usize,
				!Vec::is_empty(&self.r#has_drive_through_service) as usize,
				!Vec::is_empty(&self.r#has_map) as usize,
				!Vec::is_empty(&self.r#is_accessible_for_free) as usize,
				!Vec::is_empty(&self.r#isic_v_4) as usize,
				!Vec::is_empty(&self.r#keywords) as usize,
				!Vec::is_empty(&self.r#latitude) as usize,
				!Vec::is_empty(&self.r#logo) as usize,
				!Vec::is_empty(&self.r#longitude) as usize,
				!Vec::is_empty(&self.r#map) as usize,
				!Vec::is_empty(&self.r#maps) as usize,
				!Vec::is_empty(&self.r#maximum_attendee_capacity) as usize,
				!Vec::is_empty(&self.r#opening_hours_specification) as usize,
				!Vec::is_empty(&self.r#photo) as usize,
				!Vec::is_empty(&self.r#photos) as usize,
				!Vec::is_empty(&self.r#public_access) as usize,
				!Vec::is_empty(&self.r#review) as usize,
				!Vec::is_empty(&self.r#reviews) as usize,
				!Vec::is_empty(&self.r#slogan) as usize,
				!Vec::is_empty(&self.r#smoking_allowed) as usize,
				!Vec::is_empty(&self.r#special_opening_hours_specification) as usize,
				!Vec::is_empty(&self.r#telephone) as usize,
				!Vec::is_empty(&self.r#tour_booking_page) as usize,
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
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "DefenceEstablishment", len)?;
			if !Vec::is_empty(&self.r#opening_hours) {
				serialize_struct.serialize_field("openingHours", {
					struct SerializeWith<'a>(&'a Vec<OpeningHoursProperty>);
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
					&SerializeWith(&self.r#opening_hours)
				})?;
			} else {
				serialize_struct.skip_field("openingHours")?;
			}
			if !Vec::is_empty(&self.r#additional_property) {
				serialize_struct.serialize_field("additionalProperty", {
					struct SerializeWith<'a>(&'a Vec<AdditionalPropertyProperty>);
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
					&SerializeWith(&self.r#additional_property)
				})?;
			} else {
				serialize_struct.skip_field("additionalProperty")?;
			}
			if !Vec::is_empty(&self.r#address) {
				serialize_struct.serialize_field("address", {
					struct SerializeWith<'a>(&'a Vec<AddressProperty>);
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
					&SerializeWith(&self.r#address)
				})?;
			} else {
				serialize_struct.skip_field("address")?;
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
			if !Vec::is_empty(&self.r#amenity_feature) {
				serialize_struct.serialize_field("amenityFeature", {
					struct SerializeWith<'a>(&'a Vec<AmenityFeatureProperty>);
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
					&SerializeWith(&self.r#amenity_feature)
				})?;
			} else {
				serialize_struct.skip_field("amenityFeature")?;
			}
			if !Vec::is_empty(&self.r#branch_code) {
				serialize_struct.serialize_field("branchCode", {
					struct SerializeWith<'a>(&'a Vec<BranchCodeProperty>);
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
					&SerializeWith(&self.r#branch_code)
				})?;
			} else {
				serialize_struct.skip_field("branchCode")?;
			}
			if !Vec::is_empty(&self.r#contained_in) {
				serialize_struct.serialize_field("containedIn", {
					struct SerializeWith<'a>(&'a Vec<ContainedInProperty>);
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
					&SerializeWith(&self.r#contained_in)
				})?;
			} else {
				serialize_struct.skip_field("containedIn")?;
			}
			if !Vec::is_empty(&self.r#contained_in_place) {
				serialize_struct.serialize_field("containedInPlace", {
					struct SerializeWith<'a>(&'a Vec<ContainedInPlaceProperty>);
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
					&SerializeWith(&self.r#contained_in_place)
				})?;
			} else {
				serialize_struct.skip_field("containedInPlace")?;
			}
			if !Vec::is_empty(&self.r#contains_place) {
				serialize_struct.serialize_field("containsPlace", {
					struct SerializeWith<'a>(&'a Vec<ContainsPlaceProperty>);
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
					&SerializeWith(&self.r#contains_place)
				})?;
			} else {
				serialize_struct.skip_field("containsPlace")?;
			}
			if !Vec::is_empty(&self.r#event) {
				serialize_struct.serialize_field("event", {
					struct SerializeWith<'a>(&'a Vec<EventProperty>);
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
					&SerializeWith(&self.r#event)
				})?;
			} else {
				serialize_struct.skip_field("event")?;
			}
			if !Vec::is_empty(&self.r#events) {
				serialize_struct.serialize_field("events", {
					struct SerializeWith<'a>(&'a Vec<EventsProperty>);
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
					&SerializeWith(&self.r#events)
				})?;
			} else {
				serialize_struct.skip_field("events")?;
			}
			if !Vec::is_empty(&self.r#fax_number) {
				serialize_struct.serialize_field("faxNumber", {
					struct SerializeWith<'a>(&'a Vec<FaxNumberProperty>);
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
					&SerializeWith(&self.r#fax_number)
				})?;
			} else {
				serialize_struct.skip_field("faxNumber")?;
			}
			if !Vec::is_empty(&self.r#geo) {
				serialize_struct.serialize_field("geo", {
					struct SerializeWith<'a>(&'a Vec<GeoProperty>);
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
					&SerializeWith(&self.r#geo)
				})?;
			} else {
				serialize_struct.skip_field("geo")?;
			}
			if !Vec::is_empty(&self.r#geo_contains) {
				serialize_struct.serialize_field("geoContains", {
					struct SerializeWith<'a>(&'a Vec<GeoContainsProperty>);
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
					&SerializeWith(&self.r#geo_contains)
				})?;
			} else {
				serialize_struct.skip_field("geoContains")?;
			}
			if !Vec::is_empty(&self.r#geo_covered_by) {
				serialize_struct.serialize_field("geoCoveredBy", {
					struct SerializeWith<'a>(&'a Vec<GeoCoveredByProperty>);
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
					&SerializeWith(&self.r#geo_covered_by)
				})?;
			} else {
				serialize_struct.skip_field("geoCoveredBy")?;
			}
			if !Vec::is_empty(&self.r#geo_covers) {
				serialize_struct.serialize_field("geoCovers", {
					struct SerializeWith<'a>(&'a Vec<GeoCoversProperty>);
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
					&SerializeWith(&self.r#geo_covers)
				})?;
			} else {
				serialize_struct.skip_field("geoCovers")?;
			}
			if !Vec::is_empty(&self.r#geo_crosses) {
				serialize_struct.serialize_field("geoCrosses", {
					struct SerializeWith<'a>(&'a Vec<GeoCrossesProperty>);
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
					&SerializeWith(&self.r#geo_crosses)
				})?;
			} else {
				serialize_struct.skip_field("geoCrosses")?;
			}
			if !Vec::is_empty(&self.r#geo_disjoint) {
				serialize_struct.serialize_field("geoDisjoint", {
					struct SerializeWith<'a>(&'a Vec<GeoDisjointProperty>);
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
					&SerializeWith(&self.r#geo_disjoint)
				})?;
			} else {
				serialize_struct.skip_field("geoDisjoint")?;
			}
			if !Vec::is_empty(&self.r#geo_equals) {
				serialize_struct.serialize_field("geoEquals", {
					struct SerializeWith<'a>(&'a Vec<GeoEqualsProperty>);
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
					&SerializeWith(&self.r#geo_equals)
				})?;
			} else {
				serialize_struct.skip_field("geoEquals")?;
			}
			if !Vec::is_empty(&self.r#geo_intersects) {
				serialize_struct.serialize_field("geoIntersects", {
					struct SerializeWith<'a>(&'a Vec<GeoIntersectsProperty>);
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
					&SerializeWith(&self.r#geo_intersects)
				})?;
			} else {
				serialize_struct.skip_field("geoIntersects")?;
			}
			if !Vec::is_empty(&self.r#geo_overlaps) {
				serialize_struct.serialize_field("geoOverlaps", {
					struct SerializeWith<'a>(&'a Vec<GeoOverlapsProperty>);
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
					&SerializeWith(&self.r#geo_overlaps)
				})?;
			} else {
				serialize_struct.skip_field("geoOverlaps")?;
			}
			if !Vec::is_empty(&self.r#geo_touches) {
				serialize_struct.serialize_field("geoTouches", {
					struct SerializeWith<'a>(&'a Vec<GeoTouchesProperty>);
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
					&SerializeWith(&self.r#geo_touches)
				})?;
			} else {
				serialize_struct.skip_field("geoTouches")?;
			}
			if !Vec::is_empty(&self.r#geo_within) {
				serialize_struct.serialize_field("geoWithin", {
					struct SerializeWith<'a>(&'a Vec<GeoWithinProperty>);
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
					&SerializeWith(&self.r#geo_within)
				})?;
			} else {
				serialize_struct.skip_field("geoWithin")?;
			}
			if !Vec::is_empty(&self.r#global_location_number) {
				serialize_struct.serialize_field("globalLocationNumber", {
					struct SerializeWith<'a>(&'a Vec<GlobalLocationNumberProperty>);
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
					&SerializeWith(&self.r#global_location_number)
				})?;
			} else {
				serialize_struct.skip_field("globalLocationNumber")?;
			}
			if !Vec::is_empty(&self.r#has_drive_through_service) {
				serialize_struct.serialize_field("hasDriveThroughService", {
					struct SerializeWith<'a>(&'a Vec<HasDriveThroughServiceProperty>);
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
					&SerializeWith(&self.r#has_drive_through_service)
				})?;
			} else {
				serialize_struct.skip_field("hasDriveThroughService")?;
			}
			if !Vec::is_empty(&self.r#has_map) {
				serialize_struct.serialize_field("hasMap", {
					struct SerializeWith<'a>(&'a Vec<HasMapProperty>);
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
					&SerializeWith(&self.r#has_map)
				})?;
			} else {
				serialize_struct.skip_field("hasMap")?;
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
			if !Vec::is_empty(&self.r#isic_v_4) {
				serialize_struct.serialize_field("isicV4", {
					struct SerializeWith<'a>(&'a Vec<IsicV4Property>);
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
					&SerializeWith(&self.r#isic_v_4)
				})?;
			} else {
				serialize_struct.skip_field("isicV4")?;
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
			if !Vec::is_empty(&self.r#latitude) {
				serialize_struct.serialize_field("latitude", {
					struct SerializeWith<'a>(&'a Vec<LatitudeProperty>);
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
					&SerializeWith(&self.r#latitude)
				})?;
			} else {
				serialize_struct.skip_field("latitude")?;
			}
			if !Vec::is_empty(&self.r#logo) {
				serialize_struct.serialize_field("logo", {
					struct SerializeWith<'a>(&'a Vec<LogoProperty>);
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
					&SerializeWith(&self.r#logo)
				})?;
			} else {
				serialize_struct.skip_field("logo")?;
			}
			if !Vec::is_empty(&self.r#longitude) {
				serialize_struct.serialize_field("longitude", {
					struct SerializeWith<'a>(&'a Vec<LongitudeProperty>);
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
					&SerializeWith(&self.r#longitude)
				})?;
			} else {
				serialize_struct.skip_field("longitude")?;
			}
			if !Vec::is_empty(&self.r#map) {
				serialize_struct.serialize_field("map", {
					struct SerializeWith<'a>(&'a Vec<MapProperty>);
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
					&SerializeWith(&self.r#map)
				})?;
			} else {
				serialize_struct.skip_field("map")?;
			}
			if !Vec::is_empty(&self.r#maps) {
				serialize_struct.serialize_field("maps", {
					struct SerializeWith<'a>(&'a Vec<MapsProperty>);
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
					&SerializeWith(&self.r#maps)
				})?;
			} else {
				serialize_struct.skip_field("maps")?;
			}
			if !Vec::is_empty(&self.r#maximum_attendee_capacity) {
				serialize_struct.serialize_field("maximumAttendeeCapacity", {
					struct SerializeWith<'a>(&'a Vec<MaximumAttendeeCapacityProperty>);
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
					&SerializeWith(&self.r#maximum_attendee_capacity)
				})?;
			} else {
				serialize_struct.skip_field("maximumAttendeeCapacity")?;
			}
			if !Vec::is_empty(&self.r#opening_hours_specification) {
				serialize_struct.serialize_field("openingHoursSpecification", {
					struct SerializeWith<'a>(&'a Vec<OpeningHoursSpecificationProperty>);
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
					&SerializeWith(&self.r#opening_hours_specification)
				})?;
			} else {
				serialize_struct.skip_field("openingHoursSpecification")?;
			}
			if !Vec::is_empty(&self.r#photo) {
				serialize_struct.serialize_field("photo", {
					struct SerializeWith<'a>(&'a Vec<PhotoProperty>);
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
					&SerializeWith(&self.r#photo)
				})?;
			} else {
				serialize_struct.skip_field("photo")?;
			}
			if !Vec::is_empty(&self.r#photos) {
				serialize_struct.serialize_field("photos", {
					struct SerializeWith<'a>(&'a Vec<PhotosProperty>);
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
					&SerializeWith(&self.r#photos)
				})?;
			} else {
				serialize_struct.skip_field("photos")?;
			}
			if !Vec::is_empty(&self.r#public_access) {
				serialize_struct.serialize_field("publicAccess", {
					struct SerializeWith<'a>(&'a Vec<PublicAccessProperty>);
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
					&SerializeWith(&self.r#public_access)
				})?;
			} else {
				serialize_struct.skip_field("publicAccess")?;
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
			if !Vec::is_empty(&self.r#slogan) {
				serialize_struct.serialize_field("slogan", {
					struct SerializeWith<'a>(&'a Vec<SloganProperty>);
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
					&SerializeWith(&self.r#slogan)
				})?;
			} else {
				serialize_struct.skip_field("slogan")?;
			}
			if !Vec::is_empty(&self.r#smoking_allowed) {
				serialize_struct.serialize_field("smokingAllowed", {
					struct SerializeWith<'a>(&'a Vec<SmokingAllowedProperty>);
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
					&SerializeWith(&self.r#smoking_allowed)
				})?;
			} else {
				serialize_struct.skip_field("smokingAllowed")?;
			}
			if !Vec::is_empty(&self.r#special_opening_hours_specification) {
				serialize_struct.serialize_field("specialOpeningHoursSpecification", {
					struct SerializeWith<'a>(&'a Vec<SpecialOpeningHoursSpecificationProperty>);
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
					&SerializeWith(&self.r#special_opening_hours_specification)
				})?;
			} else {
				serialize_struct.skip_field("specialOpeningHoursSpecification")?;
			}
			if !Vec::is_empty(&self.r#telephone) {
				serialize_struct.serialize_field("telephone", {
					struct SerializeWith<'a>(&'a Vec<TelephoneProperty>);
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
					&SerializeWith(&self.r#telephone)
				})?;
			} else {
				serialize_struct.skip_field("telephone")?;
			}
			if !Vec::is_empty(&self.r#tour_booking_page) {
				serialize_struct.serialize_field("tourBookingPage", {
					struct SerializeWith<'a>(&'a Vec<TourBookingPageProperty>);
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
					&SerializeWith(&self.r#tour_booking_page)
				})?;
			} else {
				serialize_struct.skip_field("tourBookingPage")?;
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
	impl<'de> Deserialize<'de> for DefenceEstablishment {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				OpeningHours,
				AdditionalProperty,
				Address,
				AggregateRating,
				AmenityFeature,
				BranchCode,
				ContainedIn,
				ContainedInPlace,
				ContainsPlace,
				Event,
				Events,
				FaxNumber,
				Geo,
				GeoContains,
				GeoCoveredBy,
				GeoCovers,
				GeoCrosses,
				GeoDisjoint,
				GeoEquals,
				GeoIntersects,
				GeoOverlaps,
				GeoTouches,
				GeoWithin,
				GlobalLocationNumber,
				HasDriveThroughService,
				HasMap,
				IsAccessibleForFree,
				IsicV4,
				Keywords,
				Latitude,
				Logo,
				Longitude,
				Map,
				Maps,
				MaximumAttendeeCapacity,
				OpeningHoursSpecification,
				Photo,
				Photos,
				PublicAccess,
				Review,
				Reviews,
				Slogan,
				SmokingAllowed,
				SpecialOpeningHoursSpecification,
				Telephone,
				TourBookingPage,
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
						"openingHours" => Ok(Field::OpeningHours),
						"additionalProperty" => Ok(Field::AdditionalProperty),
						"address" => Ok(Field::Address),
						"aggregateRating" => Ok(Field::AggregateRating),
						"amenityFeature" => Ok(Field::AmenityFeature),
						"branchCode" => Ok(Field::BranchCode),
						"containedIn" => Ok(Field::ContainedIn),
						"containedInPlace" => Ok(Field::ContainedInPlace),
						"containsPlace" => Ok(Field::ContainsPlace),
						"event" => Ok(Field::Event),
						"events" => Ok(Field::Events),
						"faxNumber" => Ok(Field::FaxNumber),
						"geo" => Ok(Field::Geo),
						"geoContains" => Ok(Field::GeoContains),
						"geoCoveredBy" => Ok(Field::GeoCoveredBy),
						"geoCovers" => Ok(Field::GeoCovers),
						"geoCrosses" => Ok(Field::GeoCrosses),
						"geoDisjoint" => Ok(Field::GeoDisjoint),
						"geoEquals" => Ok(Field::GeoEquals),
						"geoIntersects" => Ok(Field::GeoIntersects),
						"geoOverlaps" => Ok(Field::GeoOverlaps),
						"geoTouches" => Ok(Field::GeoTouches),
						"geoWithin" => Ok(Field::GeoWithin),
						"globalLocationNumber" => Ok(Field::GlobalLocationNumber),
						"hasDriveThroughService" => Ok(Field::HasDriveThroughService),
						"hasMap" => Ok(Field::HasMap),
						"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						"isicV4" => Ok(Field::IsicV4),
						"keywords" => Ok(Field::Keywords),
						"latitude" => Ok(Field::Latitude),
						"logo" => Ok(Field::Logo),
						"longitude" => Ok(Field::Longitude),
						"map" => Ok(Field::Map),
						"maps" => Ok(Field::Maps),
						"maximumAttendeeCapacity" => Ok(Field::MaximumAttendeeCapacity),
						"openingHoursSpecification" => Ok(Field::OpeningHoursSpecification),
						"photo" => Ok(Field::Photo),
						"photos" => Ok(Field::Photos),
						"publicAccess" => Ok(Field::PublicAccess),
						"review" => Ok(Field::Review),
						"reviews" => Ok(Field::Reviews),
						"slogan" => Ok(Field::Slogan),
						"smokingAllowed" => Ok(Field::SmokingAllowed),
						"specialOpeningHoursSpecification" => {
							Ok(Field::SpecialOpeningHoursSpecification)
						}
						"telephone" => Ok(Field::Telephone),
						"tourBookingPage" => Ok(Field::TourBookingPage),
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
						"id" | "type" => Ok(Field::Ignore),
						_ => Err(de::Error::unknown_field(value, FIELDS)),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"openingHours" => Ok(Field::OpeningHours),
						b"additionalProperty" => Ok(Field::AdditionalProperty),
						b"address" => Ok(Field::Address),
						b"aggregateRating" => Ok(Field::AggregateRating),
						b"amenityFeature" => Ok(Field::AmenityFeature),
						b"branchCode" => Ok(Field::BranchCode),
						b"containedIn" => Ok(Field::ContainedIn),
						b"containedInPlace" => Ok(Field::ContainedInPlace),
						b"containsPlace" => Ok(Field::ContainsPlace),
						b"event" => Ok(Field::Event),
						b"events" => Ok(Field::Events),
						b"faxNumber" => Ok(Field::FaxNumber),
						b"geo" => Ok(Field::Geo),
						b"geoContains" => Ok(Field::GeoContains),
						b"geoCoveredBy" => Ok(Field::GeoCoveredBy),
						b"geoCovers" => Ok(Field::GeoCovers),
						b"geoCrosses" => Ok(Field::GeoCrosses),
						b"geoDisjoint" => Ok(Field::GeoDisjoint),
						b"geoEquals" => Ok(Field::GeoEquals),
						b"geoIntersects" => Ok(Field::GeoIntersects),
						b"geoOverlaps" => Ok(Field::GeoOverlaps),
						b"geoTouches" => Ok(Field::GeoTouches),
						b"geoWithin" => Ok(Field::GeoWithin),
						b"globalLocationNumber" => Ok(Field::GlobalLocationNumber),
						b"hasDriveThroughService" => Ok(Field::HasDriveThroughService),
						b"hasMap" => Ok(Field::HasMap),
						b"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						b"isicV4" => Ok(Field::IsicV4),
						b"keywords" => Ok(Field::Keywords),
						b"latitude" => Ok(Field::Latitude),
						b"logo" => Ok(Field::Logo),
						b"longitude" => Ok(Field::Longitude),
						b"map" => Ok(Field::Map),
						b"maps" => Ok(Field::Maps),
						b"maximumAttendeeCapacity" => Ok(Field::MaximumAttendeeCapacity),
						b"openingHoursSpecification" => Ok(Field::OpeningHoursSpecification),
						b"photo" => Ok(Field::Photo),
						b"photos" => Ok(Field::Photos),
						b"publicAccess" => Ok(Field::PublicAccess),
						b"review" => Ok(Field::Review),
						b"reviews" => Ok(Field::Reviews),
						b"slogan" => Ok(Field::Slogan),
						b"smokingAllowed" => Ok(Field::SmokingAllowed),
						b"specialOpeningHoursSpecification" => {
							Ok(Field::SpecialOpeningHoursSpecification)
						}
						b"telephone" => Ok(Field::Telephone),
						b"tourBookingPage" => Ok(Field::TourBookingPage),
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
				type Value = DefenceEstablishment;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema DefenceEstablishment")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#opening_hours_property = None;
					let mut r#additional_property_property = None;
					let mut r#address_property = None;
					let mut r#aggregate_rating_property = None;
					let mut r#amenity_feature_property = None;
					let mut r#branch_code_property = None;
					let mut r#contained_in_property = None;
					let mut r#contained_in_place_property = None;
					let mut r#contains_place_property = None;
					let mut r#event_property = None;
					let mut r#events_property = None;
					let mut r#fax_number_property = None;
					let mut r#geo_property = None;
					let mut r#geo_contains_property = None;
					let mut r#geo_covered_by_property = None;
					let mut r#geo_covers_property = None;
					let mut r#geo_crosses_property = None;
					let mut r#geo_disjoint_property = None;
					let mut r#geo_equals_property = None;
					let mut r#geo_intersects_property = None;
					let mut r#geo_overlaps_property = None;
					let mut r#geo_touches_property = None;
					let mut r#geo_within_property = None;
					let mut r#global_location_number_property = None;
					let mut r#has_drive_through_service_property = None;
					let mut r#has_map_property = None;
					let mut r#is_accessible_for_free_property = None;
					let mut r#isic_v_4_property = None;
					let mut r#keywords_property = None;
					let mut r#latitude_property = None;
					let mut r#logo_property = None;
					let mut r#longitude_property = None;
					let mut r#map_property = None;
					let mut r#maps_property = None;
					let mut r#maximum_attendee_capacity_property = None;
					let mut r#opening_hours_specification_property = None;
					let mut r#photo_property = None;
					let mut r#photos_property = None;
					let mut r#public_access_property = None;
					let mut r#review_property = None;
					let mut r#reviews_property = None;
					let mut r#slogan_property = None;
					let mut r#smoking_allowed_property = None;
					let mut r#special_opening_hours_specification_property = None;
					let mut r#telephone_property = None;
					let mut r#tour_booking_page_property = None;
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
							Field::OpeningHours => {
								if r#opening_hours_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"openingHours",
									));
								}
								r#opening_hours_property = Some({
									struct DeserializeWith(Vec<OpeningHoursProperty>);
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
							Field::AdditionalProperty => {
								if r#additional_property_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"additionalProperty",
									));
								}
								r#additional_property_property = Some({
									struct DeserializeWith(Vec<AdditionalPropertyProperty>);
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
							Field::Address => {
								if r#address_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"address",
									));
								}
								r#address_property = Some({
									struct DeserializeWith(Vec<AddressProperty>);
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
							Field::AmenityFeature => {
								if r#amenity_feature_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"amenityFeature",
									));
								}
								r#amenity_feature_property = Some({
									struct DeserializeWith(Vec<AmenityFeatureProperty>);
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
							Field::BranchCode => {
								if r#branch_code_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"branchCode",
									));
								}
								r#branch_code_property = Some({
									struct DeserializeWith(Vec<BranchCodeProperty>);
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
							Field::ContainedIn => {
								if r#contained_in_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"containedIn",
									));
								}
								r#contained_in_property = Some({
									struct DeserializeWith(Vec<ContainedInProperty>);
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
							Field::ContainedInPlace => {
								if r#contained_in_place_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"containedInPlace",
									));
								}
								r#contained_in_place_property = Some({
									struct DeserializeWith(Vec<ContainedInPlaceProperty>);
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
							Field::ContainsPlace => {
								if r#contains_place_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"containsPlace",
									));
								}
								r#contains_place_property = Some({
									struct DeserializeWith(Vec<ContainsPlaceProperty>);
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
							Field::Event => {
								if r#event_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("event"));
								}
								r#event_property = Some({
									struct DeserializeWith(Vec<EventProperty>);
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
							Field::Events => {
								if r#events_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("events"));
								}
								r#events_property = Some({
									struct DeserializeWith(Vec<EventsProperty>);
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
							Field::FaxNumber => {
								if r#fax_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"faxNumber",
									));
								}
								r#fax_number_property = Some({
									struct DeserializeWith(Vec<FaxNumberProperty>);
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
							Field::Geo => {
								if r#geo_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("geo"));
								}
								r#geo_property = Some({
									struct DeserializeWith(Vec<GeoProperty>);
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
							Field::GeoContains => {
								if r#geo_contains_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoContains",
									));
								}
								r#geo_contains_property = Some({
									struct DeserializeWith(Vec<GeoContainsProperty>);
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
							Field::GeoCoveredBy => {
								if r#geo_covered_by_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoCoveredBy",
									));
								}
								r#geo_covered_by_property = Some({
									struct DeserializeWith(Vec<GeoCoveredByProperty>);
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
							Field::GeoCovers => {
								if r#geo_covers_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoCovers",
									));
								}
								r#geo_covers_property = Some({
									struct DeserializeWith(Vec<GeoCoversProperty>);
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
							Field::GeoCrosses => {
								if r#geo_crosses_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoCrosses",
									));
								}
								r#geo_crosses_property = Some({
									struct DeserializeWith(Vec<GeoCrossesProperty>);
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
							Field::GeoDisjoint => {
								if r#geo_disjoint_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoDisjoint",
									));
								}
								r#geo_disjoint_property = Some({
									struct DeserializeWith(Vec<GeoDisjointProperty>);
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
							Field::GeoEquals => {
								if r#geo_equals_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoEquals",
									));
								}
								r#geo_equals_property = Some({
									struct DeserializeWith(Vec<GeoEqualsProperty>);
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
							Field::GeoIntersects => {
								if r#geo_intersects_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoIntersects",
									));
								}
								r#geo_intersects_property = Some({
									struct DeserializeWith(Vec<GeoIntersectsProperty>);
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
							Field::GeoOverlaps => {
								if r#geo_overlaps_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoOverlaps",
									));
								}
								r#geo_overlaps_property = Some({
									struct DeserializeWith(Vec<GeoOverlapsProperty>);
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
							Field::GeoTouches => {
								if r#geo_touches_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoTouches",
									));
								}
								r#geo_touches_property = Some({
									struct DeserializeWith(Vec<GeoTouchesProperty>);
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
							Field::GeoWithin => {
								if r#geo_within_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"geoWithin",
									));
								}
								r#geo_within_property = Some({
									struct DeserializeWith(Vec<GeoWithinProperty>);
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
							Field::GlobalLocationNumber => {
								if r#global_location_number_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"globalLocationNumber",
									));
								}
								r#global_location_number_property = Some({
									struct DeserializeWith(Vec<GlobalLocationNumberProperty>);
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
							Field::HasDriveThroughService => {
								if r#has_drive_through_service_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasDriveThroughService",
									));
								}
								r#has_drive_through_service_property = Some({
									struct DeserializeWith(Vec<HasDriveThroughServiceProperty>);
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
							Field::HasMap => {
								if r#has_map_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("hasMap"));
								}
								r#has_map_property = Some({
									struct DeserializeWith(Vec<HasMapProperty>);
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
							Field::IsicV4 => {
								if r#isic_v_4_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("isicV4"));
								}
								r#isic_v_4_property = Some({
									struct DeserializeWith(Vec<IsicV4Property>);
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
							Field::Latitude => {
								if r#latitude_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"latitude",
									));
								}
								r#latitude_property = Some({
									struct DeserializeWith(Vec<LatitudeProperty>);
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
							Field::Logo => {
								if r#logo_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("logo"));
								}
								r#logo_property = Some({
									struct DeserializeWith(Vec<LogoProperty>);
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
							Field::Longitude => {
								if r#longitude_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"longitude",
									));
								}
								r#longitude_property = Some({
									struct DeserializeWith(Vec<LongitudeProperty>);
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
							Field::Map => {
								if r#map_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("map"));
								}
								r#map_property = Some({
									struct DeserializeWith(Vec<MapProperty>);
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
							Field::Maps => {
								if r#maps_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("maps"));
								}
								r#maps_property = Some({
									struct DeserializeWith(Vec<MapsProperty>);
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
							Field::MaximumAttendeeCapacity => {
								if r#maximum_attendee_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maximumAttendeeCapacity",
									));
								}
								r#maximum_attendee_capacity_property = Some({
									struct DeserializeWith(Vec<MaximumAttendeeCapacityProperty>);
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
							Field::OpeningHoursSpecification => {
								if r#opening_hours_specification_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"openingHoursSpecification",
									));
								}
								r#opening_hours_specification_property = Some({
									struct DeserializeWith(Vec<OpeningHoursSpecificationProperty>);
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
							Field::Photo => {
								if r#photo_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("photo"));
								}
								r#photo_property = Some({
									struct DeserializeWith(Vec<PhotoProperty>);
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
							Field::Photos => {
								if r#photos_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("photos"));
								}
								r#photos_property = Some({
									struct DeserializeWith(Vec<PhotosProperty>);
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
							Field::PublicAccess => {
								if r#public_access_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"publicAccess",
									));
								}
								r#public_access_property = Some({
									struct DeserializeWith(Vec<PublicAccessProperty>);
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
							Field::Slogan => {
								if r#slogan_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("slogan"));
								}
								r#slogan_property = Some({
									struct DeserializeWith(Vec<SloganProperty>);
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
							Field::SmokingAllowed => {
								if r#smoking_allowed_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"smokingAllowed",
									));
								}
								r#smoking_allowed_property = Some({
									struct DeserializeWith(Vec<SmokingAllowedProperty>);
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
							Field::SpecialOpeningHoursSpecification => {
								if r#special_opening_hours_specification_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"specialOpeningHoursSpecification",
									));
								}
								r#special_opening_hours_specification_property = Some({
									struct DeserializeWith(
										Vec<SpecialOpeningHoursSpecificationProperty>,
									);
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
							Field::Telephone => {
								if r#telephone_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"telephone",
									));
								}
								r#telephone_property = Some({
									struct DeserializeWith(Vec<TelephoneProperty>);
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
							Field::TourBookingPage => {
								if r#tour_booking_page_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"tourBookingPage",
									));
								}
								r#tour_booking_page_property = Some({
									struct DeserializeWith(Vec<TourBookingPageProperty>);
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
							Field::Ignore => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(DefenceEstablishment {
						r#opening_hours: r#opening_hours_property.unwrap_or_default(),
						r#additional_property: r#additional_property_property.unwrap_or_default(),
						r#address: r#address_property.unwrap_or_default(),
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						r#amenity_feature: r#amenity_feature_property.unwrap_or_default(),
						r#branch_code: r#branch_code_property.unwrap_or_default(),
						r#contained_in: r#contained_in_property.unwrap_or_default(),
						r#contained_in_place: r#contained_in_place_property.unwrap_or_default(),
						r#contains_place: r#contains_place_property.unwrap_or_default(),
						r#event: r#event_property.unwrap_or_default(),
						r#events: r#events_property.unwrap_or_default(),
						r#fax_number: r#fax_number_property.unwrap_or_default(),
						r#geo: r#geo_property.unwrap_or_default(),
						r#geo_contains: r#geo_contains_property.unwrap_or_default(),
						r#geo_covered_by: r#geo_covered_by_property.unwrap_or_default(),
						r#geo_covers: r#geo_covers_property.unwrap_or_default(),
						r#geo_crosses: r#geo_crosses_property.unwrap_or_default(),
						r#geo_disjoint: r#geo_disjoint_property.unwrap_or_default(),
						r#geo_equals: r#geo_equals_property.unwrap_or_default(),
						r#geo_intersects: r#geo_intersects_property.unwrap_or_default(),
						r#geo_overlaps: r#geo_overlaps_property.unwrap_or_default(),
						r#geo_touches: r#geo_touches_property.unwrap_or_default(),
						r#geo_within: r#geo_within_property.unwrap_or_default(),
						r#global_location_number: r#global_location_number_property
							.unwrap_or_default(),
						r#has_drive_through_service: r#has_drive_through_service_property
							.unwrap_or_default(),
						r#has_map: r#has_map_property.unwrap_or_default(),
						r#is_accessible_for_free: r#is_accessible_for_free_property
							.unwrap_or_default(),
						r#isic_v_4: r#isic_v_4_property.unwrap_or_default(),
						r#keywords: r#keywords_property.unwrap_or_default(),
						r#latitude: r#latitude_property.unwrap_or_default(),
						r#logo: r#logo_property.unwrap_or_default(),
						r#longitude: r#longitude_property.unwrap_or_default(),
						r#map: r#map_property.unwrap_or_default(),
						r#maps: r#maps_property.unwrap_or_default(),
						r#maximum_attendee_capacity: r#maximum_attendee_capacity_property
							.unwrap_or_default(),
						r#opening_hours_specification: r#opening_hours_specification_property
							.unwrap_or_default(),
						r#photo: r#photo_property.unwrap_or_default(),
						r#photos: r#photos_property.unwrap_or_default(),
						r#public_access: r#public_access_property.unwrap_or_default(),
						r#review: r#review_property.unwrap_or_default(),
						r#reviews: r#reviews_property.unwrap_or_default(),
						r#slogan: r#slogan_property.unwrap_or_default(),
						r#smoking_allowed: r#smoking_allowed_property.unwrap_or_default(),
						r#special_opening_hours_specification:
							r#special_opening_hours_specification_property.unwrap_or_default(),
						r#telephone: r#telephone_property.unwrap_or_default(),
						r#tour_booking_page: r#tour_booking_page_property.unwrap_or_default(),
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
				"openingHours",
				"additionalProperty",
				"address",
				"aggregateRating",
				"amenityFeature",
				"branchCode",
				"containedIn",
				"containedInPlace",
				"containsPlace",
				"event",
				"events",
				"faxNumber",
				"geo",
				"geoContains",
				"geoCoveredBy",
				"geoCovers",
				"geoCrosses",
				"geoDisjoint",
				"geoEquals",
				"geoIntersects",
				"geoOverlaps",
				"geoTouches",
				"geoWithin",
				"globalLocationNumber",
				"hasDriveThroughService",
				"hasMap",
				"isAccessibleForFree",
				"isicV4",
				"keywords",
				"latitude",
				"logo",
				"longitude",
				"map",
				"maps",
				"maximumAttendeeCapacity",
				"openingHoursSpecification",
				"photo",
				"photos",
				"publicAccess",
				"review",
				"reviews",
				"slogan",
				"smokingAllowed",
				"specialOpeningHoursSpecification",
				"telephone",
				"tourBookingPage",
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
			deserializer.deserialize_struct("DefenceEstablishment", FIELDS, ClassVisitor)
		}
	}
}
