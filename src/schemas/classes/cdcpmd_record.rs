use super::*;
/// <https://schema.org/CDCPMDRecord>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct CdcpmdRecord {
	/// <https://schema.org/cvdCollectionDate>
	pub r#cvd_collection_date: Vec<CvdCollectionDateProperty>,
	/// <https://schema.org/cvdFacilityCounty>
	pub r#cvd_facility_county: Vec<CvdFacilityCountyProperty>,
	/// <https://schema.org/cvdFacilityId>
	pub r#cvd_facility_id: Vec<CvdFacilityIdProperty>,
	/// <https://schema.org/cvdNumBeds>
	pub r#cvd_num_beds: Vec<CvdNumBedsProperty>,
	/// <https://schema.org/cvdNumBedsOcc>
	pub r#cvd_num_beds_occ: Vec<CvdNumBedsOccProperty>,
	/// <https://schema.org/cvdNumC19Died>
	pub r#cvd_num_c_19_died: Vec<CvdNumC19DiedProperty>,
	/// <https://schema.org/cvdNumC19HOPats>
	pub r#cvd_num_c_19_ho_pats: Vec<CvdNumC19HoPatsProperty>,
	/// <https://schema.org/cvdNumC19HospPats>
	pub r#cvd_num_c_19_hosp_pats: Vec<CvdNumC19HospPatsProperty>,
	/// <https://schema.org/cvdNumC19MechVentPats>
	pub r#cvd_num_c_19_mech_vent_pats: Vec<CvdNumC19MechVentPatsProperty>,
	/// <https://schema.org/cvdNumC19OFMechVentPats>
	pub r#cvd_num_c_19_of_mech_vent_pats: Vec<CvdNumC19OfMechVentPatsProperty>,
	/// <https://schema.org/cvdNumC19OverflowPats>
	pub r#cvd_num_c_19_overflow_pats: Vec<CvdNumC19OverflowPatsProperty>,
	/// <https://schema.org/cvdNumICUBeds>
	pub r#cvd_num_icu_beds: Vec<CvdNumIcuBedsProperty>,
	/// <https://schema.org/cvdNumICUBedsOcc>
	pub r#cvd_num_icu_beds_occ: Vec<CvdNumIcuBedsOccProperty>,
	/// <https://schema.org/cvdNumTotBeds>
	pub r#cvd_num_tot_beds: Vec<CvdNumTotBedsProperty>,
	/// <https://schema.org/cvdNumVent>
	pub r#cvd_num_vent: Vec<CvdNumVentProperty>,
	/// <https://schema.org/cvdNumVentUse>
	pub r#cvd_num_vent_use: Vec<CvdNumVentUseProperty>,
	/// <https://schema.org/datePosted>
	pub r#date_posted: Vec<DatePostedProperty>,
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
/// This trait is for properties from <https://schema.org/CDCPMDRecord>.
pub trait CdcpmdRecordTrait {
	/// Get <https://schema.org/cvdCollectionDate> from [`Self`] as borrowed slice.
	fn get_cvd_collection_date(&self) -> &[CvdCollectionDateProperty];
	/// Take <https://schema.org/cvdCollectionDate> from [`Self`] as owned vector.
	fn take_cvd_collection_date(&mut self) -> Vec<CvdCollectionDateProperty>;
	/// Get <https://schema.org/cvdFacilityCounty> from [`Self`] as borrowed slice.
	fn get_cvd_facility_county(&self) -> &[CvdFacilityCountyProperty];
	/// Take <https://schema.org/cvdFacilityCounty> from [`Self`] as owned vector.
	fn take_cvd_facility_county(&mut self) -> Vec<CvdFacilityCountyProperty>;
	/// Get <https://schema.org/cvdFacilityId> from [`Self`] as borrowed slice.
	fn get_cvd_facility_id(&self) -> &[CvdFacilityIdProperty];
	/// Take <https://schema.org/cvdFacilityId> from [`Self`] as owned vector.
	fn take_cvd_facility_id(&mut self) -> Vec<CvdFacilityIdProperty>;
	/// Get <https://schema.org/cvdNumBeds> from [`Self`] as borrowed slice.
	fn get_cvd_num_beds(&self) -> &[CvdNumBedsProperty];
	/// Take <https://schema.org/cvdNumBeds> from [`Self`] as owned vector.
	fn take_cvd_num_beds(&mut self) -> Vec<CvdNumBedsProperty>;
	/// Get <https://schema.org/cvdNumBedsOcc> from [`Self`] as borrowed slice.
	fn get_cvd_num_beds_occ(&self) -> &[CvdNumBedsOccProperty];
	/// Take <https://schema.org/cvdNumBedsOcc> from [`Self`] as owned vector.
	fn take_cvd_num_beds_occ(&mut self) -> Vec<CvdNumBedsOccProperty>;
	/// Get <https://schema.org/cvdNumC19Died> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_died(&self) -> &[CvdNumC19DiedProperty];
	/// Take <https://schema.org/cvdNumC19Died> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_died(&mut self) -> Vec<CvdNumC19DiedProperty>;
	/// Get <https://schema.org/cvdNumC19HOPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_ho_pats(&self) -> &[CvdNumC19HoPatsProperty];
	/// Take <https://schema.org/cvdNumC19HOPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_ho_pats(&mut self) -> Vec<CvdNumC19HoPatsProperty>;
	/// Get <https://schema.org/cvdNumC19HospPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_hosp_pats(&self) -> &[CvdNumC19HospPatsProperty];
	/// Take <https://schema.org/cvdNumC19HospPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_hosp_pats(&mut self) -> Vec<CvdNumC19HospPatsProperty>;
	/// Get <https://schema.org/cvdNumC19MechVentPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_mech_vent_pats(&self) -> &[CvdNumC19MechVentPatsProperty];
	/// Take <https://schema.org/cvdNumC19MechVentPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_mech_vent_pats(&mut self) -> Vec<CvdNumC19MechVentPatsProperty>;
	/// Get <https://schema.org/cvdNumC19OFMechVentPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_of_mech_vent_pats(&self) -> &[CvdNumC19OfMechVentPatsProperty];
	/// Take <https://schema.org/cvdNumC19OFMechVentPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_of_mech_vent_pats(&mut self) -> Vec<CvdNumC19OfMechVentPatsProperty>;
	/// Get <https://schema.org/cvdNumC19OverflowPats> from [`Self`] as borrowed slice.
	fn get_cvd_num_c_19_overflow_pats(&self) -> &[CvdNumC19OverflowPatsProperty];
	/// Take <https://schema.org/cvdNumC19OverflowPats> from [`Self`] as owned vector.
	fn take_cvd_num_c_19_overflow_pats(&mut self) -> Vec<CvdNumC19OverflowPatsProperty>;
	/// Get <https://schema.org/cvdNumICUBeds> from [`Self`] as borrowed slice.
	fn get_cvd_num_icu_beds(&self) -> &[CvdNumIcuBedsProperty];
	/// Take <https://schema.org/cvdNumICUBeds> from [`Self`] as owned vector.
	fn take_cvd_num_icu_beds(&mut self) -> Vec<CvdNumIcuBedsProperty>;
	/// Get <https://schema.org/cvdNumICUBedsOcc> from [`Self`] as borrowed slice.
	fn get_cvd_num_icu_beds_occ(&self) -> &[CvdNumIcuBedsOccProperty];
	/// Take <https://schema.org/cvdNumICUBedsOcc> from [`Self`] as owned vector.
	fn take_cvd_num_icu_beds_occ(&mut self) -> Vec<CvdNumIcuBedsOccProperty>;
	/// Get <https://schema.org/cvdNumTotBeds> from [`Self`] as borrowed slice.
	fn get_cvd_num_tot_beds(&self) -> &[CvdNumTotBedsProperty];
	/// Take <https://schema.org/cvdNumTotBeds> from [`Self`] as owned vector.
	fn take_cvd_num_tot_beds(&mut self) -> Vec<CvdNumTotBedsProperty>;
	/// Get <https://schema.org/cvdNumVent> from [`Self`] as borrowed slice.
	fn get_cvd_num_vent(&self) -> &[CvdNumVentProperty];
	/// Take <https://schema.org/cvdNumVent> from [`Self`] as owned vector.
	fn take_cvd_num_vent(&mut self) -> Vec<CvdNumVentProperty>;
	/// Get <https://schema.org/cvdNumVentUse> from [`Self`] as borrowed slice.
	fn get_cvd_num_vent_use(&self) -> &[CvdNumVentUseProperty];
	/// Take <https://schema.org/cvdNumVentUse> from [`Self`] as owned vector.
	fn take_cvd_num_vent_use(&mut self) -> Vec<CvdNumVentUseProperty>;
	/// Get <https://schema.org/datePosted> from [`Self`] as borrowed slice.
	fn get_date_posted(&self) -> &[DatePostedProperty];
	/// Take <https://schema.org/datePosted> from [`Self`] as owned vector.
	fn take_date_posted(&mut self) -> Vec<DatePostedProperty>;
}
impl CdcpmdRecordTrait for CdcpmdRecord {
	fn get_cvd_collection_date(&self) -> &[CvdCollectionDateProperty] {
		self.r#cvd_collection_date.as_slice()
	}
	fn take_cvd_collection_date(&mut self) -> Vec<CvdCollectionDateProperty> {
		std::mem::take(&mut self.r#cvd_collection_date)
	}
	fn get_cvd_facility_county(&self) -> &[CvdFacilityCountyProperty] {
		self.r#cvd_facility_county.as_slice()
	}
	fn take_cvd_facility_county(&mut self) -> Vec<CvdFacilityCountyProperty> {
		std::mem::take(&mut self.r#cvd_facility_county)
	}
	fn get_cvd_facility_id(&self) -> &[CvdFacilityIdProperty] {
		self.r#cvd_facility_id.as_slice()
	}
	fn take_cvd_facility_id(&mut self) -> Vec<CvdFacilityIdProperty> {
		std::mem::take(&mut self.r#cvd_facility_id)
	}
	fn get_cvd_num_beds(&self) -> &[CvdNumBedsProperty] {
		self.r#cvd_num_beds.as_slice()
	}
	fn take_cvd_num_beds(&mut self) -> Vec<CvdNumBedsProperty> {
		std::mem::take(&mut self.r#cvd_num_beds)
	}
	fn get_cvd_num_beds_occ(&self) -> &[CvdNumBedsOccProperty] {
		self.r#cvd_num_beds_occ.as_slice()
	}
	fn take_cvd_num_beds_occ(&mut self) -> Vec<CvdNumBedsOccProperty> {
		std::mem::take(&mut self.r#cvd_num_beds_occ)
	}
	fn get_cvd_num_c_19_died(&self) -> &[CvdNumC19DiedProperty] {
		self.r#cvd_num_c_19_died.as_slice()
	}
	fn take_cvd_num_c_19_died(&mut self) -> Vec<CvdNumC19DiedProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_died)
	}
	fn get_cvd_num_c_19_ho_pats(&self) -> &[CvdNumC19HoPatsProperty] {
		self.r#cvd_num_c_19_ho_pats.as_slice()
	}
	fn take_cvd_num_c_19_ho_pats(&mut self) -> Vec<CvdNumC19HoPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_ho_pats)
	}
	fn get_cvd_num_c_19_hosp_pats(&self) -> &[CvdNumC19HospPatsProperty] {
		self.r#cvd_num_c_19_hosp_pats.as_slice()
	}
	fn take_cvd_num_c_19_hosp_pats(&mut self) -> Vec<CvdNumC19HospPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_hosp_pats)
	}
	fn get_cvd_num_c_19_mech_vent_pats(&self) -> &[CvdNumC19MechVentPatsProperty] {
		self.r#cvd_num_c_19_mech_vent_pats.as_slice()
	}
	fn take_cvd_num_c_19_mech_vent_pats(&mut self) -> Vec<CvdNumC19MechVentPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_mech_vent_pats)
	}
	fn get_cvd_num_c_19_of_mech_vent_pats(&self) -> &[CvdNumC19OfMechVentPatsProperty] {
		self.r#cvd_num_c_19_of_mech_vent_pats.as_slice()
	}
	fn take_cvd_num_c_19_of_mech_vent_pats(&mut self) -> Vec<CvdNumC19OfMechVentPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_of_mech_vent_pats)
	}
	fn get_cvd_num_c_19_overflow_pats(&self) -> &[CvdNumC19OverflowPatsProperty] {
		self.r#cvd_num_c_19_overflow_pats.as_slice()
	}
	fn take_cvd_num_c_19_overflow_pats(&mut self) -> Vec<CvdNumC19OverflowPatsProperty> {
		std::mem::take(&mut self.r#cvd_num_c_19_overflow_pats)
	}
	fn get_cvd_num_icu_beds(&self) -> &[CvdNumIcuBedsProperty] {
		self.r#cvd_num_icu_beds.as_slice()
	}
	fn take_cvd_num_icu_beds(&mut self) -> Vec<CvdNumIcuBedsProperty> {
		std::mem::take(&mut self.r#cvd_num_icu_beds)
	}
	fn get_cvd_num_icu_beds_occ(&self) -> &[CvdNumIcuBedsOccProperty] {
		self.r#cvd_num_icu_beds_occ.as_slice()
	}
	fn take_cvd_num_icu_beds_occ(&mut self) -> Vec<CvdNumIcuBedsOccProperty> {
		std::mem::take(&mut self.r#cvd_num_icu_beds_occ)
	}
	fn get_cvd_num_tot_beds(&self) -> &[CvdNumTotBedsProperty] {
		self.r#cvd_num_tot_beds.as_slice()
	}
	fn take_cvd_num_tot_beds(&mut self) -> Vec<CvdNumTotBedsProperty> {
		std::mem::take(&mut self.r#cvd_num_tot_beds)
	}
	fn get_cvd_num_vent(&self) -> &[CvdNumVentProperty] {
		self.r#cvd_num_vent.as_slice()
	}
	fn take_cvd_num_vent(&mut self) -> Vec<CvdNumVentProperty> {
		std::mem::take(&mut self.r#cvd_num_vent)
	}
	fn get_cvd_num_vent_use(&self) -> &[CvdNumVentUseProperty] {
		self.r#cvd_num_vent_use.as_slice()
	}
	fn take_cvd_num_vent_use(&mut self) -> Vec<CvdNumVentUseProperty> {
		std::mem::take(&mut self.r#cvd_num_vent_use)
	}
	fn get_date_posted(&self) -> &[DatePostedProperty] {
		self.r#date_posted.as_slice()
	}
	fn take_date_posted(&mut self) -> Vec<DatePostedProperty> {
		std::mem::take(&mut self.r#date_posted)
	}
}
impl StructuredValueTrait for CdcpmdRecord {}
impl ThingTrait for CdcpmdRecord {
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
	impl Serialize for CdcpmdRecord {
		fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
		where
			S: Serializer,
		{
			let len: usize = [
				!Vec::is_empty(&self.r#cvd_collection_date) as usize,
				!Vec::is_empty(&self.r#cvd_facility_county) as usize,
				!Vec::is_empty(&self.r#cvd_facility_id) as usize,
				!Vec::is_empty(&self.r#cvd_num_beds) as usize,
				!Vec::is_empty(&self.r#cvd_num_beds_occ) as usize,
				!Vec::is_empty(&self.r#cvd_num_c_19_died) as usize,
				!Vec::is_empty(&self.r#cvd_num_c_19_ho_pats) as usize,
				!Vec::is_empty(&self.r#cvd_num_c_19_hosp_pats) as usize,
				!Vec::is_empty(&self.r#cvd_num_c_19_mech_vent_pats) as usize,
				!Vec::is_empty(&self.r#cvd_num_c_19_of_mech_vent_pats) as usize,
				!Vec::is_empty(&self.r#cvd_num_c_19_overflow_pats) as usize,
				!Vec::is_empty(&self.r#cvd_num_icu_beds) as usize,
				!Vec::is_empty(&self.r#cvd_num_icu_beds_occ) as usize,
				!Vec::is_empty(&self.r#cvd_num_tot_beds) as usize,
				!Vec::is_empty(&self.r#cvd_num_vent) as usize,
				!Vec::is_empty(&self.r#cvd_num_vent_use) as usize,
				!Vec::is_empty(&self.r#date_posted) as usize,
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
				Serializer::serialize_struct(serializer, "CdcpmdRecord", len)?;
			if !Vec::is_empty(&self.r#cvd_collection_date) {
				serialize_struct.serialize_field("cvdCollectionDate", {
					struct SerializeWith<'a>(&'a Vec<CvdCollectionDateProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_collection_date)
				})?;
			} else {
				serialize_struct.skip_field("cvdCollectionDate")?;
			}
			if !Vec::is_empty(&self.r#cvd_facility_county) {
				serialize_struct.serialize_field("cvdFacilityCounty", {
					struct SerializeWith<'a>(&'a Vec<CvdFacilityCountyProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_facility_county)
				})?;
			} else {
				serialize_struct.skip_field("cvdFacilityCounty")?;
			}
			if !Vec::is_empty(&self.r#cvd_facility_id) {
				serialize_struct.serialize_field("cvdFacilityId", {
					struct SerializeWith<'a>(&'a Vec<CvdFacilityIdProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_facility_id)
				})?;
			} else {
				serialize_struct.skip_field("cvdFacilityId")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_beds) {
				serialize_struct.serialize_field("cvdNumBeds", {
					struct SerializeWith<'a>(&'a Vec<CvdNumBedsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_beds)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumBeds")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_beds_occ) {
				serialize_struct.serialize_field("cvdNumBedsOcc", {
					struct SerializeWith<'a>(&'a Vec<CvdNumBedsOccProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_beds_occ)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumBedsOcc")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_c_19_died) {
				serialize_struct.serialize_field("cvdNumC19Died", {
					struct SerializeWith<'a>(&'a Vec<CvdNumC19DiedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_c_19_died)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumC19Died")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_c_19_ho_pats) {
				serialize_struct.serialize_field("cvdNumC19HOPats", {
					struct SerializeWith<'a>(&'a Vec<CvdNumC19HoPatsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_c_19_ho_pats)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumC19HOPats")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_c_19_hosp_pats) {
				serialize_struct.serialize_field("cvdNumC19HospPats", {
					struct SerializeWith<'a>(&'a Vec<CvdNumC19HospPatsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_c_19_hosp_pats)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumC19HospPats")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_c_19_mech_vent_pats) {
				serialize_struct.serialize_field("cvdNumC19MechVentPats", {
					struct SerializeWith<'a>(&'a Vec<CvdNumC19MechVentPatsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_c_19_mech_vent_pats)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumC19MechVentPats")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_c_19_of_mech_vent_pats) {
				serialize_struct.serialize_field("cvdNumC19OFMechVentPats", {
					struct SerializeWith<'a>(&'a Vec<CvdNumC19OfMechVentPatsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_c_19_of_mech_vent_pats)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumC19OFMechVentPats")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_c_19_overflow_pats) {
				serialize_struct.serialize_field("cvdNumC19OverflowPats", {
					struct SerializeWith<'a>(&'a Vec<CvdNumC19OverflowPatsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_c_19_overflow_pats)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumC19OverflowPats")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_icu_beds) {
				serialize_struct.serialize_field("cvdNumICUBeds", {
					struct SerializeWith<'a>(&'a Vec<CvdNumIcuBedsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_icu_beds)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumICUBeds")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_icu_beds_occ) {
				serialize_struct.serialize_field("cvdNumICUBedsOcc", {
					struct SerializeWith<'a>(&'a Vec<CvdNumIcuBedsOccProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_icu_beds_occ)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumICUBedsOcc")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_tot_beds) {
				serialize_struct.serialize_field("cvdNumTotBeds", {
					struct SerializeWith<'a>(&'a Vec<CvdNumTotBedsProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_tot_beds)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumTotBeds")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_vent) {
				serialize_struct.serialize_field("cvdNumVent", {
					struct SerializeWith<'a>(&'a Vec<CvdNumVentProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_vent)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumVent")?;
			}
			if !Vec::is_empty(&self.r#cvd_num_vent_use) {
				serialize_struct.serialize_field("cvdNumVentUse", {
					struct SerializeWith<'a>(&'a Vec<CvdNumVentUseProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#cvd_num_vent_use)
				})?;
			} else {
				serialize_struct.skip_field("cvdNumVentUse")?;
			}
			if !Vec::is_empty(&self.r#date_posted) {
				serialize_struct.serialize_field("datePosted", {
					struct SerializeWith<'a>(&'a Vec<DatePostedProperty>);
					impl<'a> Serialize for SerializeWith<'a> {
						fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
						where
							S: Serializer,
						{
							serde_with::As::<serde_with::OneOrMany<serde_with::Same>>::serialize(
								self.0, serializer,
							)
						}
					}
					&SerializeWith(&self.r#date_posted)
				})?;
			} else {
				serialize_struct.skip_field("datePosted")?;
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
	impl<'de> Deserialize<'de> for CdcpmdRecord {
		fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where
			D: Deserializer<'de>,
		{
			enum Field {
				CvdCollectionDate,
				CvdFacilityCounty,
				CvdFacilityId,
				CvdNumBeds,
				CvdNumBedsOcc,
				CvdNumC19Died,
				CvdNumC19HoPats,
				CvdNumC19HospPats,
				CvdNumC19MechVentPats,
				CvdNumC19OfMechVentPats,
				CvdNumC19OverflowPats,
				CvdNumIcuBeds,
				CvdNumIcuBedsOcc,
				CvdNumTotBeds,
				CvdNumVent,
				CvdNumVentUse,
				DatePosted,
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
						"cvdCollectionDate" => Ok(Field::CvdCollectionDate),
						"cvdFacilityCounty" => Ok(Field::CvdFacilityCounty),
						"cvdFacilityId" => Ok(Field::CvdFacilityId),
						"cvdNumBeds" => Ok(Field::CvdNumBeds),
						"cvdNumBedsOcc" => Ok(Field::CvdNumBedsOcc),
						"cvdNumC19Died" => Ok(Field::CvdNumC19Died),
						"cvdNumC19HOPats" => Ok(Field::CvdNumC19HoPats),
						"cvdNumC19HospPats" => Ok(Field::CvdNumC19HospPats),
						"cvdNumC19MechVentPats" => Ok(Field::CvdNumC19MechVentPats),
						"cvdNumC19OFMechVentPats" => Ok(Field::CvdNumC19OfMechVentPats),
						"cvdNumC19OverflowPats" => Ok(Field::CvdNumC19OverflowPats),
						"cvdNumICUBeds" => Ok(Field::CvdNumIcuBeds),
						"cvdNumICUBedsOcc" => Ok(Field::CvdNumIcuBedsOcc),
						"cvdNumTotBeds" => Ok(Field::CvdNumTotBeds),
						"cvdNumVent" => Ok(Field::CvdNumVent),
						"cvdNumVentUse" => Ok(Field::CvdNumVentUse),
						"datePosted" => Ok(Field::DatePosted),
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
						b"cvdCollectionDate" => Ok(Field::CvdCollectionDate),
						b"cvdFacilityCounty" => Ok(Field::CvdFacilityCounty),
						b"cvdFacilityId" => Ok(Field::CvdFacilityId),
						b"cvdNumBeds" => Ok(Field::CvdNumBeds),
						b"cvdNumBedsOcc" => Ok(Field::CvdNumBedsOcc),
						b"cvdNumC19Died" => Ok(Field::CvdNumC19Died),
						b"cvdNumC19HOPats" => Ok(Field::CvdNumC19HoPats),
						b"cvdNumC19HospPats" => Ok(Field::CvdNumC19HospPats),
						b"cvdNumC19MechVentPats" => Ok(Field::CvdNumC19MechVentPats),
						b"cvdNumC19OFMechVentPats" => Ok(Field::CvdNumC19OfMechVentPats),
						b"cvdNumC19OverflowPats" => Ok(Field::CvdNumC19OverflowPats),
						b"cvdNumICUBeds" => Ok(Field::CvdNumIcuBeds),
						b"cvdNumICUBedsOcc" => Ok(Field::CvdNumIcuBedsOcc),
						b"cvdNumTotBeds" => Ok(Field::CvdNumTotBeds),
						b"cvdNumVent" => Ok(Field::CvdNumVent),
						b"cvdNumVentUse" => Ok(Field::CvdNumVentUse),
						b"datePosted" => Ok(Field::DatePosted),
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
				type Value = CdcpmdRecord;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema CDCPMDRecord")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#cvd_collection_date_property = None;
					let mut r#cvd_facility_county_property = None;
					let mut r#cvd_facility_id_property = None;
					let mut r#cvd_num_beds_property = None;
					let mut r#cvd_num_beds_occ_property = None;
					let mut r#cvd_num_c_19_died_property = None;
					let mut r#cvd_num_c_19_ho_pats_property = None;
					let mut r#cvd_num_c_19_hosp_pats_property = None;
					let mut r#cvd_num_c_19_mech_vent_pats_property = None;
					let mut r#cvd_num_c_19_of_mech_vent_pats_property = None;
					let mut r#cvd_num_c_19_overflow_pats_property = None;
					let mut r#cvd_num_icu_beds_property = None;
					let mut r#cvd_num_icu_beds_occ_property = None;
					let mut r#cvd_num_tot_beds_property = None;
					let mut r#cvd_num_vent_property = None;
					let mut r#cvd_num_vent_use_property = None;
					let mut r#date_posted_property = None;
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
							Field::CvdCollectionDate => {
								if r#cvd_collection_date_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdCollectionDate",
									));
								}
								r#cvd_collection_date_property = Some({
									struct DeserializeWith(Vec<CvdCollectionDateProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdFacilityCounty => {
								if r#cvd_facility_county_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdFacilityCounty",
									));
								}
								r#cvd_facility_county_property = Some({
									struct DeserializeWith(Vec<CvdFacilityCountyProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdFacilityId => {
								if r#cvd_facility_id_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdFacilityId",
									));
								}
								r#cvd_facility_id_property = Some({
									struct DeserializeWith(Vec<CvdFacilityIdProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumBeds => {
								if r#cvd_num_beds_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumBeds",
									));
								}
								r#cvd_num_beds_property = Some({
									struct DeserializeWith(Vec<CvdNumBedsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumBedsOcc => {
								if r#cvd_num_beds_occ_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumBedsOcc",
									));
								}
								r#cvd_num_beds_occ_property = Some({
									struct DeserializeWith(Vec<CvdNumBedsOccProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumC19Died => {
								if r#cvd_num_c_19_died_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumC19Died",
									));
								}
								r#cvd_num_c_19_died_property = Some({
									struct DeserializeWith(Vec<CvdNumC19DiedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumC19HoPats => {
								if r#cvd_num_c_19_ho_pats_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumC19HOPats",
									));
								}
								r#cvd_num_c_19_ho_pats_property = Some({
									struct DeserializeWith(Vec<CvdNumC19HoPatsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumC19HospPats => {
								if r#cvd_num_c_19_hosp_pats_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumC19HospPats",
									));
								}
								r#cvd_num_c_19_hosp_pats_property = Some({
									struct DeserializeWith(Vec<CvdNumC19HospPatsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumC19MechVentPats => {
								if r#cvd_num_c_19_mech_vent_pats_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumC19MechVentPats",
									));
								}
								r#cvd_num_c_19_mech_vent_pats_property = Some({
									struct DeserializeWith(Vec<CvdNumC19MechVentPatsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumC19OfMechVentPats => {
								if r#cvd_num_c_19_of_mech_vent_pats_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumC19OFMechVentPats",
									));
								}
								r#cvd_num_c_19_of_mech_vent_pats_property = Some({
									struct DeserializeWith(Vec<CvdNumC19OfMechVentPatsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumC19OverflowPats => {
								if r#cvd_num_c_19_overflow_pats_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumC19OverflowPats",
									));
								}
								r#cvd_num_c_19_overflow_pats_property = Some({
									struct DeserializeWith(Vec<CvdNumC19OverflowPatsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumIcuBeds => {
								if r#cvd_num_icu_beds_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumICUBeds",
									));
								}
								r#cvd_num_icu_beds_property = Some({
									struct DeserializeWith(Vec<CvdNumIcuBedsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumIcuBedsOcc => {
								if r#cvd_num_icu_beds_occ_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumICUBedsOcc",
									));
								}
								r#cvd_num_icu_beds_occ_property = Some({
									struct DeserializeWith(Vec<CvdNumIcuBedsOccProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumTotBeds => {
								if r#cvd_num_tot_beds_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumTotBeds",
									));
								}
								r#cvd_num_tot_beds_property = Some({
									struct DeserializeWith(Vec<CvdNumTotBedsProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumVent => {
								if r#cvd_num_vent_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumVent",
									));
								}
								r#cvd_num_vent_property = Some({
									struct DeserializeWith(Vec<CvdNumVentProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::CvdNumVentUse => {
								if r#cvd_num_vent_use_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"cvdNumVentUse",
									));
								}
								r#cvd_num_vent_use_property = Some({
									struct DeserializeWith(Vec<CvdNumVentUseProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
									}
									match map.next_value::<DeserializeWith>() {
										Ok(deserialize_with) => deserialize_with.0,
										Err(err) => {
											return Err(err);
										}
									}
								});
							}
							Field::DatePosted => {
								if r#date_posted_property.is_some() {
									return Err(<A::Error as de::Error>::duplicate_field(
										"datePosted",
									));
								}
								r#date_posted_property = Some({
									struct DeserializeWith(Vec<DatePostedProperty>);
									impl<'de> Deserialize<'de> for DeserializeWith {
										fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
										where
											D: Deserializer<'de>,
										{
											Ok(DeserializeWith(serde_with::As::<
												serde_with::OneOrMany<serde_with::Same>,
											>::deserialize(deserializer)?))
										}
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
					Ok(CdcpmdRecord {
						r#cvd_collection_date: r#cvd_collection_date_property.unwrap_or_default(),
						r#cvd_facility_county: r#cvd_facility_county_property.unwrap_or_default(),
						r#cvd_facility_id: r#cvd_facility_id_property.unwrap_or_default(),
						r#cvd_num_beds: r#cvd_num_beds_property.unwrap_or_default(),
						r#cvd_num_beds_occ: r#cvd_num_beds_occ_property.unwrap_or_default(),
						r#cvd_num_c_19_died: r#cvd_num_c_19_died_property.unwrap_or_default(),
						r#cvd_num_c_19_ho_pats: r#cvd_num_c_19_ho_pats_property.unwrap_or_default(),
						r#cvd_num_c_19_hosp_pats: r#cvd_num_c_19_hosp_pats_property
							.unwrap_or_default(),
						r#cvd_num_c_19_mech_vent_pats: r#cvd_num_c_19_mech_vent_pats_property
							.unwrap_or_default(),
						r#cvd_num_c_19_of_mech_vent_pats: r#cvd_num_c_19_of_mech_vent_pats_property
							.unwrap_or_default(),
						r#cvd_num_c_19_overflow_pats: r#cvd_num_c_19_overflow_pats_property
							.unwrap_or_default(),
						r#cvd_num_icu_beds: r#cvd_num_icu_beds_property.unwrap_or_default(),
						r#cvd_num_icu_beds_occ: r#cvd_num_icu_beds_occ_property.unwrap_or_default(),
						r#cvd_num_tot_beds: r#cvd_num_tot_beds_property.unwrap_or_default(),
						r#cvd_num_vent: r#cvd_num_vent_property.unwrap_or_default(),
						r#cvd_num_vent_use: r#cvd_num_vent_use_property.unwrap_or_default(),
						r#date_posted: r#date_posted_property.unwrap_or_default(),
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
				"cvdCollectionDate",
				"cvdFacilityCounty",
				"cvdFacilityId",
				"cvdNumBeds",
				"cvdNumBedsOcc",
				"cvdNumC19Died",
				"cvdNumC19HOPats",
				"cvdNumC19HospPats",
				"cvdNumC19MechVentPats",
				"cvdNumC19OFMechVentPats",
				"cvdNumC19OverflowPats",
				"cvdNumICUBeds",
				"cvdNumICUBedsOcc",
				"cvdNumTotBeds",
				"cvdNumVent",
				"cvdNumVentUse",
				"datePosted",
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
			deserializer.deserialize_struct("CdcpmdRecord", FIELDS, ClassVisitor)
		}
	}
}
