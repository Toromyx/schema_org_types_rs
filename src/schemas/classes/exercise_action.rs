use super::*;
/// The act of participating in exertive activity for the purposes of improving health and fitness.
///
/// https://schema.org/ExerciseAction
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExerciseAction {
    #[cfg(any(
        feature = "action-status-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "actionStatus"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#action_status: Vec<ActionStatusProperty>,
    #[cfg(any(
        feature = "additional-type-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(feature = "agent-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "agent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#agent: Vec<AgentProperty>,
    #[cfg(any(
        feature = "alternate-name-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        feature = "audience-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "audience"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#audience: Vec<AudienceProperty>,
    #[cfg(any(feature = "course-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "course"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#course: Vec<CourseProperty>,
    #[cfg(any(
        feature = "description-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        feature = "diet-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "diet"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#diet: Vec<DietProperty>,
    #[cfg(any(
        feature = "disambiguating-description-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        feature = "distance-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "distance"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#distance: Vec<DistanceProperty>,
    #[cfg(any(
        feature = "end-time-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "endTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#end_time: Vec<EndTimeProperty>,
    #[cfg(any(feature = "error-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "error"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#error: Vec<ErrorProperty>,
    #[cfg(any(feature = "event-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "event"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#event: Vec<EventProperty>,
    #[cfg(any(
        feature = "exercise-course-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "exerciseCourse"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#exercise_course: Vec<ExerciseCourseProperty>,
    #[cfg(any(
        feature = "exercise-plan-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "exercisePlan"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#exercise_plan: Vec<ExercisePlanProperty>,
    #[cfg(any(
        feature = "exercise-related-diet-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "exerciseRelatedDiet"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#exercise_related_diet: Vec<ExerciseRelatedDietProperty>,
    #[cfg(any(
        feature = "exercise-type-property-schema",
        feature = "health-lifesci-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "exerciseType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#exercise_type: Vec<ExerciseTypeProperty>,
    #[cfg(any(
        feature = "from-location-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "fromLocation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#from_location: Vec<FromLocationProperty>,
    #[cfg(any(
        feature = "identifier-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(feature = "image-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        feature = "instrument-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "instrument"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#instrument: Vec<InstrumentProperty>,
    #[cfg(any(
        feature = "location-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "location"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#location: Vec<LocationProperty>,
    #[cfg(any(
        feature = "main-entity-of-page-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(feature = "name-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(feature = "object-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "object"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#object: Vec<ObjectProperty>,
    #[cfg(any(
        feature = "opponent-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "opponent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#opponent: Vec<OpponentProperty>,
    #[cfg(any(
        feature = "participant-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "participant"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#participant: Vec<ParticipantProperty>,
    #[cfg(any(
        feature = "potential-action-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        feature = "provider-property-schema",
        feature = "pending-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "provider"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#provider: Vec<ProviderProperty>,
    #[cfg(any(feature = "result-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "result"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#result: Vec<ResultProperty>,
    #[cfg(any(
        feature = "same-as-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        feature = "sports-activity-location-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sportsActivityLocation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sports_activity_location: Vec<SportsActivityLocationProperty>,
    #[cfg(any(
        feature = "sports-event-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sportsEvent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sports_event: Vec<SportsEventProperty>,
    #[cfg(any(
        feature = "sports-team-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sportsTeam"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#sports_team: Vec<SportsTeamProperty>,
    #[cfg(any(
        feature = "start-time-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "startTime"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#start_time: Vec<StartTimeProperty>,
    #[cfg(any(
        feature = "subject-of-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(feature = "target-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "target"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#target: Vec<TargetProperty>,
    #[cfg(any(
        feature = "to-location-property-schema",
        feature = "general-schema-section"
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "toLocation"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#to_location: Vec<ToLocationProperty>,
    #[cfg(any(feature = "url-property-schema", feature = "general-schema-section"))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#url: Vec<UrlProperty>,
}