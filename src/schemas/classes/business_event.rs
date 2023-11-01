use super::*;
/// <https://schema.org/BusinessEvent>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct BusinessEvent {
	#[cfg(any(
		any(feature = "about-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#about: Vec<AboutProperty>,
	#[cfg(any(
		any(feature = "actor-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#actor: Vec<ActorProperty>,
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
			feature = "aggregate-rating-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#aggregate_rating: Vec<AggregateRatingProperty>,
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
			feature = "attendee-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#attendee: Vec<AttendeeProperty>,
	#[cfg(any(
		any(
			feature = "attendees-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#attendees: Vec<AttendeesProperty>,
	#[cfg(any(
		any(
			feature = "audience-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#audience: Vec<AudienceProperty>,
	#[cfg(any(
		any(
			feature = "composer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#composer: Vec<ComposerProperty>,
	#[cfg(any(
		any(
			feature = "contributor-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#contributor: Vec<ContributorProperty>,
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
			feature = "director-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#director: Vec<DirectorProperty>,
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
			feature = "door-time-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#door_time: Vec<DoorTimeProperty>,
	#[cfg(any(
		any(
			feature = "duration-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#duration: Vec<DurationProperty>,
	#[cfg(any(
		any(
			feature = "end-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#end_date: Vec<EndDateProperty>,
	#[cfg(any(
		any(
			feature = "event-attendance-mode-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#event_attendance_mode: Vec<EventAttendanceModeProperty>,
	#[cfg(any(
		any(
			feature = "event-schedule-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#event_schedule: Vec<EventScheduleProperty>,
	#[cfg(any(
		any(
			feature = "event-status-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#event_status: Vec<EventStatusProperty>,
	#[cfg(any(
		any(feature = "funder-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#funder: Vec<FunderProperty>,
	#[cfg(any(
		any(
			feature = "funding-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#funding: Vec<FundingProperty>,
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
			feature = "in-language-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#in_language: Vec<InLanguageProperty>,
	#[cfg(any(
		any(
			feature = "is-accessible-for-free-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#is_accessible_for_free: Vec<IsAccessibleForFreeProperty>,
	#[cfg(any(
		any(
			feature = "keywords-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#keywords: Vec<KeywordsProperty>,
	#[cfg(any(
		any(
			feature = "location-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#location: Vec<LocationProperty>,
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
			feature = "maximum-attendee-capacity-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#maximum_attendee_capacity: Vec<MaximumAttendeeCapacityProperty>,
	#[cfg(any(
		any(
			feature = "maximum-physical-attendee-capacity-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#maximum_physical_attendee_capacity: Vec<MaximumPhysicalAttendeeCapacityProperty>,
	#[cfg(any(
		any(
			feature = "maximum-virtual-attendee-capacity-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#maximum_virtual_attendee_capacity: Vec<MaximumVirtualAttendeeCapacityProperty>,
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
			feature = "organizer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#organizer: Vec<OrganizerProperty>,
	#[cfg(any(
		any(
			feature = "performer-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#performer: Vec<PerformerProperty>,
	#[cfg(any(
		any(
			feature = "performers-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#performers: Vec<PerformersProperty>,
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
			feature = "previous-start-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#previous_start_date: Vec<PreviousStartDateProperty>,
	#[cfg(any(
		any(
			feature = "recorded-in-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#recorded_in: Vec<RecordedInProperty>,
	#[cfg(any(
		any(
			feature = "remaining-attendee-capacity-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#remaining_attendee_capacity: Vec<RemainingAttendeeCapacityProperty>,
	#[cfg(any(
		any(feature = "review-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#review: Vec<ReviewProperty>,
	#[cfg(any(
		any(
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(
			feature = "sponsor-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#sponsor: Vec<SponsorProperty>,
	#[cfg(any(
		any(
			feature = "start-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#start_date: Vec<StartDateProperty>,
	#[cfg(any(
		any(
			feature = "sub-event-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#sub_event: Vec<SubEventProperty>,
	#[cfg(any(
		any(
			feature = "sub-events-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#sub_events: Vec<SubEventsProperty>,
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
			feature = "super-event-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#super_event: Vec<SuperEventProperty>,
	#[cfg(any(
		any(
			feature = "translator-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#translator: Vec<TranslatorProperty>,
	#[cfg(any(
		any(
			feature = "typical-age-range-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#typical_age_range: Vec<TypicalAgeRangeProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
	#[cfg(any(
		any(
			feature = "work-featured-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#work_featured: Vec<WorkFeaturedProperty>,
	#[cfg(any(
		any(
			feature = "work-performed-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#work_performed: Vec<WorkPerformedProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for BusinessEvent {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				if cfg!(any(
					any(
						feature = "about-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#about) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "actor-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#actor) as usize
				} else {
					0
				},
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
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#aggregate_rating) as usize
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
						feature = "attendee-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#attendee) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "attendees-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#attendees) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#audience) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "composer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#composer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "contributor-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#contributor) as usize
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
						feature = "director-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#director) as usize
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
						feature = "door-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#door_time) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#duration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "end-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#end_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "event-attendance-mode-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#event_attendance_mode) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "event-schedule-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#event_schedule) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "event-status-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#event_status) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "funder-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#funder) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#funding) as usize
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
						feature = "in-language-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#in_language) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "is-accessible-for-free-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#is_accessible_for_free) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#keywords) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "location-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#location) as usize
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
						feature = "maximum-attendee-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#maximum_attendee_capacity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "maximum-physical-attendee-capacity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#maximum_physical_attendee_capacity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "maximum-virtual-attendee-capacity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#maximum_virtual_attendee_capacity) as usize
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
						feature = "organizer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#organizer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "performer-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#performer) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "performers-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#performers) as usize
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
						feature = "previous-start-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#previous_start_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "recorded-in-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#recorded_in) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "remaining-attendee-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#remaining_attendee_capacity) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "review-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#review) as usize
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
						feature = "sponsor-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sponsor) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "start-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#start_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sub-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sub_event) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "sub-events-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#sub_events) as usize
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
						feature = "super-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#super_event) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "translator-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#translator) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "typical-age-range-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#typical_age_range) as usize
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
						feature = "work-featured-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#work_featured) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "work-performed-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#work_performed) as usize
				} else {
					0
				},
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "BusinessEvent", len)?;
			#[cfg(any(
				any(feature = "about-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(feature = "actor-property-schema", feature = "general-schema-section"),
				doc
			))]
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
					feature = "aggregate-rating-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "attendee-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "attendees-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "audience-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "composer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "contributor-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "director-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "door-time-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "duration-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "end-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "event-attendance-mode-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "event-schedule-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "event-status-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "funder-property-schema", feature = "general-schema-section"),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "funding-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
					feature = "in-language-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "is-accessible-for-free-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "keywords-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "location-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "maximum-attendee-capacity-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "maximum-physical-attendee-capacity-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "maximum-virtual-attendee-capacity-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
					feature = "organizer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "performer-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "performers-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "previous-start-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "recorded-in-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "remaining-attendee-capacity-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(feature = "review-property-schema", feature = "general-schema-section"),
				doc
			))]
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
				any(
					feature = "sponsor-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "start-date-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "sub-event-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "sub-events-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "super-event-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "translator-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "typical-age-range-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
					feature = "work-featured-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "work-performed-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for BusinessEvent {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				#[cfg(any(
					any(feature = "about-property-schema", feature = "general-schema-section"),
					doc
				))]
				About,
				#[cfg(any(
					any(feature = "actor-property-schema", feature = "general-schema-section"),
					doc
				))]
				Actor,
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
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AggregateRating,
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
						feature = "attendee-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Attendee,
				#[cfg(any(
					any(
						feature = "attendees-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Attendees,
				#[cfg(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Audience,
				#[cfg(any(
					any(
						feature = "composer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Composer,
				#[cfg(any(
					any(
						feature = "contributor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Contributor,
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
						feature = "director-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Director,
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
						feature = "door-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DoorTime,
				#[cfg(any(
					any(
						feature = "duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Duration,
				#[cfg(any(
					any(
						feature = "end-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EndDate,
				#[cfg(any(
					any(
						feature = "event-attendance-mode-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EventAttendanceMode,
				#[cfg(any(
					any(
						feature = "event-schedule-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EventSchedule,
				#[cfg(any(
					any(
						feature = "event-status-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EventStatus,
				#[cfg(any(
					any(feature = "funder-property-schema", feature = "general-schema-section"),
					doc
				))]
				Funder,
				#[cfg(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				Funding,
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
						feature = "in-language-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				InLanguage,
				#[cfg(any(
					any(
						feature = "is-accessible-for-free-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				IsAccessibleForFree,
				#[cfg(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Keywords,
				#[cfg(any(
					any(
						feature = "location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Location,
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
						feature = "maximum-attendee-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MaximumAttendeeCapacity,
				#[cfg(any(
					any(
						feature = "maximum-physical-attendee-capacity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MaximumPhysicalAttendeeCapacity,
				#[cfg(any(
					any(
						feature = "maximum-virtual-attendee-capacity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MaximumVirtualAttendeeCapacity,
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
						feature = "organizer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Organizer,
				#[cfg(any(
					any(
						feature = "performer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Performer,
				#[cfg(any(
					any(
						feature = "performers-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Performers,
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
						feature = "previous-start-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				PreviousStartDate,
				#[cfg(any(
					any(
						feature = "recorded-in-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RecordedIn,
				#[cfg(any(
					any(
						feature = "remaining-attendee-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				RemainingAttendeeCapacity,
				#[cfg(any(
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				Review,
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(
						feature = "sponsor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Sponsor,
				#[cfg(any(
					any(
						feature = "start-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				StartDate,
				#[cfg(any(
					any(
						feature = "sub-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubEvent,
				#[cfg(any(
					any(
						feature = "sub-events-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubEvents,
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
						feature = "super-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SuperEvent,
				#[cfg(any(
					any(
						feature = "translator-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				Translator,
				#[cfg(any(
					any(
						feature = "typical-age-range-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				TypicalAgeRange,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				Url,
				#[cfg(any(
					any(
						feature = "work-featured-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				WorkFeatured,
				#[cfg(any(
					any(
						feature = "work-performed-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				WorkPerformed,
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
								feature = "about-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"about" => Ok(Field::About),
						#[cfg(any(
							any(
								feature = "actor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"actor" => Ok(Field::Actor),
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
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"aggregateRating" => Ok(Field::AggregateRating),
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
								feature = "attendee-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"attendee" => Ok(Field::Attendee),
						#[cfg(any(
							any(
								feature = "attendees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"attendees" => Ok(Field::Attendees),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "composer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"composer" => Ok(Field::Composer),
						#[cfg(any(
							any(
								feature = "contributor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"contributor" => Ok(Field::Contributor),
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
								feature = "director-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"director" => Ok(Field::Director),
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
								feature = "door-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"doorTime" => Ok(Field::DoorTime),
						#[cfg(any(
							any(
								feature = "duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"duration" => Ok(Field::Duration),
						#[cfg(any(
							any(
								feature = "end-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"endDate" => Ok(Field::EndDate),
						#[cfg(any(
							any(
								feature = "event-attendance-mode-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"eventAttendanceMode" => Ok(Field::EventAttendanceMode),
						#[cfg(any(
							any(
								feature = "event-schedule-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"eventSchedule" => Ok(Field::EventSchedule),
						#[cfg(any(
							any(
								feature = "event-status-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"eventStatus" => Ok(Field::EventStatus),
						#[cfg(any(
							any(
								feature = "funder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"funder" => Ok(Field::Funder),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"funding" => Ok(Field::Funding),
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
								feature = "in-language-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"inLanguage" => Ok(Field::InLanguage),
						#[cfg(any(
							any(
								feature = "is-accessible-for-free-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"keywords" => Ok(Field::Keywords),
						#[cfg(any(
							any(
								feature = "location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"location" => Ok(Field::Location),
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
								feature = "maximum-attendee-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"maximumAttendeeCapacity" => Ok(Field::MaximumAttendeeCapacity),
						#[cfg(any(
							any(
								feature = "maximum-physical-attendee-capacity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"maximumPhysicalAttendeeCapacity" => Ok(Field::MaximumPhysicalAttendeeCapacity),
						#[cfg(any(
							any(
								feature = "maximum-virtual-attendee-capacity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"maximumVirtualAttendeeCapacity" => Ok(Field::MaximumVirtualAttendeeCapacity),
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
								feature = "organizer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"organizer" => Ok(Field::Organizer),
						#[cfg(any(
							any(
								feature = "performer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"performer" => Ok(Field::Performer),
						#[cfg(any(
							any(
								feature = "performers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"performers" => Ok(Field::Performers),
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
								feature = "previous-start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"previousStartDate" => Ok(Field::PreviousStartDate),
						#[cfg(any(
							any(
								feature = "recorded-in-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"recordedIn" => Ok(Field::RecordedIn),
						#[cfg(any(
							any(
								feature = "remaining-attendee-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"remainingAttendeeCapacity" => Ok(Field::RemainingAttendeeCapacity),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"review" => Ok(Field::Review),
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
								feature = "sponsor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sponsor" => Ok(Field::Sponsor),
						#[cfg(any(
							any(
								feature = "start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"startDate" => Ok(Field::StartDate),
						#[cfg(any(
							any(
								feature = "sub-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"subEvent" => Ok(Field::SubEvent),
						#[cfg(any(
							any(
								feature = "sub-events-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"subEvents" => Ok(Field::SubEvents),
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
								feature = "super-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"superEvent" => Ok(Field::SuperEvent),
						#[cfg(any(
							any(
								feature = "translator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"translator" => Ok(Field::Translator),
						#[cfg(any(
							any(
								feature = "typical-age-range-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"typicalAgeRange" => Ok(Field::TypicalAgeRange),
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
								feature = "work-featured-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"workFeatured" => Ok(Field::WorkFeatured),
						#[cfg(any(
							any(
								feature = "work-performed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"workPerformed" => Ok(Field::WorkPerformed),
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
								feature = "about-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"about" => Ok(Field::About),
						#[cfg(any(
							any(
								feature = "actor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"actor" => Ok(Field::Actor),
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
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"aggregateRating" => Ok(Field::AggregateRating),
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
								feature = "attendee-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"attendee" => Ok(Field::Attendee),
						#[cfg(any(
							any(
								feature = "attendees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"attendees" => Ok(Field::Attendees),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"audience" => Ok(Field::Audience),
						#[cfg(any(
							any(
								feature = "composer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"composer" => Ok(Field::Composer),
						#[cfg(any(
							any(
								feature = "contributor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"contributor" => Ok(Field::Contributor),
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
								feature = "director-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"director" => Ok(Field::Director),
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
								feature = "door-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"doorTime" => Ok(Field::DoorTime),
						#[cfg(any(
							any(
								feature = "duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"duration" => Ok(Field::Duration),
						#[cfg(any(
							any(
								feature = "end-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"endDate" => Ok(Field::EndDate),
						#[cfg(any(
							any(
								feature = "event-attendance-mode-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"eventAttendanceMode" => Ok(Field::EventAttendanceMode),
						#[cfg(any(
							any(
								feature = "event-schedule-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"eventSchedule" => Ok(Field::EventSchedule),
						#[cfg(any(
							any(
								feature = "event-status-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"eventStatus" => Ok(Field::EventStatus),
						#[cfg(any(
							any(
								feature = "funder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"funder" => Ok(Field::Funder),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"funding" => Ok(Field::Funding),
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
								feature = "in-language-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"inLanguage" => Ok(Field::InLanguage),
						#[cfg(any(
							any(
								feature = "is-accessible-for-free-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"isAccessibleForFree" => Ok(Field::IsAccessibleForFree),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"keywords" => Ok(Field::Keywords),
						#[cfg(any(
							any(
								feature = "location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"location" => Ok(Field::Location),
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
								feature = "maximum-attendee-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"maximumAttendeeCapacity" => Ok(Field::MaximumAttendeeCapacity),
						#[cfg(any(
							any(
								feature = "maximum-physical-attendee-capacity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"maximumPhysicalAttendeeCapacity" => Ok(Field::MaximumPhysicalAttendeeCapacity),
						#[cfg(any(
							any(
								feature = "maximum-virtual-attendee-capacity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"maximumVirtualAttendeeCapacity" => Ok(Field::MaximumVirtualAttendeeCapacity),
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
								feature = "organizer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"organizer" => Ok(Field::Organizer),
						#[cfg(any(
							any(
								feature = "performer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"performer" => Ok(Field::Performer),
						#[cfg(any(
							any(
								feature = "performers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"performers" => Ok(Field::Performers),
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
								feature = "previous-start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"previousStartDate" => Ok(Field::PreviousStartDate),
						#[cfg(any(
							any(
								feature = "recorded-in-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"recordedIn" => Ok(Field::RecordedIn),
						#[cfg(any(
							any(
								feature = "remaining-attendee-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"remainingAttendeeCapacity" => Ok(Field::RemainingAttendeeCapacity),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"review" => Ok(Field::Review),
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
								feature = "sponsor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sponsor" => Ok(Field::Sponsor),
						#[cfg(any(
							any(
								feature = "start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"startDate" => Ok(Field::StartDate),
						#[cfg(any(
							any(
								feature = "sub-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"subEvent" => Ok(Field::SubEvent),
						#[cfg(any(
							any(
								feature = "sub-events-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"subEvents" => Ok(Field::SubEvents),
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
								feature = "super-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"superEvent" => Ok(Field::SuperEvent),
						#[cfg(any(
							any(
								feature = "translator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"translator" => Ok(Field::Translator),
						#[cfg(any(
							any(
								feature = "typical-age-range-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"typicalAgeRange" => Ok(Field::TypicalAgeRange),
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
								feature = "work-featured-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"workFeatured" => Ok(Field::WorkFeatured),
						#[cfg(any(
							any(
								feature = "work-performed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"workPerformed" => Ok(Field::WorkPerformed),
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
				type Value = BusinessEvent;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema BusinessEvent")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					#[cfg(any(
						any(feature = "about-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#about_property = None;
					#[cfg(any(
						any(feature = "actor-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#actor_property = None;
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
							feature = "aggregate-rating-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#aggregate_rating_property = None;
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
							feature = "attendee-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#attendee_property = None;
					#[cfg(any(
						any(
							feature = "attendees-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#attendees_property = None;
					#[cfg(any(
						any(
							feature = "audience-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#audience_property = None;
					#[cfg(any(
						any(
							feature = "composer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#composer_property = None;
					#[cfg(any(
						any(
							feature = "contributor-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#contributor_property = None;
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
							feature = "director-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#director_property = None;
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
							feature = "door-time-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#door_time_property = None;
					#[cfg(any(
						any(
							feature = "duration-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#duration_property = None;
					#[cfg(any(
						any(
							feature = "end-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#end_date_property = None;
					#[cfg(any(
						any(
							feature = "event-attendance-mode-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#event_attendance_mode_property = None;
					#[cfg(any(
						any(
							feature = "event-schedule-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#event_schedule_property = None;
					#[cfg(any(
						any(
							feature = "event-status-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#event_status_property = None;
					#[cfg(any(
						any(
							feature = "funder-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#funder_property = None;
					#[cfg(any(
						any(
							feature = "funding-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#funding_property = None;
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
							feature = "in-language-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#in_language_property = None;
					#[cfg(any(
						any(
							feature = "is-accessible-for-free-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#is_accessible_for_free_property = None;
					#[cfg(any(
						any(
							feature = "keywords-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#keywords_property = None;
					#[cfg(any(
						any(
							feature = "location-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#location_property = None;
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
							feature = "maximum-attendee-capacity-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#maximum_attendee_capacity_property = None;
					#[cfg(any(
						any(
							feature = "maximum-physical-attendee-capacity-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#maximum_physical_attendee_capacity_property = None;
					#[cfg(any(
						any(
							feature = "maximum-virtual-attendee-capacity-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#maximum_virtual_attendee_capacity_property = None;
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
							feature = "organizer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#organizer_property = None;
					#[cfg(any(
						any(
							feature = "performer-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#performer_property = None;
					#[cfg(any(
						any(
							feature = "performers-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#performers_property = None;
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
							feature = "previous-start-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#previous_start_date_property = None;
					#[cfg(any(
						any(
							feature = "recorded-in-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#recorded_in_property = None;
					#[cfg(any(
						any(
							feature = "remaining-attendee-capacity-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#remaining_attendee_capacity_property = None;
					#[cfg(any(
						any(
							feature = "review-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#review_property = None;
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
							feature = "sponsor-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#sponsor_property = None;
					#[cfg(any(
						any(
							feature = "start-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#start_date_property = None;
					#[cfg(any(
						any(
							feature = "sub-event-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#sub_event_property = None;
					#[cfg(any(
						any(
							feature = "sub-events-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#sub_events_property = None;
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
							feature = "super-event-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#super_event_property = None;
					#[cfg(any(
						any(
							feature = "translator-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#translator_property = None;
					#[cfg(any(
						any(
							feature = "typical-age-range-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#typical_age_range_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
					#[cfg(any(
						any(
							feature = "work-featured-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#work_featured_property = None;
					#[cfg(any(
						any(
							feature = "work-performed-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#work_performed_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
							#[cfg(any(
								any(
									feature = "about-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "actor-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "aggregate-rating-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "attendee-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "attendees-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "audience-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "composer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "contributor-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "director-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "door-time-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "duration-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "end-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "event-attendance-mode-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "event-schedule-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "event-status-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "funder-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "funding-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
									feature = "in-language-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "is-accessible-for-free-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "keywords-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "location-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "maximum-attendee-capacity-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "maximum-physical-attendee-capacity-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "maximum-virtual-attendee-capacity-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
									feature = "organizer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "performer-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "performers-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "previous-start-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "recorded-in-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "remaining-attendee-capacity-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "review-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "sponsor-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "start-date-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "sub-event-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "sub-events-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "super-event-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "translator-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "typical-age-range-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
									feature = "work-featured-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "work-performed-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(BusinessEvent {
						#[cfg(any(
							any(
								feature = "about-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#about: r#about_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "actor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#actor: r#actor_property.unwrap_or_default(),
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
								feature = "aggregate-rating-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#aggregate_rating: r#aggregate_rating_property.unwrap_or_default(),
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
								feature = "attendee-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#attendee: r#attendee_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "attendees-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#attendees: r#attendees_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "audience-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#audience: r#audience_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "composer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#composer: r#composer_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "contributor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#contributor: r#contributor_property.unwrap_or_default(),
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
								feature = "director-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#director: r#director_property.unwrap_or_default(),
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
								feature = "door-time-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#door_time: r#door_time_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "duration-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#duration: r#duration_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "end-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#end_date: r#end_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "event-attendance-mode-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#event_attendance_mode: r#event_attendance_mode_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "event-schedule-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#event_schedule: r#event_schedule_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "event-status-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#event_status: r#event_status_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "funder-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#funder: r#funder_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "funding-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#funding: r#funding_property.unwrap_or_default(),
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
								feature = "in-language-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#in_language: r#in_language_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "is-accessible-for-free-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#is_accessible_for_free: r#is_accessible_for_free_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "keywords-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#keywords: r#keywords_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "location-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#location: r#location_property.unwrap_or_default(),
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
								feature = "maximum-attendee-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#maximum_attendee_capacity: r#maximum_attendee_capacity_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "maximum-physical-attendee-capacity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#maximum_physical_attendee_capacity:
							r#maximum_physical_attendee_capacity_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "maximum-virtual-attendee-capacity-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#maximum_virtual_attendee_capacity:
							r#maximum_virtual_attendee_capacity_property.unwrap_or_default(),
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
								feature = "organizer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#organizer: r#organizer_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "performer-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#performer: r#performer_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "performers-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#performers: r#performers_property.unwrap_or_default(),
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
								feature = "previous-start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#previous_start_date: r#previous_start_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "recorded-in-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#recorded_in: r#recorded_in_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "remaining-attendee-capacity-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#remaining_attendee_capacity: r#remaining_attendee_capacity_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "review-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#review: r#review_property.unwrap_or_default(),
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
								feature = "sponsor-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#sponsor: r#sponsor_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#start_date: r#start_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sub-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#sub_event: r#sub_event_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "sub-events-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#sub_events: r#sub_events_property.unwrap_or_default(),
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
								feature = "super-event-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#super_event: r#super_event_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "translator-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#translator: r#translator_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "typical-age-range-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#typical_age_range: r#typical_age_range_property.unwrap_or_default(),
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
								feature = "work-featured-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#work_featured: r#work_featured_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "work-performed-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#work_performed: r#work_performed_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				#[cfg(any(
					any(feature = "about-property-schema", feature = "general-schema-section"),
					doc
				))]
				"about",
				#[cfg(any(
					any(feature = "actor-property-schema", feature = "general-schema-section"),
					doc
				))]
				"actor",
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
						feature = "aggregate-rating-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"aggregateRating",
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
						feature = "attendee-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"attendee",
				#[cfg(any(
					any(
						feature = "attendees-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"attendees",
				#[cfg(any(
					any(
						feature = "audience-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"audience",
				#[cfg(any(
					any(
						feature = "composer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"composer",
				#[cfg(any(
					any(
						feature = "contributor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"contributor",
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
						feature = "director-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"director",
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
						feature = "door-time-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"doorTime",
				#[cfg(any(
					any(
						feature = "duration-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"duration",
				#[cfg(any(
					any(
						feature = "end-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"endDate",
				#[cfg(any(
					any(
						feature = "event-attendance-mode-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"eventAttendanceMode",
				#[cfg(any(
					any(
						feature = "event-schedule-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"eventSchedule",
				#[cfg(any(
					any(
						feature = "event-status-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"eventStatus",
				#[cfg(any(
					any(feature = "funder-property-schema", feature = "general-schema-section"),
					doc
				))]
				"funder",
				#[cfg(any(
					any(
						feature = "funding-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"funding",
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
						feature = "in-language-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"inLanguage",
				#[cfg(any(
					any(
						feature = "is-accessible-for-free-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"isAccessibleForFree",
				#[cfg(any(
					any(
						feature = "keywords-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"keywords",
				#[cfg(any(
					any(
						feature = "location-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"location",
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
						feature = "maximum-attendee-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"maximumAttendeeCapacity",
				#[cfg(any(
					any(
						feature = "maximum-physical-attendee-capacity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"maximumPhysicalAttendeeCapacity",
				#[cfg(any(
					any(
						feature = "maximum-virtual-attendee-capacity-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"maximumVirtualAttendeeCapacity",
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
						feature = "organizer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"organizer",
				#[cfg(any(
					any(
						feature = "performer-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"performer",
				#[cfg(any(
					any(
						feature = "performers-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"performers",
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
						feature = "previous-start-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"previousStartDate",
				#[cfg(any(
					any(
						feature = "recorded-in-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"recordedIn",
				#[cfg(any(
					any(
						feature = "remaining-attendee-capacity-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"remainingAttendeeCapacity",
				#[cfg(any(
					any(feature = "review-property-schema", feature = "general-schema-section"),
					doc
				))]
				"review",
				#[cfg(any(
					any(
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(
						feature = "sponsor-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sponsor",
				#[cfg(any(
					any(
						feature = "start-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"startDate",
				#[cfg(any(
					any(
						feature = "sub-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subEvent",
				#[cfg(any(
					any(
						feature = "sub-events-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subEvents",
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
						feature = "super-event-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"superEvent",
				#[cfg(any(
					any(
						feature = "translator-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"translator",
				#[cfg(any(
					any(
						feature = "typical-age-range-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"typicalAgeRange",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
				#[cfg(any(
					any(
						feature = "work-featured-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"workFeatured",
				#[cfg(any(
					any(
						feature = "work-performed-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"workPerformed",
			];
			deserializer.deserialize_struct("BusinessEvent", FIELDS, ClassVisitor)
		}
	}
}
