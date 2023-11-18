use super::*;
/// <https://schema.org/ExhibitionEvent>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct ExhibitionEvent {
	/// <https://schema.org/about>
	pub r#about: Vec<AboutProperty>,
	/// <https://schema.org/actor>
	pub r#actor: Vec<ActorProperty>,
	/// <https://schema.org/aggregateRating>
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
	/// <https://schema.org/attendee>
	pub r#attendee: Vec<AttendeeProperty>,
	/// <https://schema.org/attendees>
	#[deprecated = "This schema is superseded by <https://schema.org/attendee>."]
	pub r#attendees: Vec<AttendeesProperty>,
	/// <https://schema.org/audience>
	pub r#audience: Vec<AudienceProperty>,
	/// <https://schema.org/composer>
	pub r#composer: Vec<ComposerProperty>,
	/// <https://schema.org/contributor>
	pub r#contributor: Vec<ContributorProperty>,
	/// <https://schema.org/director>
	pub r#director: Vec<DirectorProperty>,
	/// <https://schema.org/doorTime>
	pub r#door_time: Vec<DoorTimeProperty>,
	/// <https://schema.org/duration>
	pub r#duration: Vec<DurationProperty>,
	/// <https://schema.org/endDate>
	pub r#end_date: Vec<EndDateProperty>,
	/// <https://schema.org/eventAttendanceMode>
	pub r#event_attendance_mode: Vec<EventAttendanceModeProperty>,
	/// <https://schema.org/eventSchedule>
	pub r#event_schedule: Vec<EventScheduleProperty>,
	/// <https://schema.org/eventStatus>
	pub r#event_status: Vec<EventStatusProperty>,
	/// <https://schema.org/funder>
	pub r#funder: Vec<FunderProperty>,
	/// <https://schema.org/funding>
	pub r#funding: Vec<FundingProperty>,
	/// <https://schema.org/inLanguage>
	pub r#in_language: Vec<InLanguageProperty>,
	/// <https://schema.org/isAccessibleForFree>
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	/// <https://schema.org/keywords>
	pub r#keywords: Vec<KeywordsProperty>,
	/// <https://schema.org/location>
	pub r#location: Vec<LocationProperty>,
	/// <https://schema.org/maximumAttendeeCapacity>
	pub r#maximum_attendee_capacity: Vec<MaximumAttendeeCapacityProperty>,
	/// <https://schema.org/maximumPhysicalAttendeeCapacity>
	pub r#maximum_physical_attendee_capacity: Vec<MaximumPhysicalAttendeeCapacityProperty>,
	/// <https://schema.org/maximumVirtualAttendeeCapacity>
	pub r#maximum_virtual_attendee_capacity: Vec<MaximumVirtualAttendeeCapacityProperty>,
	/// <https://schema.org/offers>
	pub r#offers: Vec<OffersProperty>,
	/// <https://schema.org/organizer>
	pub r#organizer: Vec<OrganizerProperty>,
	/// <https://schema.org/performer>
	pub r#performer: Vec<PerformerProperty>,
	/// <https://schema.org/performers>
	#[deprecated = "This schema is superseded by <https://schema.org/performer>."]
	pub r#performers: Vec<PerformersProperty>,
	/// <https://schema.org/previousStartDate>
	pub r#previous_start_date: Vec<PreviousStartDateProperty>,
	/// <https://schema.org/recordedIn>
	pub r#recorded_in: Vec<RecordedInProperty>,
	/// <https://schema.org/remainingAttendeeCapacity>
	pub r#remaining_attendee_capacity: Vec<RemainingAttendeeCapacityProperty>,
	/// <https://schema.org/review>
	pub r#review: Vec<ReviewProperty>,
	/// <https://schema.org/sponsor>
	pub r#sponsor: Vec<SponsorProperty>,
	/// <https://schema.org/startDate>
	pub r#start_date: Vec<StartDateProperty>,
	/// <https://schema.org/subEvent>
	pub r#sub_event: Vec<SubEventProperty>,
	/// <https://schema.org/subEvents>
	#[deprecated = "This schema is superseded by <https://schema.org/subEvent>."]
	pub r#sub_events: Vec<SubEventsProperty>,
	/// <https://schema.org/superEvent>
	pub r#super_event: Vec<SuperEventProperty>,
	/// <https://schema.org/translator>
	pub r#translator: Vec<TranslatorProperty>,
	/// <https://schema.org/typicalAgeRange>
	pub r#typical_age_range: Vec<TypicalAgeRangeProperty>,
	/// <https://schema.org/workFeatured>
	pub r#work_featured: Vec<WorkFeaturedProperty>,
	/// <https://schema.org/workPerformed>
	pub r#work_performed: Vec<WorkPerformedProperty>,
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
/// This trait is for properties from <https://schema.org/ExhibitionEvent>.
pub trait ExhibitionEventTrait {}
impl ExhibitionEventTrait for ExhibitionEvent {}
impl EventTrait for ExhibitionEvent {
	fn get_about(&self) -> &[AboutProperty] {
		self.r#about.as_slice()
	}
	fn take_about(&mut self) -> Vec<AboutProperty> {
		std::mem::take(&mut self.r#about)
	}
	fn get_actor(&self) -> &[ActorProperty] {
		self.r#actor.as_slice()
	}
	fn take_actor(&mut self) -> Vec<ActorProperty> {
		std::mem::take(&mut self.r#actor)
	}
	fn get_aggregate_rating(&self) -> &[AggregateRatingProperty] {
		self.r#aggregate_rating.as_slice()
	}
	fn take_aggregate_rating(&mut self) -> Vec<AggregateRatingProperty> {
		std::mem::take(&mut self.r#aggregate_rating)
	}
	fn get_attendee(&self) -> &[AttendeeProperty] {
		self.r#attendee.as_slice()
	}
	fn take_attendee(&mut self) -> Vec<AttendeeProperty> {
		std::mem::take(&mut self.r#attendee)
	}
	fn get_attendees(&self) -> &[AttendeesProperty] {
		self.r#attendees.as_slice()
	}
	fn take_attendees(&mut self) -> Vec<AttendeesProperty> {
		std::mem::take(&mut self.r#attendees)
	}
	fn get_audience(&self) -> &[AudienceProperty] {
		self.r#audience.as_slice()
	}
	fn take_audience(&mut self) -> Vec<AudienceProperty> {
		std::mem::take(&mut self.r#audience)
	}
	fn get_composer(&self) -> &[ComposerProperty] {
		self.r#composer.as_slice()
	}
	fn take_composer(&mut self) -> Vec<ComposerProperty> {
		std::mem::take(&mut self.r#composer)
	}
	fn get_contributor(&self) -> &[ContributorProperty] {
		self.r#contributor.as_slice()
	}
	fn take_contributor(&mut self) -> Vec<ContributorProperty> {
		std::mem::take(&mut self.r#contributor)
	}
	fn get_director(&self) -> &[DirectorProperty] {
		self.r#director.as_slice()
	}
	fn take_director(&mut self) -> Vec<DirectorProperty> {
		std::mem::take(&mut self.r#director)
	}
	fn get_door_time(&self) -> &[DoorTimeProperty] {
		self.r#door_time.as_slice()
	}
	fn take_door_time(&mut self) -> Vec<DoorTimeProperty> {
		std::mem::take(&mut self.r#door_time)
	}
	fn get_duration(&self) -> &[DurationProperty] {
		self.r#duration.as_slice()
	}
	fn take_duration(&mut self) -> Vec<DurationProperty> {
		std::mem::take(&mut self.r#duration)
	}
	fn get_end_date(&self) -> &[EndDateProperty] {
		self.r#end_date.as_slice()
	}
	fn take_end_date(&mut self) -> Vec<EndDateProperty> {
		std::mem::take(&mut self.r#end_date)
	}
	fn get_event_attendance_mode(&self) -> &[EventAttendanceModeProperty] {
		self.r#event_attendance_mode.as_slice()
	}
	fn take_event_attendance_mode(&mut self) -> Vec<EventAttendanceModeProperty> {
		std::mem::take(&mut self.r#event_attendance_mode)
	}
	fn get_event_schedule(&self) -> &[EventScheduleProperty] {
		self.r#event_schedule.as_slice()
	}
	fn take_event_schedule(&mut self) -> Vec<EventScheduleProperty> {
		std::mem::take(&mut self.r#event_schedule)
	}
	fn get_event_status(&self) -> &[EventStatusProperty] {
		self.r#event_status.as_slice()
	}
	fn take_event_status(&mut self) -> Vec<EventStatusProperty> {
		std::mem::take(&mut self.r#event_status)
	}
	fn get_funder(&self) -> &[FunderProperty] {
		self.r#funder.as_slice()
	}
	fn take_funder(&mut self) -> Vec<FunderProperty> {
		std::mem::take(&mut self.r#funder)
	}
	fn get_funding(&self) -> &[FundingProperty] {
		self.r#funding.as_slice()
	}
	fn take_funding(&mut self) -> Vec<FundingProperty> {
		std::mem::take(&mut self.r#funding)
	}
	fn get_in_language(&self) -> &[InLanguageProperty] {
		self.r#in_language.as_slice()
	}
	fn take_in_language(&mut self) -> Vec<InLanguageProperty> {
		std::mem::take(&mut self.r#in_language)
	}
	fn get_is_accessible_for_free(&self) -> &[IsAccessibleForFreeProperty] {
		self.r#is_accessible_for_free.as_slice()
	}
	fn take_is_accessible_for_free(&mut self) -> Vec<IsAccessibleForFreeProperty> {
		std::mem::take(&mut self.r#is_accessible_for_free)
	}
	fn get_keywords(&self) -> &[KeywordsProperty] {
		self.r#keywords.as_slice()
	}
	fn take_keywords(&mut self) -> Vec<KeywordsProperty> {
		std::mem::take(&mut self.r#keywords)
	}
	fn get_location(&self) -> &[LocationProperty] {
		self.r#location.as_slice()
	}
	fn take_location(&mut self) -> Vec<LocationProperty> {
		std::mem::take(&mut self.r#location)
	}
	fn get_maximum_attendee_capacity(&self) -> &[MaximumAttendeeCapacityProperty] {
		self.r#maximum_attendee_capacity.as_slice()
	}
	fn take_maximum_attendee_capacity(&mut self) -> Vec<MaximumAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#maximum_attendee_capacity)
	}
	fn get_maximum_physical_attendee_capacity(&self) -> &[MaximumPhysicalAttendeeCapacityProperty] {
		self.r#maximum_physical_attendee_capacity.as_slice()
	}
	fn take_maximum_physical_attendee_capacity(
		&mut self,
	) -> Vec<MaximumPhysicalAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#maximum_physical_attendee_capacity)
	}
	fn get_maximum_virtual_attendee_capacity(&self) -> &[MaximumVirtualAttendeeCapacityProperty] {
		self.r#maximum_virtual_attendee_capacity.as_slice()
	}
	fn take_maximum_virtual_attendee_capacity(
		&mut self,
	) -> Vec<MaximumVirtualAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#maximum_virtual_attendee_capacity)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_organizer(&self) -> &[OrganizerProperty] {
		self.r#organizer.as_slice()
	}
	fn take_organizer(&mut self) -> Vec<OrganizerProperty> {
		std::mem::take(&mut self.r#organizer)
	}
	fn get_performer(&self) -> &[PerformerProperty] {
		self.r#performer.as_slice()
	}
	fn take_performer(&mut self) -> Vec<PerformerProperty> {
		std::mem::take(&mut self.r#performer)
	}
	fn get_performers(&self) -> &[PerformersProperty] {
		self.r#performers.as_slice()
	}
	fn take_performers(&mut self) -> Vec<PerformersProperty> {
		std::mem::take(&mut self.r#performers)
	}
	fn get_previous_start_date(&self) -> &[PreviousStartDateProperty] {
		self.r#previous_start_date.as_slice()
	}
	fn take_previous_start_date(&mut self) -> Vec<PreviousStartDateProperty> {
		std::mem::take(&mut self.r#previous_start_date)
	}
	fn get_recorded_in(&self) -> &[RecordedInProperty] {
		self.r#recorded_in.as_slice()
	}
	fn take_recorded_in(&mut self) -> Vec<RecordedInProperty> {
		std::mem::take(&mut self.r#recorded_in)
	}
	fn get_remaining_attendee_capacity(&self) -> &[RemainingAttendeeCapacityProperty] {
		self.r#remaining_attendee_capacity.as_slice()
	}
	fn take_remaining_attendee_capacity(&mut self) -> Vec<RemainingAttendeeCapacityProperty> {
		std::mem::take(&mut self.r#remaining_attendee_capacity)
	}
	fn get_review(&self) -> &[ReviewProperty] {
		self.r#review.as_slice()
	}
	fn take_review(&mut self) -> Vec<ReviewProperty> {
		std::mem::take(&mut self.r#review)
	}
	fn get_sponsor(&self) -> &[SponsorProperty] {
		self.r#sponsor.as_slice()
	}
	fn take_sponsor(&mut self) -> Vec<SponsorProperty> {
		std::mem::take(&mut self.r#sponsor)
	}
	fn get_start_date(&self) -> &[StartDateProperty] {
		self.r#start_date.as_slice()
	}
	fn take_start_date(&mut self) -> Vec<StartDateProperty> {
		std::mem::take(&mut self.r#start_date)
	}
	fn get_sub_event(&self) -> &[SubEventProperty] {
		self.r#sub_event.as_slice()
	}
	fn take_sub_event(&mut self) -> Vec<SubEventProperty> {
		std::mem::take(&mut self.r#sub_event)
	}
	fn get_sub_events(&self) -> &[SubEventsProperty] {
		self.r#sub_events.as_slice()
	}
	fn take_sub_events(&mut self) -> Vec<SubEventsProperty> {
		std::mem::take(&mut self.r#sub_events)
	}
	fn get_super_event(&self) -> &[SuperEventProperty] {
		self.r#super_event.as_slice()
	}
	fn take_super_event(&mut self) -> Vec<SuperEventProperty> {
		std::mem::take(&mut self.r#super_event)
	}
	fn get_translator(&self) -> &[TranslatorProperty] {
		self.r#translator.as_slice()
	}
	fn take_translator(&mut self) -> Vec<TranslatorProperty> {
		std::mem::take(&mut self.r#translator)
	}
	fn get_typical_age_range(&self) -> &[TypicalAgeRangeProperty] {
		self.r#typical_age_range.as_slice()
	}
	fn take_typical_age_range(&mut self) -> Vec<TypicalAgeRangeProperty> {
		std::mem::take(&mut self.r#typical_age_range)
	}
	fn get_work_featured(&self) -> &[WorkFeaturedProperty] {
		self.r#work_featured.as_slice()
	}
	fn take_work_featured(&mut self) -> Vec<WorkFeaturedProperty> {
		std::mem::take(&mut self.r#work_featured)
	}
	fn get_work_performed(&self) -> &[WorkPerformedProperty] {
		self.r#work_performed.as_slice()
	}
	fn take_work_performed(&mut self) -> Vec<WorkPerformedProperty> {
		std::mem::take(&mut self.r#work_performed)
	}
}
impl ThingTrait for ExhibitionEvent {
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
	impl Serialize for ExhibitionEvent {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#about) as usize,
				!Vec::is_empty(&self.r#actor) as usize,
				!Vec::is_empty(&self.r#aggregate_rating) as usize,
				!Vec::is_empty(&self.r#attendee) as usize,
				!Vec::is_empty(&self.r#attendees) as usize,
				!Vec::is_empty(&self.r#audience) as usize,
				!Vec::is_empty(&self.r#composer) as usize,
				!Vec::is_empty(&self.r#contributor) as usize,
				!Vec::is_empty(&self.r#director) as usize,
				!Vec::is_empty(&self.r#door_time) as usize,
				!Vec::is_empty(&self.r#duration) as usize,
				!Vec::is_empty(&self.r#end_date) as usize,
				!Vec::is_empty(&self.r#event_attendance_mode) as usize,
				!Vec::is_empty(&self.r#event_schedule) as usize,
				!Vec::is_empty(&self.r#event_status) as usize,
				!Vec::is_empty(&self.r#funder) as usize,
				!Vec::is_empty(&self.r#funding) as usize,
				!Vec::is_empty(&self.r#in_language) as usize,
				!Vec::is_empty(&self.r#is_accessible_for_free) as usize,
				!Vec::is_empty(&self.r#keywords) as usize,
				!Vec::is_empty(&self.r#location) as usize,
				!Vec::is_empty(&self.r#maximum_attendee_capacity) as usize,
				!Vec::is_empty(&self.r#maximum_physical_attendee_capacity) as usize,
				!Vec::is_empty(&self.r#maximum_virtual_attendee_capacity) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#organizer) as usize,
				!Vec::is_empty(&self.r#performer) as usize,
				!Vec::is_empty(&self.r#performers) as usize,
				!Vec::is_empty(&self.r#previous_start_date) as usize,
				!Vec::is_empty(&self.r#recorded_in) as usize,
				!Vec::is_empty(&self.r#remaining_attendee_capacity) as usize,
				!Vec::is_empty(&self.r#review) as usize,
				!Vec::is_empty(&self.r#sponsor) as usize,
				!Vec::is_empty(&self.r#start_date) as usize,
				!Vec::is_empty(&self.r#sub_event) as usize,
				!Vec::is_empty(&self.r#sub_events) as usize,
				!Vec::is_empty(&self.r#super_event) as usize,
				!Vec::is_empty(&self.r#translator) as usize,
				!Vec::is_empty(&self.r#typical_age_range) as usize,
				!Vec::is_empty(&self.r#work_featured) as usize,
				!Vec::is_empty(&self.r#work_performed) as usize,
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
				Serializer::serialize_struct(serializer, "ExhibitionEvent", len)?;
			if !Vec::is_empty(&self.r#about) {
				serialize_struct.serialize_field("about", {
					struct SerializeWith<'a>(&'a Vec<AboutProperty>);
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
					&SerializeWith(&self.r#about)
				})?;
			} else {
				serialize_struct.skip_field("about")?;
			}
			if !Vec::is_empty(&self.r#actor) {
				serialize_struct.serialize_field("actor", {
					struct SerializeWith<'a>(&'a Vec<ActorProperty>);
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
					&SerializeWith(&self.r#actor)
				})?;
			} else {
				serialize_struct.skip_field("actor")?;
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
			if !Vec::is_empty(&self.r#attendee) {
				serialize_struct.serialize_field("attendee", {
					struct SerializeWith<'a>(&'a Vec<AttendeeProperty>);
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
					&SerializeWith(&self.r#attendee)
				})?;
			} else {
				serialize_struct.skip_field("attendee")?;
			}
			if !Vec::is_empty(&self.r#attendees) {
				serialize_struct.serialize_field("attendees", {
					struct SerializeWith<'a>(&'a Vec<AttendeesProperty>);
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
					&SerializeWith(&self.r#attendees)
				})?;
			} else {
				serialize_struct.skip_field("attendees")?;
			}
			if !Vec::is_empty(&self.r#audience) {
				serialize_struct.serialize_field("audience", {
					struct SerializeWith<'a>(&'a Vec<AudienceProperty>);
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
					&SerializeWith(&self.r#audience)
				})?;
			} else {
				serialize_struct.skip_field("audience")?;
			}
			if !Vec::is_empty(&self.r#composer) {
				serialize_struct.serialize_field("composer", {
					struct SerializeWith<'a>(&'a Vec<ComposerProperty>);
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
					&SerializeWith(&self.r#composer)
				})?;
			} else {
				serialize_struct.skip_field("composer")?;
			}
			if !Vec::is_empty(&self.r#contributor) {
				serialize_struct.serialize_field("contributor", {
					struct SerializeWith<'a>(&'a Vec<ContributorProperty>);
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
					&SerializeWith(&self.r#contributor)
				})?;
			} else {
				serialize_struct.skip_field("contributor")?;
			}
			if !Vec::is_empty(&self.r#director) {
				serialize_struct.serialize_field("director", {
					struct SerializeWith<'a>(&'a Vec<DirectorProperty>);
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
					&SerializeWith(&self.r#director)
				})?;
			} else {
				serialize_struct.skip_field("director")?;
			}
			if !Vec::is_empty(&self.r#door_time) {
				serialize_struct.serialize_field("doorTime", {
					struct SerializeWith<'a>(&'a Vec<DoorTimeProperty>);
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
					&SerializeWith(&self.r#door_time)
				})?;
			} else {
				serialize_struct.skip_field("doorTime")?;
			}
			if !Vec::is_empty(&self.r#duration) {
				serialize_struct.serialize_field("duration", {
					struct SerializeWith<'a>(&'a Vec<DurationProperty>);
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
					&SerializeWith(&self.r#duration)
				})?;
			} else {
				serialize_struct.skip_field("duration")?;
			}
			if !Vec::is_empty(&self.r#end_date) {
				serialize_struct.serialize_field("endDate", {
					struct SerializeWith<'a>(&'a Vec<EndDateProperty>);
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
					&SerializeWith(&self.r#end_date)
				})?;
			} else {
				serialize_struct.skip_field("endDate")?;
			}
			if !Vec::is_empty(&self.r#event_attendance_mode) {
				serialize_struct.serialize_field("eventAttendanceMode", {
					struct SerializeWith<'a>(&'a Vec<EventAttendanceModeProperty>);
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
					&SerializeWith(&self.r#event_attendance_mode)
				})?;
			} else {
				serialize_struct.skip_field("eventAttendanceMode")?;
			}
			if !Vec::is_empty(&self.r#event_schedule) {
				serialize_struct.serialize_field("eventSchedule", {
					struct SerializeWith<'a>(&'a Vec<EventScheduleProperty>);
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
					&SerializeWith(&self.r#event_schedule)
				})?;
			} else {
				serialize_struct.skip_field("eventSchedule")?;
			}
			if !Vec::is_empty(&self.r#event_status) {
				serialize_struct.serialize_field("eventStatus", {
					struct SerializeWith<'a>(&'a Vec<EventStatusProperty>);
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
					&SerializeWith(&self.r#event_status)
				})?;
			} else {
				serialize_struct.skip_field("eventStatus")?;
			}
			if !Vec::is_empty(&self.r#funder) {
				serialize_struct.serialize_field("funder", {
					struct SerializeWith<'a>(&'a Vec<FunderProperty>);
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
					&SerializeWith(&self.r#funder)
				})?;
			} else {
				serialize_struct.skip_field("funder")?;
			}
			if !Vec::is_empty(&self.r#funding) {
				serialize_struct.serialize_field("funding", {
					struct SerializeWith<'a>(&'a Vec<FundingProperty>);
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
					&SerializeWith(&self.r#funding)
				})?;
			} else {
				serialize_struct.skip_field("funding")?;
			}
			if !Vec::is_empty(&self.r#in_language) {
				serialize_struct.serialize_field("inLanguage", {
					struct SerializeWith<'a>(&'a Vec<InLanguageProperty>);
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
					&SerializeWith(&self.r#in_language)
				})?;
			} else {
				serialize_struct.skip_field("inLanguage")?;
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
			if !Vec::is_empty(&self.r#location) {
				serialize_struct.serialize_field("location", {
					struct SerializeWith<'a>(&'a Vec<LocationProperty>);
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
					&SerializeWith(&self.r#location)
				})?;
			} else {
				serialize_struct.skip_field("location")?;
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
			if !Vec::is_empty(&self.r#maximum_physical_attendee_capacity) {
				serialize_struct.serialize_field("maximumPhysicalAttendeeCapacity", {
					struct SerializeWith<'a>(&'a Vec<MaximumPhysicalAttendeeCapacityProperty>);
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
					&SerializeWith(&self.r#maximum_physical_attendee_capacity)
				})?;
			} else {
				serialize_struct.skip_field("maximumPhysicalAttendeeCapacity")?;
			}
			if !Vec::is_empty(&self.r#maximum_virtual_attendee_capacity) {
				serialize_struct.serialize_field("maximumVirtualAttendeeCapacity", {
					struct SerializeWith<'a>(&'a Vec<MaximumVirtualAttendeeCapacityProperty>);
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
					&SerializeWith(&self.r#maximum_virtual_attendee_capacity)
				})?;
			} else {
				serialize_struct.skip_field("maximumVirtualAttendeeCapacity")?;
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
			if !Vec::is_empty(&self.r#organizer) {
				serialize_struct.serialize_field("organizer", {
					struct SerializeWith<'a>(&'a Vec<OrganizerProperty>);
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
					&SerializeWith(&self.r#organizer)
				})?;
			} else {
				serialize_struct.skip_field("organizer")?;
			}
			if !Vec::is_empty(&self.r#performer) {
				serialize_struct.serialize_field("performer", {
					struct SerializeWith<'a>(&'a Vec<PerformerProperty>);
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
					&SerializeWith(&self.r#performer)
				})?;
			} else {
				serialize_struct.skip_field("performer")?;
			}
			if !Vec::is_empty(&self.r#performers) {
				serialize_struct.serialize_field("performers", {
					struct SerializeWith<'a>(&'a Vec<PerformersProperty>);
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
					&SerializeWith(&self.r#performers)
				})?;
			} else {
				serialize_struct.skip_field("performers")?;
			}
			if !Vec::is_empty(&self.r#previous_start_date) {
				serialize_struct.serialize_field("previousStartDate", {
					struct SerializeWith<'a>(&'a Vec<PreviousStartDateProperty>);
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
					&SerializeWith(&self.r#previous_start_date)
				})?;
			} else {
				serialize_struct.skip_field("previousStartDate")?;
			}
			if !Vec::is_empty(&self.r#recorded_in) {
				serialize_struct.serialize_field("recordedIn", {
					struct SerializeWith<'a>(&'a Vec<RecordedInProperty>);
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
					&SerializeWith(&self.r#recorded_in)
				})?;
			} else {
				serialize_struct.skip_field("recordedIn")?;
			}
			if !Vec::is_empty(&self.r#remaining_attendee_capacity) {
				serialize_struct.serialize_field("remainingAttendeeCapacity", {
					struct SerializeWith<'a>(&'a Vec<RemainingAttendeeCapacityProperty>);
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
					&SerializeWith(&self.r#remaining_attendee_capacity)
				})?;
			} else {
				serialize_struct.skip_field("remainingAttendeeCapacity")?;
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
			if !Vec::is_empty(&self.r#sponsor) {
				serialize_struct.serialize_field("sponsor", {
					struct SerializeWith<'a>(&'a Vec<SponsorProperty>);
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
					&SerializeWith(&self.r#sponsor)
				})?;
			} else {
				serialize_struct.skip_field("sponsor")?;
			}
			if !Vec::is_empty(&self.r#start_date) {
				serialize_struct.serialize_field("startDate", {
					struct SerializeWith<'a>(&'a Vec<StartDateProperty>);
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
					&SerializeWith(&self.r#start_date)
				})?;
			} else {
				serialize_struct.skip_field("startDate")?;
			}
			if !Vec::is_empty(&self.r#sub_event) {
				serialize_struct.serialize_field("subEvent", {
					struct SerializeWith<'a>(&'a Vec<SubEventProperty>);
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
					&SerializeWith(&self.r#sub_event)
				})?;
			} else {
				serialize_struct.skip_field("subEvent")?;
			}
			if !Vec::is_empty(&self.r#sub_events) {
				serialize_struct.serialize_field("subEvents", {
					struct SerializeWith<'a>(&'a Vec<SubEventsProperty>);
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
					&SerializeWith(&self.r#sub_events)
				})?;
			} else {
				serialize_struct.skip_field("subEvents")?;
			}
			if !Vec::is_empty(&self.r#super_event) {
				serialize_struct.serialize_field("superEvent", {
					struct SerializeWith<'a>(&'a Vec<SuperEventProperty>);
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
					&SerializeWith(&self.r#super_event)
				})?;
			} else {
				serialize_struct.skip_field("superEvent")?;
			}
			if !Vec::is_empty(&self.r#translator) {
				serialize_struct.serialize_field("translator", {
					struct SerializeWith<'a>(&'a Vec<TranslatorProperty>);
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
					&SerializeWith(&self.r#translator)
				})?;
			} else {
				serialize_struct.skip_field("translator")?;
			}
			if !Vec::is_empty(&self.r#typical_age_range) {
				serialize_struct.serialize_field("typicalAgeRange", {
					struct SerializeWith<'a>(&'a Vec<TypicalAgeRangeProperty>);
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
					&SerializeWith(&self.r#typical_age_range)
				})?;
			} else {
				serialize_struct.skip_field("typicalAgeRange")?;
			}
			if !Vec::is_empty(&self.r#work_featured) {
				serialize_struct.serialize_field("workFeatured", {
					struct SerializeWith<'a>(&'a Vec<WorkFeaturedProperty>);
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
					&SerializeWith(&self.r#work_featured)
				})?;
			} else {
				serialize_struct.skip_field("workFeatured")?;
			}
			if !Vec::is_empty(&self.r#work_performed) {
				serialize_struct.serialize_field("workPerformed", {
					struct SerializeWith<'a>(&'a Vec<WorkPerformedProperty>);
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
					&SerializeWith(&self.r#work_performed)
				})?;
			} else {
				serialize_struct.skip_field("workPerformed")?;
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
	impl<'de> Deserialize<'de> for ExhibitionEvent {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				About,
				Actor,
				AggregateRating,
				Attendee,
				Attendees,
				Audience,
				Composer,
				Contributor,
				Director,
				DoorTime,
				Duration,
				EndDate,
				EventAttendanceMode,
				EventSchedule,
				EventStatus,
				Funder,
				Funding,
				InLanguage,
				IsAccessibleForFree,
				Keywords,
				Location,
				MaximumAttendeeCapacity,
				MaximumPhysicalAttendeeCapacity,
				MaximumVirtualAttendeeCapacity,
				Offers,
				Organizer,
				Performer,
				Performers,
				PreviousStartDate,
				RecordedIn,
				RemainingAttendeeCapacity,
				Review,
				Sponsor,
				StartDate,
				SubEvent,
				SubEvents,
				SuperEvent,
				Translator,
				TypicalAgeRange,
				WorkFeatured,
				WorkPerformed,
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
						"about" => Ok(Field::About),
						"actor" => Ok(Field::Actor),
						"aggregateRating" => Ok(Field::AggregateRating),
						"attendee" => Ok(Field::Attendee),
						"attendees" => Ok(Field::Attendees),
						"audience" => Ok(Field::Audience),
						"composer" => Ok(Field::Composer),
						"contributor" => Ok(Field::Contributor),
						"director" => Ok(Field::Director),
						"doorTime" => Ok(Field::DoorTime),
						"duration" => Ok(Field::Duration),
						"endDate" => Ok(Field::EndDate),
						"eventAttendanceMode" => Ok(Field::EventAttendanceMode),
						"eventSchedule" => Ok(Field::EventSchedule),
						"eventStatus" => Ok(Field::EventStatus),
						"funder" => Ok(Field::Funder),
						"funding" => Ok(Field::Funding),
						"inLanguage" => Ok(Field::InLanguage),
						"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						"keywords" => Ok(Field::Keywords),
						"location" => Ok(Field::Location),
						"maximumAttendeeCapacity" => Ok(Field::MaximumAttendeeCapacity),
						"maximumPhysicalAttendeeCapacity" => {
							Ok(Field::MaximumPhysicalAttendeeCapacity)
						}
						"maximumVirtualAttendeeCapacity" => {
							Ok(Field::MaximumVirtualAttendeeCapacity)
						}
						"offers" => Ok(Field::Offers),
						"organizer" => Ok(Field::Organizer),
						"performer" => Ok(Field::Performer),
						"performers" => Ok(Field::Performers),
						"previousStartDate" => Ok(Field::PreviousStartDate),
						"recordedIn" => Ok(Field::RecordedIn),
						"remainingAttendeeCapacity" => Ok(Field::RemainingAttendeeCapacity),
						"review" => Ok(Field::Review),
						"sponsor" => Ok(Field::Sponsor),
						"startDate" => Ok(Field::StartDate),
						"subEvent" => Ok(Field::SubEvent),
						"subEvents" => Ok(Field::SubEvents),
						"superEvent" => Ok(Field::SuperEvent),
						"translator" => Ok(Field::Translator),
						"typicalAgeRange" => Ok(Field::TypicalAgeRange),
						"workFeatured" => Ok(Field::WorkFeatured),
						"workPerformed" => Ok(Field::WorkPerformed),
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
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"about" => Ok(Field::About),
						b"actor" => Ok(Field::Actor),
						b"aggregateRating" => Ok(Field::AggregateRating),
						b"attendee" => Ok(Field::Attendee),
						b"attendees" => Ok(Field::Attendees),
						b"audience" => Ok(Field::Audience),
						b"composer" => Ok(Field::Composer),
						b"contributor" => Ok(Field::Contributor),
						b"director" => Ok(Field::Director),
						b"doorTime" => Ok(Field::DoorTime),
						b"duration" => Ok(Field::Duration),
						b"endDate" => Ok(Field::EndDate),
						b"eventAttendanceMode" => Ok(Field::EventAttendanceMode),
						b"eventSchedule" => Ok(Field::EventSchedule),
						b"eventStatus" => Ok(Field::EventStatus),
						b"funder" => Ok(Field::Funder),
						b"funding" => Ok(Field::Funding),
						b"inLanguage" => Ok(Field::InLanguage),
						b"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						b"keywords" => Ok(Field::Keywords),
						b"location" => Ok(Field::Location),
						b"maximumAttendeeCapacity" => Ok(Field::MaximumAttendeeCapacity),
						b"maximumPhysicalAttendeeCapacity" => {
							Ok(Field::MaximumPhysicalAttendeeCapacity)
						}
						b"maximumVirtualAttendeeCapacity" => {
							Ok(Field::MaximumVirtualAttendeeCapacity)
						}
						b"offers" => Ok(Field::Offers),
						b"organizer" => Ok(Field::Organizer),
						b"performer" => Ok(Field::Performer),
						b"performers" => Ok(Field::Performers),
						b"previousStartDate" => Ok(Field::PreviousStartDate),
						b"recordedIn" => Ok(Field::RecordedIn),
						b"remainingAttendeeCapacity" => Ok(Field::RemainingAttendeeCapacity),
						b"review" => Ok(Field::Review),
						b"sponsor" => Ok(Field::Sponsor),
						b"startDate" => Ok(Field::StartDate),
						b"subEvent" => Ok(Field::SubEvent),
						b"subEvents" => Ok(Field::SubEvents),
						b"superEvent" => Ok(Field::SuperEvent),
						b"translator" => Ok(Field::Translator),
						b"typicalAgeRange" => Ok(Field::TypicalAgeRange),
						b"workFeatured" => Ok(Field::WorkFeatured),
						b"workPerformed" => Ok(Field::WorkPerformed),
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
				type Value = ExhibitionEvent;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema ExhibitionEvent")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#about_property = None;
					let mut r#actor_property = None;
					let mut r#aggregate_rating_property = None;
					let mut r#attendee_property = None;
					let mut r#attendees_property = None;
					let mut r#audience_property = None;
					let mut r#composer_property = None;
					let mut r#contributor_property = None;
					let mut r#director_property = None;
					let mut r#door_time_property = None;
					let mut r#duration_property = None;
					let mut r#end_date_property = None;
					let mut r#event_attendance_mode_property = None;
					let mut r#event_schedule_property = None;
					let mut r#event_status_property = None;
					let mut r#funder_property = None;
					let mut r#funding_property = None;
					let mut r#in_language_property = None;
					let mut r#is_accessible_for_free_property = None;
					let mut r#keywords_property = None;
					let mut r#location_property = None;
					let mut r#maximum_attendee_capacity_property = None;
					let mut r#maximum_physical_attendee_capacity_property = None;
					let mut r#maximum_virtual_attendee_capacity_property = None;
					let mut r#offers_property = None;
					let mut r#organizer_property = None;
					let mut r#performer_property = None;
					let mut r#performers_property = None;
					let mut r#previous_start_date_property = None;
					let mut r#recorded_in_property = None;
					let mut r#remaining_attendee_capacity_property = None;
					let mut r#review_property = None;
					let mut r#sponsor_property = None;
					let mut r#start_date_property = None;
					let mut r#sub_event_property = None;
					let mut r#sub_events_property = None;
					let mut r#super_event_property = None;
					let mut r#translator_property = None;
					let mut r#typical_age_range_property = None;
					let mut r#work_featured_property = None;
					let mut r#work_performed_property = None;
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
							Field::About => {
								if r#about_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("about"));
								}
								r#about_property = Some({
									struct DeserializeWith(Vec<AboutProperty>);
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
							Field::Actor => {
								if r#actor_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("actor"));
								}
								r#actor_property = Some({
									struct DeserializeWith(Vec<ActorProperty>);
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
							Field::Attendee => {
								if r#attendee_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"attendee",
									));
								}
								r#attendee_property = Some({
									struct DeserializeWith(Vec<AttendeeProperty>);
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
							Field::Attendees => {
								if r#attendees_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"attendees",
									));
								}
								r#attendees_property = Some({
									struct DeserializeWith(Vec<AttendeesProperty>);
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
							Field::Audience => {
								if r#audience_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"audience",
									));
								}
								r#audience_property = Some({
									struct DeserializeWith(Vec<AudienceProperty>);
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
							Field::Composer => {
								if r#composer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"composer",
									));
								}
								r#composer_property = Some({
									struct DeserializeWith(Vec<ComposerProperty>);
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
							Field::Contributor => {
								if r#contributor_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"contributor",
									));
								}
								r#contributor_property = Some({
									struct DeserializeWith(Vec<ContributorProperty>);
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
							Field::Director => {
								if r#director_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"director",
									));
								}
								r#director_property = Some({
									struct DeserializeWith(Vec<DirectorProperty>);
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
							Field::DoorTime => {
								if r#door_time_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"doorTime",
									));
								}
								r#door_time_property = Some({
									struct DeserializeWith(Vec<DoorTimeProperty>);
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
							Field::Duration => {
								if r#duration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"duration",
									));
								}
								r#duration_property = Some({
									struct DeserializeWith(Vec<DurationProperty>);
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
							Field::EndDate => {
								if r#end_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"endDate",
									));
								}
								r#end_date_property = Some({
									struct DeserializeWith(Vec<EndDateProperty>);
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
							Field::EventAttendanceMode => {
								if r#event_attendance_mode_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eventAttendanceMode",
									));
								}
								r#event_attendance_mode_property = Some({
									struct DeserializeWith(Vec<EventAttendanceModeProperty>);
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
							Field::EventSchedule => {
								if r#event_schedule_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eventSchedule",
									));
								}
								r#event_schedule_property = Some({
									struct DeserializeWith(Vec<EventScheduleProperty>);
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
							Field::EventStatus => {
								if r#event_status_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"eventStatus",
									));
								}
								r#event_status_property = Some({
									struct DeserializeWith(Vec<EventStatusProperty>);
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
							Field::Funder => {
								if r#funder_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field("funder"));
								}
								r#funder_property = Some({
									struct DeserializeWith(Vec<FunderProperty>);
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
							Field::Funding => {
								if r#funding_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"funding",
									));
								}
								r#funding_property = Some({
									struct DeserializeWith(Vec<FundingProperty>);
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
							Field::InLanguage => {
								if r#in_language_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"inLanguage",
									));
								}
								r#in_language_property = Some({
									struct DeserializeWith(Vec<InLanguageProperty>);
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
							Field::Location => {
								if r#location_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"location",
									));
								}
								r#location_property = Some({
									struct DeserializeWith(Vec<LocationProperty>);
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
							Field::MaximumPhysicalAttendeeCapacity => {
								if r#maximum_physical_attendee_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maximumPhysicalAttendeeCapacity",
									));
								}
								r#maximum_physical_attendee_capacity_property = Some({
									struct DeserializeWith(
										Vec<MaximumPhysicalAttendeeCapacityProperty>,
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
							Field::MaximumVirtualAttendeeCapacity => {
								if r#maximum_virtual_attendee_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maximumVirtualAttendeeCapacity",
									));
								}
								r#maximum_virtual_attendee_capacity_property = Some({
									struct DeserializeWith(
										Vec<MaximumVirtualAttendeeCapacityProperty>,
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
							Field::Organizer => {
								if r#organizer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"organizer",
									));
								}
								r#organizer_property = Some({
									struct DeserializeWith(Vec<OrganizerProperty>);
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
							Field::Performer => {
								if r#performer_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"performer",
									));
								}
								r#performer_property = Some({
									struct DeserializeWith(Vec<PerformerProperty>);
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
							Field::Performers => {
								if r#performers_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"performers",
									));
								}
								r#performers_property = Some({
									struct DeserializeWith(Vec<PerformersProperty>);
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
							Field::PreviousStartDate => {
								if r#previous_start_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"previousStartDate",
									));
								}
								r#previous_start_date_property = Some({
									struct DeserializeWith(Vec<PreviousStartDateProperty>);
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
							Field::RecordedIn => {
								if r#recorded_in_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"recordedIn",
									));
								}
								r#recorded_in_property = Some({
									struct DeserializeWith(Vec<RecordedInProperty>);
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
							Field::RemainingAttendeeCapacity => {
								if r#remaining_attendee_capacity_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"remainingAttendeeCapacity",
									));
								}
								r#remaining_attendee_capacity_property = Some({
									struct DeserializeWith(Vec<RemainingAttendeeCapacityProperty>);
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
							Field::Sponsor => {
								if r#sponsor_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"sponsor",
									));
								}
								r#sponsor_property = Some({
									struct DeserializeWith(Vec<SponsorProperty>);
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
							Field::StartDate => {
								if r#start_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"startDate",
									));
								}
								r#start_date_property = Some({
									struct DeserializeWith(Vec<StartDateProperty>);
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
							Field::SubEvent => {
								if r#sub_event_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subEvent",
									));
								}
								r#sub_event_property = Some({
									struct DeserializeWith(Vec<SubEventProperty>);
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
							Field::SubEvents => {
								if r#sub_events_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"subEvents",
									));
								}
								r#sub_events_property = Some({
									struct DeserializeWith(Vec<SubEventsProperty>);
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
							Field::SuperEvent => {
								if r#super_event_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"superEvent",
									));
								}
								r#super_event_property = Some({
									struct DeserializeWith(Vec<SuperEventProperty>);
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
							Field::Translator => {
								if r#translator_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"translator",
									));
								}
								r#translator_property = Some({
									struct DeserializeWith(Vec<TranslatorProperty>);
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
							Field::TypicalAgeRange => {
								if r#typical_age_range_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"typicalAgeRange",
									));
								}
								r#typical_age_range_property = Some({
									struct DeserializeWith(Vec<TypicalAgeRangeProperty>);
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
							Field::WorkFeatured => {
								if r#work_featured_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"workFeatured",
									));
								}
								r#work_featured_property = Some({
									struct DeserializeWith(Vec<WorkFeaturedProperty>);
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
							Field::WorkPerformed => {
								if r#work_performed_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"workPerformed",
									));
								}
								r#work_performed_property = Some({
									struct DeserializeWith(Vec<WorkPerformedProperty>);
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
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(ExhibitionEvent {
						r#about: r#about_property.unwrap_or_default(),
						r#actor: r#actor_property.unwrap_or_default(),
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
						r#attendee: r#attendee_property.unwrap_or_default(),
						r#attendees: r#attendees_property.unwrap_or_default(),
						r#audience: r#audience_property.unwrap_or_default(),
						r#composer: r#composer_property.unwrap_or_default(),
						r#contributor: r#contributor_property.unwrap_or_default(),
						r#director: r#director_property.unwrap_or_default(),
						r#door_time: r#door_time_property.unwrap_or_default(),
						r#duration: r#duration_property.unwrap_or_default(),
						r#end_date: r#end_date_property.unwrap_or_default(),
						r#event_attendance_mode: r#event_attendance_mode_property
							.unwrap_or_default(),
						r#event_schedule: r#event_schedule_property.unwrap_or_default(),
						r#event_status: r#event_status_property.unwrap_or_default(),
						r#funder: r#funder_property.unwrap_or_default(),
						r#funding: r#funding_property.unwrap_or_default(),
						r#in_language: r#in_language_property.unwrap_or_default(),
						r#is_accessible_for_free: r#is_accessible_for_free_property
							.unwrap_or_default(),
						r#keywords: r#keywords_property.unwrap_or_default(),
						r#location: r#location_property.unwrap_or_default(),
						r#maximum_attendee_capacity: r#maximum_attendee_capacity_property
							.unwrap_or_default(),
						r#maximum_physical_attendee_capacity:
							r#maximum_physical_attendee_capacity_property.unwrap_or_default(),
						r#maximum_virtual_attendee_capacity:
							r#maximum_virtual_attendee_capacity_property.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#organizer: r#organizer_property.unwrap_or_default(),
						r#performer: r#performer_property.unwrap_or_default(),
						r#performers: r#performers_property.unwrap_or_default(),
						r#previous_start_date: r#previous_start_date_property.unwrap_or_default(),
						r#recorded_in: r#recorded_in_property.unwrap_or_default(),
						r#remaining_attendee_capacity: r#remaining_attendee_capacity_property
							.unwrap_or_default(),
						r#review: r#review_property.unwrap_or_default(),
						r#sponsor: r#sponsor_property.unwrap_or_default(),
						r#start_date: r#start_date_property.unwrap_or_default(),
						r#sub_event: r#sub_event_property.unwrap_or_default(),
						r#sub_events: r#sub_events_property.unwrap_or_default(),
						r#super_event: r#super_event_property.unwrap_or_default(),
						r#translator: r#translator_property.unwrap_or_default(),
						r#typical_age_range: r#typical_age_range_property.unwrap_or_default(),
						r#work_featured: r#work_featured_property.unwrap_or_default(),
						r#work_performed: r#work_performed_property.unwrap_or_default(),
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
				"about",
				"actor",
				"aggregateRating",
				"attendee",
				"attendees",
				"audience",
				"composer",
				"contributor",
				"director",
				"doorTime",
				"duration",
				"endDate",
				"eventAttendanceMode",
				"eventSchedule",
				"eventStatus",
				"funder",
				"funding",
				"inLanguage",
				"isAccessibleForFree",
				"keywords",
				"location",
				"maximumAttendeeCapacity",
				"maximumPhysicalAttendeeCapacity",
				"maximumVirtualAttendeeCapacity",
				"offers",
				"organizer",
				"performer",
				"performers",
				"previousStartDate",
				"recordedIn",
				"remainingAttendeeCapacity",
				"review",
				"sponsor",
				"startDate",
				"subEvent",
				"subEvents",
				"superEvent",
				"translator",
				"typicalAgeRange",
				"workFeatured",
				"workPerformed",
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
			deserializer.deserialize_struct("ExhibitionEvent", FIELDS, ClassVisitor)
		}
	}
}
