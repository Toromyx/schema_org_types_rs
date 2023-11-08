use super::*;
/// <https://schema.org/EducationalOccupationalProgram>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct EducationalOccupationalProgram {
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#application_deadline: Vec<ApplicationDeadlineProperty>,
	pub r#application_start_date: Vec<ApplicationStartDateProperty>,
	pub r#day_of_week: Vec<DayOfWeekProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#educational_credential_awarded: Vec<EducationalCredentialAwardedProperty>,
	pub r#educational_program_mode: Vec<EducationalProgramModeProperty>,
	pub r#end_date: Vec<EndDateProperty>,
	pub r#financial_aid_eligible: Vec<FinancialAidEligibleProperty>,
	pub r#has_course: Vec<HasCourseProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#maximum_enrollment: Vec<MaximumEnrollmentProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#number_of_credits: Vec<NumberOfCreditsProperty>,
	pub r#occupational_category: Vec<OccupationalCategoryProperty>,
	pub r#occupational_credential_awarded: Vec<OccupationalCredentialAwardedProperty>,
	pub r#offers: Vec<OffersProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#program_prerequisites: Vec<ProgramPrerequisitesProperty>,
	pub r#program_type: Vec<ProgramTypeProperty>,
	pub r#provider: Vec<ProviderProperty>,
	pub r#salary_upon_completion: Vec<SalaryUponCompletionProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#start_date: Vec<StartDateProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#term_duration: Vec<TermDurationProperty>,
	pub r#terms_per_year: Vec<TermsPerYearProperty>,
	pub r#time_of_day: Vec<TimeOfDayProperty>,
	pub r#time_to_complete: Vec<TimeToCompleteProperty>,
	pub r#training_salary: Vec<TrainingSalaryProperty>,
	pub r#typical_credits_per_term: Vec<TypicalCreditsPerTermProperty>,
	pub r#url: Vec<UrlProperty>,
}
#[cfg(feature = "serde")]
mod serde {
	use std::{fmt, fmt::Formatter};

	use ::serde::{
		de, de::Visitor, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
	};

	use super::*;
	impl Serialize for EducationalOccupationalProgram {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
				!Vec::is_empty(&self.r#application_deadline) as usize,
				!Vec::is_empty(&self.r#application_start_date) as usize,
				!Vec::is_empty(&self.r#day_of_week) as usize,
				!Vec::is_empty(&self.r#description) as usize,
				!Vec::is_empty(&self.r#disambiguating_description) as usize,
				!Vec::is_empty(&self.r#educational_credential_awarded) as usize,
				!Vec::is_empty(&self.r#educational_program_mode) as usize,
				!Vec::is_empty(&self.r#end_date) as usize,
				!Vec::is_empty(&self.r#financial_aid_eligible) as usize,
				!Vec::is_empty(&self.r#has_course) as usize,
				!Vec::is_empty(&self.r#identifier) as usize,
				!Vec::is_empty(&self.r#image) as usize,
				!Vec::is_empty(&self.r#main_entity_of_page) as usize,
				!Vec::is_empty(&self.r#maximum_enrollment) as usize,
				!Vec::is_empty(&self.r#name) as usize,
				!Vec::is_empty(&self.r#number_of_credits) as usize,
				!Vec::is_empty(&self.r#occupational_category) as usize,
				!Vec::is_empty(&self.r#occupational_credential_awarded) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#potential_action) as usize,
				!Vec::is_empty(&self.r#program_prerequisites) as usize,
				!Vec::is_empty(&self.r#program_type) as usize,
				!Vec::is_empty(&self.r#provider) as usize,
				!Vec::is_empty(&self.r#salary_upon_completion) as usize,
				!Vec::is_empty(&self.r#same_as) as usize,
				!Vec::is_empty(&self.r#start_date) as usize,
				!Vec::is_empty(&self.r#subject_of) as usize,
				!Vec::is_empty(&self.r#term_duration) as usize,
				!Vec::is_empty(&self.r#terms_per_year) as usize,
				!Vec::is_empty(&self.r#time_of_day) as usize,
				!Vec::is_empty(&self.r#time_to_complete) as usize,
				!Vec::is_empty(&self.r#training_salary) as usize,
				!Vec::is_empty(&self.r#typical_credits_per_term) as usize,
				!Vec::is_empty(&self.r#url) as usize,
			]
			.iter()
			.sum();
			let mut serialize_struct =
				Serializer::serialize_struct(serializer, "EducationalOccupationalProgram", len)?;
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
	impl<'de> Deserialize<'de> for EducationalOccupationalProgram {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				AdditionalType,
				AlternateName,
				ApplicationDeadline,
				ApplicationStartDate,
				DayOfWeek,
				Description,
				DisambiguatingDescription,
				EducationalCredentialAwarded,
				EducationalProgramMode,
				EndDate,
				FinancialAidEligible,
				HasCourse,
				Identifier,
				Image,
				MainEntityOfPage,
				MaximumEnrollment,
				Name,
				NumberOfCredits,
				OccupationalCategory,
				OccupationalCredentialAwarded,
				Offers,
				PotentialAction,
				ProgramPrerequisites,
				ProgramType,
				Provider,
				SalaryUponCompletion,
				SameAs,
				StartDate,
				SubjectOf,
				TermDuration,
				TermsPerYear,
				TimeOfDay,
				TimeToComplete,
				TrainingSalary,
				TypicalCreditsPerTerm,
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
						"additionalType" => Ok(Field::AdditionalType),
						"alternateName" => Ok(Field::AlternateName),
						"applicationDeadline" => Ok(Field::ApplicationDeadline),
						"applicationStartDate" => Ok(Field::ApplicationStartDate),
						"dayOfWeek" => Ok(Field::DayOfWeek),
						"description" => Ok(Field::Description),
						"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						"educationalCredentialAwarded" => Ok(Field::EducationalCredentialAwarded),
						"educationalProgramMode" => Ok(Field::EducationalProgramMode),
						"endDate" => Ok(Field::EndDate),
						"financialAidEligible" => Ok(Field::FinancialAidEligible),
						"hasCourse" => Ok(Field::HasCourse),
						"identifier" => Ok(Field::Identifier),
						"image" => Ok(Field::Image),
						"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						"maximumEnrollment" => Ok(Field::MaximumEnrollment),
						"name" => Ok(Field::Name),
						"numberOfCredits" => Ok(Field::NumberOfCredits),
						"occupationalCategory" => Ok(Field::OccupationalCategory),
						"occupationalCredentialAwarded" => Ok(Field::OccupationalCredentialAwarded),
						"offers" => Ok(Field::Offers),
						"potentialAction" => Ok(Field::PotentialAction),
						"programPrerequisites" => Ok(Field::ProgramPrerequisites),
						"programType" => Ok(Field::ProgramType),
						"provider" => Ok(Field::Provider),
						"salaryUponCompletion" => Ok(Field::SalaryUponCompletion),
						"sameAs" => Ok(Field::SameAs),
						"startDate" => Ok(Field::StartDate),
						"subjectOf" => Ok(Field::SubjectOf),
						"termDuration" => Ok(Field::TermDuration),
						"termsPerYear" => Ok(Field::TermsPerYear),
						"timeOfDay" => Ok(Field::TimeOfDay),
						"timeToComplete" => Ok(Field::TimeToComplete),
						"trainingSalary" => Ok(Field::TrainingSalary),
						"typicalCreditsPerTerm" => Ok(Field::TypicalCreditsPerTerm),
						"url" => Ok(Field::Url),
						_ => Ok(Field::Ignore),
					}
				}
				fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
				where
					E: de::Error,
				{
					match value {
						b"additionalType" => Ok(Field::AdditionalType),
						b"alternateName" => Ok(Field::AlternateName),
						b"applicationDeadline" => Ok(Field::ApplicationDeadline),
						b"applicationStartDate" => Ok(Field::ApplicationStartDate),
						b"dayOfWeek" => Ok(Field::DayOfWeek),
						b"description" => Ok(Field::Description),
						b"disambiguatingDescription" => Ok(Field::DisambiguatingDescription),
						b"educationalCredentialAwarded" => Ok(Field::EducationalCredentialAwarded),
						b"educationalProgramMode" => Ok(Field::EducationalProgramMode),
						b"endDate" => Ok(Field::EndDate),
						b"financialAidEligible" => Ok(Field::FinancialAidEligible),
						b"hasCourse" => Ok(Field::HasCourse),
						b"identifier" => Ok(Field::Identifier),
						b"image" => Ok(Field::Image),
						b"mainEntityOfPage" => Ok(Field::MainEntityOfPage),
						b"maximumEnrollment" => Ok(Field::MaximumEnrollment),
						b"name" => Ok(Field::Name),
						b"numberOfCredits" => Ok(Field::NumberOfCredits),
						b"occupationalCategory" => Ok(Field::OccupationalCategory),
						b"occupationalCredentialAwarded" => {
							Ok(Field::OccupationalCredentialAwarded)
						}
						b"offers" => Ok(Field::Offers),
						b"potentialAction" => Ok(Field::PotentialAction),
						b"programPrerequisites" => Ok(Field::ProgramPrerequisites),
						b"programType" => Ok(Field::ProgramType),
						b"provider" => Ok(Field::Provider),
						b"salaryUponCompletion" => Ok(Field::SalaryUponCompletion),
						b"sameAs" => Ok(Field::SameAs),
						b"startDate" => Ok(Field::StartDate),
						b"subjectOf" => Ok(Field::SubjectOf),
						b"termDuration" => Ok(Field::TermDuration),
						b"termsPerYear" => Ok(Field::TermsPerYear),
						b"timeOfDay" => Ok(Field::TimeOfDay),
						b"timeToComplete" => Ok(Field::TimeToComplete),
						b"trainingSalary" => Ok(Field::TrainingSalary),
						b"typicalCreditsPerTerm" => Ok(Field::TypicalCreditsPerTerm),
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
				type Value = EducationalOccupationalProgram;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema EducationalOccupationalProgram")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#additional_type_property = None;
					let mut r#alternate_name_property = None;
					let mut r#application_deadline_property = None;
					let mut r#application_start_date_property = None;
					let mut r#day_of_week_property = None;
					let mut r#description_property = None;
					let mut r#disambiguating_description_property = None;
					let mut r#educational_credential_awarded_property = None;
					let mut r#educational_program_mode_property = None;
					let mut r#end_date_property = None;
					let mut r#financial_aid_eligible_property = None;
					let mut r#has_course_property = None;
					let mut r#identifier_property = None;
					let mut r#image_property = None;
					let mut r#main_entity_of_page_property = None;
					let mut r#maximum_enrollment_property = None;
					let mut r#name_property = None;
					let mut r#number_of_credits_property = None;
					let mut r#occupational_category_property = None;
					let mut r#occupational_credential_awarded_property = None;
					let mut r#offers_property = None;
					let mut r#potential_action_property = None;
					let mut r#program_prerequisites_property = None;
					let mut r#program_type_property = None;
					let mut r#provider_property = None;
					let mut r#salary_upon_completion_property = None;
					let mut r#same_as_property = None;
					let mut r#start_date_property = None;
					let mut r#subject_of_property = None;
					let mut r#term_duration_property = None;
					let mut r#terms_per_year_property = None;
					let mut r#time_of_day_property = None;
					let mut r#time_to_complete_property = None;
					let mut r#training_salary_property = None;
					let mut r#typical_credits_per_term_property = None;
					let mut r#url_property = None;
					while let Some(key) = map.next_key::<Field>()? {
						match key {
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
					Ok(EducationalOccupationalProgram {
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
						r#application_deadline: r#application_deadline_property.unwrap_or_default(),
						r#application_start_date: r#application_start_date_property
							.unwrap_or_default(),
						r#day_of_week: r#day_of_week_property.unwrap_or_default(),
						r#description: r#description_property.unwrap_or_default(),
						r#disambiguating_description: r#disambiguating_description_property
							.unwrap_or_default(),
						r#educational_credential_awarded: r#educational_credential_awarded_property
							.unwrap_or_default(),
						r#educational_program_mode: r#educational_program_mode_property
							.unwrap_or_default(),
						r#end_date: r#end_date_property.unwrap_or_default(),
						r#financial_aid_eligible: r#financial_aid_eligible_property
							.unwrap_or_default(),
						r#has_course: r#has_course_property.unwrap_or_default(),
						r#identifier: r#identifier_property.unwrap_or_default(),
						r#image: r#image_property.unwrap_or_default(),
						r#main_entity_of_page: r#main_entity_of_page_property.unwrap_or_default(),
						r#maximum_enrollment: r#maximum_enrollment_property.unwrap_or_default(),
						r#name: r#name_property.unwrap_or_default(),
						r#number_of_credits: r#number_of_credits_property.unwrap_or_default(),
						r#occupational_category: r#occupational_category_property
							.unwrap_or_default(),
						r#occupational_credential_awarded:
							r#occupational_credential_awarded_property.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#potential_action: r#potential_action_property.unwrap_or_default(),
						r#program_prerequisites: r#program_prerequisites_property
							.unwrap_or_default(),
						r#program_type: r#program_type_property.unwrap_or_default(),
						r#provider: r#provider_property.unwrap_or_default(),
						r#salary_upon_completion: r#salary_upon_completion_property
							.unwrap_or_default(),
						r#same_as: r#same_as_property.unwrap_or_default(),
						r#start_date: r#start_date_property.unwrap_or_default(),
						r#subject_of: r#subject_of_property.unwrap_or_default(),
						r#term_duration: r#term_duration_property.unwrap_or_default(),
						r#terms_per_year: r#terms_per_year_property.unwrap_or_default(),
						r#time_of_day: r#time_of_day_property.unwrap_or_default(),
						r#time_to_complete: r#time_to_complete_property.unwrap_or_default(),
						r#training_salary: r#training_salary_property.unwrap_or_default(),
						r#typical_credits_per_term: r#typical_credits_per_term_property
							.unwrap_or_default(),
						r#url: r#url_property.unwrap_or_default(),
					})
				}
			}
			const FIELDS: &[&str] = &[
				"additionalType",
				"alternateName",
				"applicationDeadline",
				"applicationStartDate",
				"dayOfWeek",
				"description",
				"disambiguatingDescription",
				"educationalCredentialAwarded",
				"educationalProgramMode",
				"endDate",
				"financialAidEligible",
				"hasCourse",
				"identifier",
				"image",
				"mainEntityOfPage",
				"maximumEnrollment",
				"name",
				"numberOfCredits",
				"occupationalCategory",
				"occupationalCredentialAwarded",
				"offers",
				"potentialAction",
				"programPrerequisites",
				"programType",
				"provider",
				"salaryUponCompletion",
				"sameAs",
				"startDate",
				"subjectOf",
				"termDuration",
				"termsPerYear",
				"timeOfDay",
				"timeToComplete",
				"trainingSalary",
				"typicalCreditsPerTerm",
				"url",
			];
			deserializer.deserialize_struct("EducationalOccupationalProgram", FIELDS, ClassVisitor)
		}
	}
}
