use super::*;
/// <https://schema.org/WorkBasedProgram>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct WorkBasedProgram {
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
			feature = "alternate-name-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#alternate_name: Vec<AlternateNameProperty>,
	#[cfg(any(
		any(
			feature = "application-deadline-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#application_deadline: Vec<ApplicationDeadlineProperty>,
	#[cfg(any(
		any(
			feature = "application-start-date-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#application_start_date: Vec<ApplicationStartDateProperty>,
	#[cfg(any(
		any(
			feature = "day-of-week-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#day_of_week: Vec<DayOfWeekProperty>,
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
			feature = "educational-credential-awarded-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#educational_credential_awarded: Vec<EducationalCredentialAwardedProperty>,
	#[cfg(any(
		any(
			feature = "educational-program-mode-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#educational_program_mode: Vec<EducationalProgramModeProperty>,
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
			feature = "financial-aid-eligible-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#financial_aid_eligible: Vec<FinancialAidEligibleProperty>,
	#[cfg(any(
		any(
			feature = "has-course-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#has_course: Vec<HasCourseProperty>,
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
			feature = "main-entity-of-page-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	#[cfg(any(
		any(
			feature = "maximum-enrollment-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#maximum_enrollment: Vec<MaximumEnrollmentProperty>,
	#[cfg(any(
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
	#[cfg(any(
		any(
			feature = "number-of-credits-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#number_of_credits: Vec<NumberOfCreditsProperty>,
	#[cfg(any(
		any(
			feature = "occupational-category-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#occupational_category: Vec<OccupationalCategoryProperty>,
	#[cfg(any(
		any(
			feature = "occupational-credential-awarded-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#occupational_credential_awarded: Vec<OccupationalCredentialAwardedProperty>,
	#[cfg(any(
		any(feature = "offers-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#offers: Vec<OffersProperty>,
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
			feature = "program-prerequisites-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#program_prerequisites: Vec<ProgramPrerequisitesProperty>,
	#[cfg(any(
		any(
			feature = "program-type-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#program_type: Vec<ProgramTypeProperty>,
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
			feature = "salary-upon-completion-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#salary_upon_completion: Vec<SalaryUponCompletionProperty>,
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
			feature = "start-date-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#start_date: Vec<StartDateProperty>,
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
			feature = "term-duration-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#term_duration: Vec<TermDurationProperty>,
	#[cfg(any(
		any(
			feature = "terms-per-year-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#terms_per_year: Vec<TermsPerYearProperty>,
	#[cfg(any(
		any(
			feature = "time-of-day-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#time_of_day: Vec<TimeOfDayProperty>,
	#[cfg(any(
		any(
			feature = "time-to-complete-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#time_to_complete: Vec<TimeToCompleteProperty>,
	#[cfg(any(
		any(
			feature = "training-salary-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#training_salary: Vec<TrainingSalaryProperty>,
	#[cfg(any(
		any(
			feature = "typical-credits-per-term-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#typical_credits_per_term: Vec<TypicalCreditsPerTermProperty>,
	#[cfg(any(
		any(feature = "url-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#url: Vec<UrlProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for WorkBasedProgram {
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
						feature = "application-deadline-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#application_deadline) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "application-start-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#application_start_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "day-of-week-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#day_of_week) as usize
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
						feature = "educational-credential-awarded-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#educational_credential_awarded) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "educational-program-mode-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#educational_program_mode) as usize
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
						feature = "financial-aid-eligible-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#financial_aid_eligible) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "has-course-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#has_course) as usize
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
						feature = "maximum-enrollment-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#maximum_enrollment) as usize
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
						feature = "number-of-credits-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#number_of_credits) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "occupational-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#occupational_category) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "occupational-credential-awarded-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#occupational_credential_awarded) as usize
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
						feature = "program-prerequisites-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#program_prerequisites) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "program-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#program_type) as usize
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
						feature = "salary-upon-completion-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#salary_upon_completion) as usize
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
						feature = "term-duration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#term_duration) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "terms-per-year-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#terms_per_year) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "time-of-day-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#time_of_day) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "time-to-complete-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#time_to_complete) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "training-salary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#training_salary) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "typical-credits-per-term-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#typical_credits_per_term) as usize
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
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "WorkBasedProgram", len)?;
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
					feature = "application-deadline-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#application_deadline) {
				serialize_struct.serialize_field("applicationDeadline", {
					struct SerializeWith<'a>(&'a Vec<ApplicationDeadlineProperty>);
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
					&SerializeWith(&self.r#application_deadline)
				})?;
			} else {
				serialize_struct.skip_field("applicationDeadline")?;
			}
			#[cfg(any(
				any(
					feature = "application-start-date-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#application_start_date) {
				serialize_struct.serialize_field("applicationStartDate", {
					struct SerializeWith<'a>(&'a Vec<ApplicationStartDateProperty>);
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
					&SerializeWith(&self.r#application_start_date)
				})?;
			} else {
				serialize_struct.skip_field("applicationStartDate")?;
			}
			#[cfg(any(
				any(
					feature = "day-of-week-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#day_of_week) {
				serialize_struct.serialize_field("dayOfWeek", {
					struct SerializeWith<'a>(&'a Vec<DayOfWeekProperty>);
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
					&SerializeWith(&self.r#day_of_week)
				})?;
			} else {
				serialize_struct.skip_field("dayOfWeek")?;
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
					feature = "educational-credential-awarded-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#educational_credential_awarded) {
				serialize_struct.serialize_field("educationalCredentialAwarded", {
					struct SerializeWith<'a>(&'a Vec<EducationalCredentialAwardedProperty>);
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
					&SerializeWith(&self.r#educational_credential_awarded)
				})?;
			} else {
				serialize_struct.skip_field("educationalCredentialAwarded")?;
			}
			#[cfg(any(
				any(
					feature = "educational-program-mode-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#educational_program_mode) {
				serialize_struct.serialize_field("educationalProgramMode", {
					struct SerializeWith<'a>(&'a Vec<EducationalProgramModeProperty>);
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
					&SerializeWith(&self.r#educational_program_mode)
				})?;
			} else {
				serialize_struct.skip_field("educationalProgramMode")?;
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
					feature = "financial-aid-eligible-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#financial_aid_eligible) {
				serialize_struct.serialize_field("financialAidEligible", {
					struct SerializeWith<'a>(&'a Vec<FinancialAidEligibleProperty>);
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
					&SerializeWith(&self.r#financial_aid_eligible)
				})?;
			} else {
				serialize_struct.skip_field("financialAidEligible")?;
			}
			#[cfg(any(
				any(
					feature = "has-course-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#has_course) {
				serialize_struct.serialize_field("hasCourse", {
					struct SerializeWith<'a>(&'a Vec<HasCourseProperty>);
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
					&SerializeWith(&self.r#has_course)
				})?;
			} else {
				serialize_struct.skip_field("hasCourse")?;
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
					feature = "maximum-enrollment-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#maximum_enrollment) {
				serialize_struct.serialize_field("maximumEnrollment", {
					struct SerializeWith<'a>(&'a Vec<MaximumEnrollmentProperty>);
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
					&SerializeWith(&self.r#maximum_enrollment)
				})?;
			} else {
				serialize_struct.skip_field("maximumEnrollment")?;
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
				any(
					feature = "number-of-credits-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#number_of_credits) {
				serialize_struct.serialize_field("numberOfCredits", {
					struct SerializeWith<'a>(&'a Vec<NumberOfCreditsProperty>);
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
					&SerializeWith(&self.r#number_of_credits)
				})?;
			} else {
				serialize_struct.skip_field("numberOfCredits")?;
			}
			#[cfg(any(
				any(
					feature = "occupational-category-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#occupational_category) {
				serialize_struct.serialize_field("occupationalCategory", {
					struct SerializeWith<'a>(&'a Vec<OccupationalCategoryProperty>);
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
					&SerializeWith(&self.r#occupational_category)
				})?;
			} else {
				serialize_struct.skip_field("occupationalCategory")?;
			}
			#[cfg(any(
				any(
					feature = "occupational-credential-awarded-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#occupational_credential_awarded) {
				serialize_struct.serialize_field("occupationalCredentialAwarded", {
					struct SerializeWith<'a>(&'a Vec<OccupationalCredentialAwardedProperty>);
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
					&SerializeWith(&self.r#occupational_credential_awarded)
				})?;
			} else {
				serialize_struct.skip_field("occupationalCredentialAwarded")?;
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
					feature = "program-prerequisites-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#program_prerequisites) {
				serialize_struct.serialize_field("programPrerequisites", {
					struct SerializeWith<'a>(&'a Vec<ProgramPrerequisitesProperty>);
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
					&SerializeWith(&self.r#program_prerequisites)
				})?;
			} else {
				serialize_struct.skip_field("programPrerequisites")?;
			}
			#[cfg(any(
				any(
					feature = "program-type-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#program_type) {
				serialize_struct.serialize_field("programType", {
					struct SerializeWith<'a>(&'a Vec<ProgramTypeProperty>);
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
					&SerializeWith(&self.r#program_type)
				})?;
			} else {
				serialize_struct.skip_field("programType")?;
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
					feature = "salary-upon-completion-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#salary_upon_completion) {
				serialize_struct.serialize_field("salaryUponCompletion", {
					struct SerializeWith<'a>(&'a Vec<SalaryUponCompletionProperty>);
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
					&SerializeWith(&self.r#salary_upon_completion)
				})?;
			} else {
				serialize_struct.skip_field("salaryUponCompletion")?;
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
					feature = "term-duration-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#term_duration) {
				serialize_struct.serialize_field("termDuration", {
					struct SerializeWith<'a>(&'a Vec<TermDurationProperty>);
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
					&SerializeWith(&self.r#term_duration)
				})?;
			} else {
				serialize_struct.skip_field("termDuration")?;
			}
			#[cfg(any(
				any(
					feature = "terms-per-year-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#terms_per_year) {
				serialize_struct.serialize_field("termsPerYear", {
					struct SerializeWith<'a>(&'a Vec<TermsPerYearProperty>);
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
					&SerializeWith(&self.r#terms_per_year)
				})?;
			} else {
				serialize_struct.skip_field("termsPerYear")?;
			}
			#[cfg(any(
				any(
					feature = "time-of-day-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#time_of_day) {
				serialize_struct.serialize_field("timeOfDay", {
					struct SerializeWith<'a>(&'a Vec<TimeOfDayProperty>);
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
					&SerializeWith(&self.r#time_of_day)
				})?;
			} else {
				serialize_struct.skip_field("timeOfDay")?;
			}
			#[cfg(any(
				any(
					feature = "time-to-complete-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#time_to_complete) {
				serialize_struct.serialize_field("timeToComplete", {
					struct SerializeWith<'a>(&'a Vec<TimeToCompleteProperty>);
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
					&SerializeWith(&self.r#time_to_complete)
				})?;
			} else {
				serialize_struct.skip_field("timeToComplete")?;
			}
			#[cfg(any(
				any(
					feature = "training-salary-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#training_salary) {
				serialize_struct.serialize_field("trainingSalary", {
					struct SerializeWith<'a>(&'a Vec<TrainingSalaryProperty>);
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
					&SerializeWith(&self.r#training_salary)
				})?;
			} else {
				serialize_struct.skip_field("trainingSalary")?;
			}
			#[cfg(any(
				any(
					feature = "typical-credits-per-term-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
			if !Vec::is_empty(&self.r#typical_credits_per_term) {
				serialize_struct.serialize_field("typicalCreditsPerTerm", {
					struct SerializeWith<'a>(&'a Vec<TypicalCreditsPerTermProperty>);
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
					&SerializeWith(&self.r#typical_credits_per_term)
				})?;
			} else {
				serialize_struct.skip_field("typicalCreditsPerTerm")?;
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
			serialize_struct.end()
		}
	}
	impl<'de> Deserialize<'de> for WorkBasedProgram {
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
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				AlternateName,
				#[cfg(any(
					any(
						feature = "application-deadline-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ApplicationDeadline,
				#[cfg(any(
					any(
						feature = "application-start-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ApplicationStartDate,
				#[cfg(any(
					any(
						feature = "day-of-week-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DayOfWeek,
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
						feature = "educational-credential-awarded-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				EducationalCredentialAwarded,
				#[cfg(any(
					any(
						feature = "educational-program-mode-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				EducationalProgramMode,
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
						feature = "financial-aid-eligible-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				FinancialAidEligible,
				#[cfg(any(
					any(
						feature = "has-course-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				HasCourse,
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
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				MainEntityOfPage,
				#[cfg(any(
					any(
						feature = "maximum-enrollment-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				MaximumEnrollment,
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
				#[cfg(any(
					any(
						feature = "number-of-credits-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				NumberOfCredits,
				#[cfg(any(
					any(
						feature = "occupational-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				OccupationalCategory,
				#[cfg(any(
					any(
						feature = "occupational-credential-awarded-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				OccupationalCredentialAwarded,
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				Offers,
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
						feature = "program-prerequisites-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ProgramPrerequisites,
				#[cfg(any(
					any(
						feature = "program-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				ProgramType,
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
						feature = "salary-upon-completion-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				SalaryUponCompletion,
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
						feature = "start-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				StartDate,
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
						feature = "term-duration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TermDuration,
				#[cfg(any(
					any(
						feature = "terms-per-year-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TermsPerYear,
				#[cfg(any(
					any(
						feature = "time-of-day-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TimeOfDay,
				#[cfg(any(
					any(
						feature = "time-to-complete-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TimeToComplete,
				#[cfg(any(
					any(
						feature = "training-salary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TrainingSalary,
				#[cfg(any(
					any(
						feature = "typical-credits-per-term-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				TypicalCreditsPerTerm,
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
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
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "application-deadline-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"applicationDeadline" => Ok(Field::ApplicationDeadline),
						#[cfg(any(
							any(
								feature = "application-start-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"applicationStartDate" => Ok(Field::ApplicationStartDate),
						#[cfg(any(
							any(
								feature = "day-of-week-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"dayOfWeek" => Ok(Field::DayOfWeek),
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
								feature = "educational-credential-awarded-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"educationalCredentialAwarded" => Ok(Field::EducationalCredentialAwarded),
						#[cfg(any(
							any(
								feature = "educational-program-mode-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"educationalProgramMode" => Ok(Field::EducationalProgramMode),
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
								feature = "financial-aid-eligible-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"financialAidEligible" => Ok(Field::FinancialAidEligible),
						#[cfg(any(
							any(
								feature = "has-course-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"hasCourse" => Ok(Field::HasCourse),
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
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "maximum-enrollment-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"maximumEnrollment" => Ok(Field::MaximumEnrollment),
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
								feature = "number-of-credits-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"numberOfCredits" => Ok(Field::NumberOfCredits),
						#[cfg(any(
							any(
								feature = "occupational-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"occupationalCategory" => Ok(Field::OccupationalCategory),
						#[cfg(any(
							any(
								feature = "occupational-credential-awarded-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"occupationalCredentialAwarded" => Ok(Field::OccupationalCredentialAwarded),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "program-prerequisites-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"programPrerequisites" => Ok(Field::ProgramPrerequisites),
						#[cfg(any(
							any(
								feature = "program-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"programType" => Ok(Field::ProgramType),
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
								feature = "salary-upon-completion-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"salaryUponCompletion" => Ok(Field::SalaryUponCompletion),
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
								feature = "start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"startDate" => Ok(Field::StartDate),
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
								feature = "term-duration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"termDuration" => Ok(Field::TermDuration),
						#[cfg(any(
							any(
								feature = "terms-per-year-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"termsPerYear" => Ok(Field::TermsPerYear),
						#[cfg(any(
							any(
								feature = "time-of-day-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"timeOfDay" => Ok(Field::TimeOfDay),
						#[cfg(any(
							any(
								feature = "time-to-complete-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"timeToComplete" => Ok(Field::TimeToComplete),
						#[cfg(any(
							any(
								feature = "training-salary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"trainingSalary" => Ok(Field::TrainingSalary),
						#[cfg(any(
							any(
								feature = "typical-credits-per-term-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"typicalCreditsPerTerm" => Ok(Field::TypicalCreditsPerTerm),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"url" => Ok(Field::Url),
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
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"alternateName" => Ok(Field::AlternateName),
						#[cfg(any(
							any(
								feature = "application-deadline-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"applicationDeadline" => Ok(Field::ApplicationDeadline),
						#[cfg(any(
							any(
								feature = "application-start-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"applicationStartDate" => Ok(Field::ApplicationStartDate),
						#[cfg(any(
							any(
								feature = "day-of-week-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"dayOfWeek" => Ok(Field::DayOfWeek),
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
								feature = "educational-credential-awarded-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"educationalCredentialAwarded" => Ok(Field::EducationalCredentialAwarded),
						#[cfg(any(
							any(
								feature = "educational-program-mode-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"educationalProgramMode" => Ok(Field::EducationalProgramMode),
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
								feature = "financial-aid-eligible-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"financialAidEligible" => Ok(Field::FinancialAidEligible),
						#[cfg(any(
							any(
								feature = "has-course-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"hasCourse" => Ok(Field::HasCourse),
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
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						#[cfg(any(
							any(
								feature = "maximum-enrollment-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"maximumEnrollment" => Ok(Field::MaximumEnrollment),
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
								feature = "number-of-credits-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"numberOfCredits" => Ok(Field::NumberOfCredits),
						#[cfg(any(
							any(
								feature = "occupational-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"occupationalCategory" => Ok(Field::OccupationalCategory),
						#[cfg(any(
							any(
								feature = "occupational-credential-awarded-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"occupationalCredentialAwarded" => Ok(Field::OccupationalCredentialAwarded),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"potentialAction" => Ok(Field::PotentialAction),
						#[cfg(any(
							any(
								feature = "program-prerequisites-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"programPrerequisites" => Ok(Field::ProgramPrerequisites),
						#[cfg(any(
							any(
								feature = "program-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"programType" => Ok(Field::ProgramType),
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
								feature = "salary-upon-completion-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"salaryUponCompletion" => Ok(Field::SalaryUponCompletion),
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
								feature = "start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"startDate" => Ok(Field::StartDate),
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
								feature = "term-duration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"termDuration" => Ok(Field::TermDuration),
						#[cfg(any(
							any(
								feature = "terms-per-year-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"termsPerYear" => Ok(Field::TermsPerYear),
						#[cfg(any(
							any(
								feature = "time-of-day-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"timeOfDay" => Ok(Field::TimeOfDay),
						#[cfg(any(
							any(
								feature = "time-to-complete-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"timeToComplete" => Ok(Field::TimeToComplete),
						#[cfg(any(
							any(
								feature = "training-salary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"trainingSalary" => Ok(Field::TrainingSalary),
						#[cfg(any(
							any(
								feature = "typical-credits-per-term-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"typicalCreditsPerTerm" => Ok(Field::TypicalCreditsPerTerm),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
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
				type Value = WorkBasedProgram;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema WorkBasedProgram")
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
							feature = "alternate-name-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#alternate_name_property = None;
					#[cfg(any(
						any(
							feature = "application-deadline-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#application_deadline_property = None;
					#[cfg(any(
						any(
							feature = "application-start-date-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#application_start_date_property = None;
					#[cfg(any(
						any(
							feature = "day-of-week-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#day_of_week_property = None;
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
							feature = "educational-credential-awarded-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#educational_credential_awarded_property = None;
					#[cfg(any(
						any(
							feature = "educational-program-mode-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#educational_program_mode_property = None;
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
							feature = "financial-aid-eligible-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#financial_aid_eligible_property = None;
					#[cfg(any(
						any(
							feature = "has-course-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#has_course_property = None;
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
							feature = "main-entity-of-page-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#main_entity_of_page_property = None;
					#[cfg(any(
						any(
							feature = "maximum-enrollment-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#maximum_enrollment_property = None;
					#[cfg(any(
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
					#[cfg(any(
						any(
							feature = "number-of-credits-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#number_of_credits_property = None;
					#[cfg(any(
						any(
							feature = "occupational-category-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#occupational_category_property = None;
					#[cfg(any(
						any(
							feature = "occupational-credential-awarded-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#occupational_credential_awarded_property = None;
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
							feature = "potential-action-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#potential_action_property = None;
					#[cfg(any(
						any(
							feature = "program-prerequisites-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#program_prerequisites_property = None;
					#[cfg(any(
						any(
							feature = "program-type-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#program_type_property = None;
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
							feature = "salary-upon-completion-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#salary_upon_completion_property = None;
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
							feature = "start-date-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#start_date_property = None;
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
							feature = "term-duration-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#term_duration_property = None;
					#[cfg(any(
						any(
							feature = "terms-per-year-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#terms_per_year_property = None;
					#[cfg(any(
						any(
							feature = "time-of-day-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#time_of_day_property = None;
					#[cfg(any(
						any(
							feature = "time-to-complete-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#time_to_complete_property = None;
					#[cfg(any(
						any(
							feature = "training-salary-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#training_salary_property = None;
					#[cfg(any(
						any(
							feature = "typical-credits-per-term-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#typical_credits_per_term_property = None;
					#[cfg(any(
						any(feature = "url-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#url_property = None;
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
									feature = "application-deadline-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ApplicationDeadline => {
								if r#application_deadline_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"applicationDeadline",
									));
								}
								r#application_deadline_property = Some({
									struct DeserializeWith(Vec<ApplicationDeadlineProperty>);
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
									feature = "application-start-date-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ApplicationStartDate => {
								if r#application_start_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"applicationStartDate",
									));
								}
								r#application_start_date_property = Some({
									struct DeserializeWith(Vec<ApplicationStartDateProperty>);
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
									feature = "day-of-week-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::DayOfWeek => {
								if r#day_of_week_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"dayOfWeek",
									));
								}
								r#day_of_week_property = Some({
									struct DeserializeWith(Vec<DayOfWeekProperty>);
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
									feature = "educational-credential-awarded-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
							Field::EducationalCredentialAwarded => {
								if r#educational_credential_awarded_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"educationalCredentialAwarded",
									));
								}
								r#educational_credential_awarded_property = Some({
									struct DeserializeWith(
										Vec<EducationalCredentialAwardedProperty>,
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
									feature = "educational-program-mode-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::EducationalProgramMode => {
								if r#educational_program_mode_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"educationalProgramMode",
									));
								}
								r#educational_program_mode_property = Some({
									struct DeserializeWith(Vec<EducationalProgramModeProperty>);
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
									feature = "financial-aid-eligible-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::FinancialAidEligible => {
								if r#financial_aid_eligible_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"financialAidEligible",
									));
								}
								r#financial_aid_eligible_property = Some({
									struct DeserializeWith(Vec<FinancialAidEligibleProperty>);
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
									feature = "has-course-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::HasCourse => {
								if r#has_course_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"hasCourse",
									));
								}
								r#has_course_property = Some({
									struct DeserializeWith(Vec<HasCourseProperty>);
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
									feature = "maximum-enrollment-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::MaximumEnrollment => {
								if r#maximum_enrollment_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"maximumEnrollment",
									));
								}
								r#maximum_enrollment_property = Some({
									struct DeserializeWith(Vec<MaximumEnrollmentProperty>);
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
									feature = "number-of-credits-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::NumberOfCredits => {
								if r#number_of_credits_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"numberOfCredits",
									));
								}
								r#number_of_credits_property = Some({
									struct DeserializeWith(Vec<NumberOfCreditsProperty>);
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
									feature = "occupational-category-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::OccupationalCategory => {
								if r#occupational_category_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"occupationalCategory",
									));
								}
								r#occupational_category_property = Some({
									struct DeserializeWith(Vec<OccupationalCategoryProperty>);
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
									feature = "occupational-credential-awarded-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::OccupationalCredentialAwarded => {
								if r#occupational_credential_awarded_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"occupationalCredentialAwarded",
									));
								}
								r#occupational_credential_awarded_property = Some({
									struct DeserializeWith(
										Vec<OccupationalCredentialAwardedProperty>,
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
									feature = "program-prerequisites-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ProgramPrerequisites => {
								if r#program_prerequisites_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"programPrerequisites",
									));
								}
								r#program_prerequisites_property = Some({
									struct DeserializeWith(Vec<ProgramPrerequisitesProperty>);
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
									feature = "program-type-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::ProgramType => {
								if r#program_type_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"programType",
									));
								}
								r#program_type_property = Some({
									struct DeserializeWith(Vec<ProgramTypeProperty>);
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
									feature = "salary-upon-completion-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::SalaryUponCompletion => {
								if r#salary_upon_completion_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"salaryUponCompletion",
									));
								}
								r#salary_upon_completion_property = Some({
									struct DeserializeWith(Vec<SalaryUponCompletionProperty>);
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
									feature = "term-duration-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::TermDuration => {
								if r#term_duration_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"termDuration",
									));
								}
								r#term_duration_property = Some({
									struct DeserializeWith(Vec<TermDurationProperty>);
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
									feature = "terms-per-year-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::TermsPerYear => {
								if r#terms_per_year_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"termsPerYear",
									));
								}
								r#terms_per_year_property = Some({
									struct DeserializeWith(Vec<TermsPerYearProperty>);
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
									feature = "time-of-day-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::TimeOfDay => {
								if r#time_of_day_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"timeOfDay",
									));
								}
								r#time_of_day_property = Some({
									struct DeserializeWith(Vec<TimeOfDayProperty>);
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
									feature = "time-to-complete-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::TimeToComplete => {
								if r#time_to_complete_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"timeToComplete",
									));
								}
								r#time_to_complete_property = Some({
									struct DeserializeWith(Vec<TimeToCompleteProperty>);
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
									feature = "training-salary-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::TrainingSalary => {
								if r#training_salary_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"trainingSalary",
									));
								}
								r#training_salary_property = Some({
									struct DeserializeWith(Vec<TrainingSalaryProperty>);
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
									feature = "typical-credits-per-term-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
							Field::TypicalCreditsPerTerm => {
								if r#typical_credits_per_term_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"typicalCreditsPerTerm",
									));
								}
								r#typical_credits_per_term_property = Some({
									struct DeserializeWith(Vec<TypicalCreditsPerTermProperty>);
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
							_ => {
								let _ = map.next_value::<de::IgnoredAny>()?;
							}
						}
					}
					Ok(WorkBasedProgram {
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
								feature = "alternate-name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "application-deadline-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#application_deadline: r#application_deadline_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "application-start-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#application_start_date: r#application_start_date_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "day-of-week-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#day_of_week: r#day_of_week_property.unwrap_or_default(),
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
								feature = "educational-credential-awarded-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#educational_credential_awarded: r#educational_credential_awarded_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "educational-program-mode-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#educational_program_mode: r#educational_program_mode_property
							.unwrap_or_default(),
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
								feature = "financial-aid-eligible-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#financial_aid_eligible: r#financial_aid_eligible_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "has-course-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#has_course: r#has_course_property.unwrap_or_default(),
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
								feature = "main-entity-of-page-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "maximum-enrollment-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#maximum_enrollment: r#maximum_enrollment_property.unwrap_or_default(),
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
								feature = "number-of-credits-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#number_of_credits: r#number_of_credits_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "occupational-category-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#occupational_category: r#occupational_category_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "occupational-credential-awarded-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#occupational_credential_awarded:
							r#occupational_credential_awarded_property.unwrap_or_default(),
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
								feature = "potential-action-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "program-prerequisites-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#program_prerequisites: r#program_prerequisites_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "program-type-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#program_type: r#program_type_property.unwrap_or_default(),
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
								feature = "salary-upon-completion-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#salary_upon_completion: r#salary_upon_completion_property
							.unwrap_or_default(),
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
								feature = "start-date-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#start_date: r#start_date_property.unwrap_or_default(),
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
								feature = "term-duration-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#term_duration: r#term_duration_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "terms-per-year-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#terms_per_year: r#terms_per_year_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "time-of-day-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#time_of_day: r#time_of_day_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "time-to-complete-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#time_to_complete: r#time_to_complete_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "training-salary-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#training_salary: r#training_salary_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "typical-credits-per-term-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#typical_credits_per_term: r#typical_credits_per_term_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "url-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#url: r#url_property.unwrap_or_default(),
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
						feature = "alternate-name-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"alternateName",
				#[cfg(any(
					any(
						feature = "application-deadline-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"applicationDeadline",
				#[cfg(any(
					any(
						feature = "application-start-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"applicationStartDate",
				#[cfg(any(
					any(
						feature = "day-of-week-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"dayOfWeek",
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
						feature = "educational-credential-awarded-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"educationalCredentialAwarded",
				#[cfg(any(
					any(
						feature = "educational-program-mode-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"educationalProgramMode",
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
						feature = "financial-aid-eligible-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"financialAidEligible",
				#[cfg(any(
					any(
						feature = "has-course-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"hasCourse",
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
						feature = "main-entity-of-page-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"mainEntityOfPage",
				#[cfg(any(
					any(
						feature = "maximum-enrollment-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"maximumEnrollment",
				#[cfg(any(
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
				#[cfg(any(
					any(
						feature = "number-of-credits-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"numberOfCredits",
				#[cfg(any(
					any(
						feature = "occupational-category-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"occupationalCategory",
				#[cfg(any(
					any(
						feature = "occupational-credential-awarded-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"occupationalCredentialAwarded",
				#[cfg(any(
					any(feature = "offers-property-schema", feature = "general-schema-section"),
					doc
				))]
				"offers",
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
						feature = "program-prerequisites-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"programPrerequisites",
				#[cfg(any(
					any(
						feature = "program-type-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"programType",
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
						feature = "salary-upon-completion-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"salaryUponCompletion",
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
						feature = "start-date-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"startDate",
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
						feature = "term-duration-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"termDuration",
				#[cfg(any(
					any(
						feature = "terms-per-year-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"termsPerYear",
				#[cfg(any(
					any(
						feature = "time-of-day-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"timeOfDay",
				#[cfg(any(
					any(
						feature = "time-to-complete-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"timeToComplete",
				#[cfg(any(
					any(
						feature = "training-salary-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"trainingSalary",
				#[cfg(any(
					any(
						feature = "typical-credits-per-term-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"typicalCreditsPerTerm",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
			];
			deserializer.deserialize_struct("WorkBasedProgram", FIELDS, ClassVisitor)
		}
	}
}
