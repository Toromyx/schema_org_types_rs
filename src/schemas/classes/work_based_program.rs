use super::*;
/// <https://schema.org/WorkBasedProgram>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct WorkBasedProgram {
	pub r#occupational_category: Vec<OccupationalCategoryProperty>,
	pub r#training_salary: Vec<TrainingSalaryProperty>,
	pub r#application_deadline: Vec<ApplicationDeadlineProperty>,
	pub r#application_start_date: Vec<ApplicationStartDateProperty>,
	pub r#day_of_week: Vec<DayOfWeekProperty>,
	pub r#educational_credential_awarded: Vec<EducationalCredentialAwardedProperty>,
	pub r#educational_program_mode: Vec<EducationalProgramModeProperty>,
	pub r#end_date: Vec<EndDateProperty>,
	pub r#financial_aid_eligible: Vec<FinancialAidEligibleProperty>,
	pub r#has_course: Vec<HasCourseProperty>,
	pub r#maximum_enrollment: Vec<MaximumEnrollmentProperty>,
	pub r#number_of_credits: Vec<NumberOfCreditsProperty>,
	pub r#occupational_credential_awarded: Vec<OccupationalCredentialAwardedProperty>,
	pub r#offers: Vec<OffersProperty>,
	pub r#program_prerequisites: Vec<ProgramPrerequisitesProperty>,
	pub r#program_type: Vec<ProgramTypeProperty>,
	pub r#provider: Vec<ProviderProperty>,
	pub r#salary_upon_completion: Vec<SalaryUponCompletionProperty>,
	pub r#start_date: Vec<StartDateProperty>,
	pub r#term_duration: Vec<TermDurationProperty>,
	pub r#terms_per_year: Vec<TermsPerYearProperty>,
	pub r#time_of_day: Vec<TimeOfDayProperty>,
	pub r#time_to_complete: Vec<TimeToCompleteProperty>,
	pub r#typical_credits_per_term: Vec<TypicalCreditsPerTermProperty>,
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#description: Vec<DescriptionProperty>,
	pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
	pub r#identifier: Vec<IdentifierProperty>,
	pub r#image: Vec<ImageProperty>,
	pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
	pub r#name: Vec<NameProperty>,
	pub r#potential_action: Vec<PotentialActionProperty>,
	pub r#same_as: Vec<SameAsProperty>,
	pub r#subject_of: Vec<SubjectOfProperty>,
	pub r#url: Vec<UrlProperty>,
}
pub trait WorkBasedProgramTrait {
	fn get_occupational_category(&self) -> &[OccupationalCategoryProperty];
	fn take_occupational_category(&mut self) -> Vec<OccupationalCategoryProperty>;
	fn get_training_salary(&self) -> &[TrainingSalaryProperty];
	fn take_training_salary(&mut self) -> Vec<TrainingSalaryProperty>;
}
impl WorkBasedProgramTrait for WorkBasedProgram {
	fn get_occupational_category(&self) -> &[OccupationalCategoryProperty] {
		self.r#occupational_category.as_slice()
	}
	fn take_occupational_category(&mut self) -> Vec<OccupationalCategoryProperty> {
		std::mem::take(&mut self.r#occupational_category)
	}
	fn get_training_salary(&self) -> &[TrainingSalaryProperty] {
		self.r#training_salary.as_slice()
	}
	fn take_training_salary(&mut self) -> Vec<TrainingSalaryProperty> {
		std::mem::take(&mut self.r#training_salary)
	}
}
impl EducationalOccupationalProgramTrait for WorkBasedProgram {
	fn get_application_deadline(&self) -> &[ApplicationDeadlineProperty] {
		self.r#application_deadline.as_slice()
	}
	fn take_application_deadline(&mut self) -> Vec<ApplicationDeadlineProperty> {
		std::mem::take(&mut self.r#application_deadline)
	}
	fn get_application_start_date(&self) -> &[ApplicationStartDateProperty] {
		self.r#application_start_date.as_slice()
	}
	fn take_application_start_date(&mut self) -> Vec<ApplicationStartDateProperty> {
		std::mem::take(&mut self.r#application_start_date)
	}
	fn get_day_of_week(&self) -> &[DayOfWeekProperty] {
		self.r#day_of_week.as_slice()
	}
	fn take_day_of_week(&mut self) -> Vec<DayOfWeekProperty> {
		std::mem::take(&mut self.r#day_of_week)
	}
	fn get_educational_credential_awarded(&self) -> &[EducationalCredentialAwardedProperty] {
		self.r#educational_credential_awarded.as_slice()
	}
	fn take_educational_credential_awarded(&mut self) -> Vec<EducationalCredentialAwardedProperty> {
		std::mem::take(&mut self.r#educational_credential_awarded)
	}
	fn get_educational_program_mode(&self) -> &[EducationalProgramModeProperty] {
		self.r#educational_program_mode.as_slice()
	}
	fn take_educational_program_mode(&mut self) -> Vec<EducationalProgramModeProperty> {
		std::mem::take(&mut self.r#educational_program_mode)
	}
	fn get_end_date(&self) -> &[EndDateProperty] {
		self.r#end_date.as_slice()
	}
	fn take_end_date(&mut self) -> Vec<EndDateProperty> {
		std::mem::take(&mut self.r#end_date)
	}
	fn get_financial_aid_eligible(&self) -> &[FinancialAidEligibleProperty] {
		self.r#financial_aid_eligible.as_slice()
	}
	fn take_financial_aid_eligible(&mut self) -> Vec<FinancialAidEligibleProperty> {
		std::mem::take(&mut self.r#financial_aid_eligible)
	}
	fn get_has_course(&self) -> &[HasCourseProperty] {
		self.r#has_course.as_slice()
	}
	fn take_has_course(&mut self) -> Vec<HasCourseProperty> {
		std::mem::take(&mut self.r#has_course)
	}
	fn get_maximum_enrollment(&self) -> &[MaximumEnrollmentProperty] {
		self.r#maximum_enrollment.as_slice()
	}
	fn take_maximum_enrollment(&mut self) -> Vec<MaximumEnrollmentProperty> {
		std::mem::take(&mut self.r#maximum_enrollment)
	}
	fn get_number_of_credits(&self) -> &[NumberOfCreditsProperty] {
		self.r#number_of_credits.as_slice()
	}
	fn take_number_of_credits(&mut self) -> Vec<NumberOfCreditsProperty> {
		std::mem::take(&mut self.r#number_of_credits)
	}
	fn get_occupational_category(&self) -> &[OccupationalCategoryProperty] {
		self.r#occupational_category.as_slice()
	}
	fn take_occupational_category(&mut self) -> Vec<OccupationalCategoryProperty> {
		std::mem::take(&mut self.r#occupational_category)
	}
	fn get_occupational_credential_awarded(&self) -> &[OccupationalCredentialAwardedProperty] {
		self.r#occupational_credential_awarded.as_slice()
	}
	fn take_occupational_credential_awarded(
		&mut self,
	) -> Vec<OccupationalCredentialAwardedProperty> {
		std::mem::take(&mut self.r#occupational_credential_awarded)
	}
	fn get_offers(&self) -> &[OffersProperty] {
		self.r#offers.as_slice()
	}
	fn take_offers(&mut self) -> Vec<OffersProperty> {
		std::mem::take(&mut self.r#offers)
	}
	fn get_program_prerequisites(&self) -> &[ProgramPrerequisitesProperty] {
		self.r#program_prerequisites.as_slice()
	}
	fn take_program_prerequisites(&mut self) -> Vec<ProgramPrerequisitesProperty> {
		std::mem::take(&mut self.r#program_prerequisites)
	}
	fn get_program_type(&self) -> &[ProgramTypeProperty] {
		self.r#program_type.as_slice()
	}
	fn take_program_type(&mut self) -> Vec<ProgramTypeProperty> {
		std::mem::take(&mut self.r#program_type)
	}
	fn get_provider(&self) -> &[ProviderProperty] {
		self.r#provider.as_slice()
	}
	fn take_provider(&mut self) -> Vec<ProviderProperty> {
		std::mem::take(&mut self.r#provider)
	}
	fn get_salary_upon_completion(&self) -> &[SalaryUponCompletionProperty] {
		self.r#salary_upon_completion.as_slice()
	}
	fn take_salary_upon_completion(&mut self) -> Vec<SalaryUponCompletionProperty> {
		std::mem::take(&mut self.r#salary_upon_completion)
	}
	fn get_start_date(&self) -> &[StartDateProperty] {
		self.r#start_date.as_slice()
	}
	fn take_start_date(&mut self) -> Vec<StartDateProperty> {
		std::mem::take(&mut self.r#start_date)
	}
	fn get_term_duration(&self) -> &[TermDurationProperty] {
		self.r#term_duration.as_slice()
	}
	fn take_term_duration(&mut self) -> Vec<TermDurationProperty> {
		std::mem::take(&mut self.r#term_duration)
	}
	fn get_terms_per_year(&self) -> &[TermsPerYearProperty] {
		self.r#terms_per_year.as_slice()
	}
	fn take_terms_per_year(&mut self) -> Vec<TermsPerYearProperty> {
		std::mem::take(&mut self.r#terms_per_year)
	}
	fn get_time_of_day(&self) -> &[TimeOfDayProperty] {
		self.r#time_of_day.as_slice()
	}
	fn take_time_of_day(&mut self) -> Vec<TimeOfDayProperty> {
		std::mem::take(&mut self.r#time_of_day)
	}
	fn get_time_to_complete(&self) -> &[TimeToCompleteProperty] {
		self.r#time_to_complete.as_slice()
	}
	fn take_time_to_complete(&mut self) -> Vec<TimeToCompleteProperty> {
		std::mem::take(&mut self.r#time_to_complete)
	}
	fn get_training_salary(&self) -> &[TrainingSalaryProperty] {
		self.r#training_salary.as_slice()
	}
	fn take_training_salary(&mut self) -> Vec<TrainingSalaryProperty> {
		std::mem::take(&mut self.r#training_salary)
	}
	fn get_typical_credits_per_term(&self) -> &[TypicalCreditsPerTermProperty] {
		self.r#typical_credits_per_term.as_slice()
	}
	fn take_typical_credits_per_term(&mut self) -> Vec<TypicalCreditsPerTermProperty> {
		std::mem::take(&mut self.r#typical_credits_per_term)
	}
}
impl ThingTrait for WorkBasedProgram {
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
	impl Serialize for WorkBasedProgram {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#occupational_category) as usize,
				!Vec::is_empty(&self.r#training_salary) as usize,
				!Vec::is_empty(&self.r#application_deadline) as usize,
				!Vec::is_empty(&self.r#application_start_date) as usize,
				!Vec::is_empty(&self.r#day_of_week) as usize,
				!Vec::is_empty(&self.r#educational_credential_awarded) as usize,
				!Vec::is_empty(&self.r#educational_program_mode) as usize,
				!Vec::is_empty(&self.r#end_date) as usize,
				!Vec::is_empty(&self.r#financial_aid_eligible) as usize,
				!Vec::is_empty(&self.r#has_course) as usize,
				!Vec::is_empty(&self.r#maximum_enrollment) as usize,
				!Vec::is_empty(&self.r#number_of_credits) as usize,
				!Vec::is_empty(&self.r#occupational_credential_awarded) as usize,
				!Vec::is_empty(&self.r#offers) as usize,
				!Vec::is_empty(&self.r#program_prerequisites) as usize,
				!Vec::is_empty(&self.r#program_type) as usize,
				!Vec::is_empty(&self.r#provider) as usize,
				!Vec::is_empty(&self.r#salary_upon_completion) as usize,
				!Vec::is_empty(&self.r#start_date) as usize,
				!Vec::is_empty(&self.r#term_duration) as usize,
				!Vec::is_empty(&self.r#terms_per_year) as usize,
				!Vec::is_empty(&self.r#time_of_day) as usize,
				!Vec::is_empty(&self.r#time_to_complete) as usize,
				!Vec::is_empty(&self.r#typical_credits_per_term) as usize,
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
				Serializer::serialize_struct(serializer, "WorkBasedProgram", len)?;
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
	impl<'de> Deserialize<'de> for WorkBasedProgram {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				OccupationalCategory,
				TrainingSalary,
				ApplicationDeadline,
				ApplicationStartDate,
				DayOfWeek,
				EducationalCredentialAwarded,
				EducationalProgramMode,
				EndDate,
				FinancialAidEligible,
				HasCourse,
				MaximumEnrollment,
				NumberOfCredits,
				OccupationalCredentialAwarded,
				Offers,
				ProgramPrerequisites,
				ProgramType,
				Provider,
				SalaryUponCompletion,
				StartDate,
				TermDuration,
				TermsPerYear,
				TimeOfDay,
				TimeToComplete,
				TypicalCreditsPerTerm,
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
						"occupationalCategory" => Ok(Field::OccupationalCategory),
						"trainingSalary" => Ok(Field::TrainingSalary),
						"applicationDeadline" => Ok(Field::ApplicationDeadline),
						"applicationStartDate" => Ok(Field::ApplicationStartDate),
						"dayOfWeek" => Ok(Field::DayOfWeek),
						"educationalCredentialAwarded" => Ok(Field::EducationalCredentialAwarded),
						"educationalProgramMode" => Ok(Field::EducationalProgramMode),
						"endDate" => Ok(Field::EndDate),
						"financialAidEligible" => Ok(Field::FinancialAidEligible),
						"hasCourse" => Ok(Field::HasCourse),
						"maximumEnrollment" => Ok(Field::MaximumEnrollment),
						"numberOfCredits" => Ok(Field::NumberOfCredits),
						"occupationalCredentialAwarded" => Ok(Field::OccupationalCredentialAwarded),
						"offers" => Ok(Field::Offers),
						"programPrerequisites" => Ok(Field::ProgramPrerequisites),
						"programType" => Ok(Field::ProgramType),
						"provider" => Ok(Field::Provider),
						"salaryUponCompletion" => Ok(Field::SalaryUponCompletion),
						"startDate" => Ok(Field::StartDate),
						"termDuration" => Ok(Field::TermDuration),
						"termsPerYear" => Ok(Field::TermsPerYear),
						"timeOfDay" => Ok(Field::TimeOfDay),
						"timeToComplete" => Ok(Field::TimeToComplete),
						"typicalCreditsPerTerm" => Ok(Field::TypicalCreditsPerTerm),
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
						b"occupationalCategory" => Ok(Field::OccupationalCategory),
						b"trainingSalary" => Ok(Field::TrainingSalary),
						b"applicationDeadline" => Ok(Field::ApplicationDeadline),
						b"applicationStartDate" => Ok(Field::ApplicationStartDate),
						b"dayOfWeek" => Ok(Field::DayOfWeek),
						b"educationalCredentialAwarded" => Ok(Field::EducationalCredentialAwarded),
						b"educationalProgramMode" => Ok(Field::EducationalProgramMode),
						b"endDate" => Ok(Field::EndDate),
						b"financialAidEligible" => Ok(Field::FinancialAidEligible),
						b"hasCourse" => Ok(Field::HasCourse),
						b"maximumEnrollment" => Ok(Field::MaximumEnrollment),
						b"numberOfCredits" => Ok(Field::NumberOfCredits),
						b"occupationalCredentialAwarded" => {
							Ok(Field::OccupationalCredentialAwarded)
						}
						b"offers" => Ok(Field::Offers),
						b"programPrerequisites" => Ok(Field::ProgramPrerequisites),
						b"programType" => Ok(Field::ProgramType),
						b"provider" => Ok(Field::Provider),
						b"salaryUponCompletion" => Ok(Field::SalaryUponCompletion),
						b"startDate" => Ok(Field::StartDate),
						b"termDuration" => Ok(Field::TermDuration),
						b"termsPerYear" => Ok(Field::TermsPerYear),
						b"timeOfDay" => Ok(Field::TimeOfDay),
						b"timeToComplete" => Ok(Field::TimeToComplete),
						b"typicalCreditsPerTerm" => Ok(Field::TypicalCreditsPerTerm),
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
				type Value = WorkBasedProgram;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema WorkBasedProgram")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#occupational_category_property = None;
					let mut r#training_salary_property = None;
					let mut r#application_deadline_property = None;
					let mut r#application_start_date_property = None;
					let mut r#day_of_week_property = None;
					let mut r#educational_credential_awarded_property = None;
					let mut r#educational_program_mode_property = None;
					let mut r#end_date_property = None;
					let mut r#financial_aid_eligible_property = None;
					let mut r#has_course_property = None;
					let mut r#maximum_enrollment_property = None;
					let mut r#number_of_credits_property = None;
					let mut r#occupational_credential_awarded_property = None;
					let mut r#offers_property = None;
					let mut r#program_prerequisites_property = None;
					let mut r#program_type_property = None;
					let mut r#provider_property = None;
					let mut r#salary_upon_completion_property = None;
					let mut r#start_date_property = None;
					let mut r#term_duration_property = None;
					let mut r#terms_per_year_property = None;
					let mut r#time_of_day_property = None;
					let mut r#time_to_complete_property = None;
					let mut r#typical_credits_per_term_property = None;
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
					Ok(WorkBasedProgram {
						r#occupational_category: r#occupational_category_property
							.unwrap_or_default(),
						r#training_salary: r#training_salary_property.unwrap_or_default(),
						r#application_deadline: r#application_deadline_property.unwrap_or_default(),
						r#application_start_date: r#application_start_date_property
							.unwrap_or_default(),
						r#day_of_week: r#day_of_week_property.unwrap_or_default(),
						r#educational_credential_awarded: r#educational_credential_awarded_property
							.unwrap_or_default(),
						r#educational_program_mode: r#educational_program_mode_property
							.unwrap_or_default(),
						r#end_date: r#end_date_property.unwrap_or_default(),
						r#financial_aid_eligible: r#financial_aid_eligible_property
							.unwrap_or_default(),
						r#has_course: r#has_course_property.unwrap_or_default(),
						r#maximum_enrollment: r#maximum_enrollment_property.unwrap_or_default(),
						r#number_of_credits: r#number_of_credits_property.unwrap_or_default(),
						r#occupational_credential_awarded:
							r#occupational_credential_awarded_property.unwrap_or_default(),
						r#offers: r#offers_property.unwrap_or_default(),
						r#program_prerequisites: r#program_prerequisites_property
							.unwrap_or_default(),
						r#program_type: r#program_type_property.unwrap_or_default(),
						r#provider: r#provider_property.unwrap_or_default(),
						r#salary_upon_completion: r#salary_upon_completion_property
							.unwrap_or_default(),
						r#start_date: r#start_date_property.unwrap_or_default(),
						r#term_duration: r#term_duration_property.unwrap_or_default(),
						r#terms_per_year: r#terms_per_year_property.unwrap_or_default(),
						r#time_of_day: r#time_of_day_property.unwrap_or_default(),
						r#time_to_complete: r#time_to_complete_property.unwrap_or_default(),
						r#typical_credits_per_term: r#typical_credits_per_term_property
							.unwrap_or_default(),
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
				"occupationalCategory",
				"trainingSalary",
				"applicationDeadline",
				"applicationStartDate",
				"dayOfWeek",
				"educationalCredentialAwarded",
				"educationalProgramMode",
				"endDate",
				"financialAidEligible",
				"hasCourse",
				"maximumEnrollment",
				"numberOfCredits",
				"occupationalCredentialAwarded",
				"offers",
				"programPrerequisites",
				"programType",
				"provider",
				"salaryUponCompletion",
				"startDate",
				"termDuration",
				"termsPerYear",
				"timeOfDay",
				"timeToComplete",
				"typicalCreditsPerTerm",
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
			deserializer.deserialize_struct("WorkBasedProgram", FIELDS, ClassVisitor)
		}
	}
}
