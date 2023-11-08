use super::*;
/// <https://schema.org/CDCPMDRecord>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct CdcpmdRecord {
	pub r#additional_type: Vec<AdditionalTypeProperty>,
	pub r#alternate_name: Vec<AlternateNameProperty>,
	pub r#cvd_collection_date: Vec<CvdCollectionDateProperty>,
	pub r#cvd_facility_county: Vec<CvdFacilityCountyProperty>,
	pub r#cvd_facility_id: Vec<CvdFacilityIdProperty>,
	pub r#cvd_num_beds: Vec<CvdNumBedsProperty>,
	pub r#cvd_num_beds_occ: Vec<CvdNumBedsOccProperty>,
	pub r#cvd_num_c_19_died: Vec<CvdNumC19DiedProperty>,
	pub r#cvd_num_c_19_ho_pats: Vec<CvdNumC19HoPatsProperty>,
	pub r#cvd_num_c_19_hosp_pats: Vec<CvdNumC19HospPatsProperty>,
	pub r#cvd_num_c_19_mech_vent_pats: Vec<CvdNumC19MechVentPatsProperty>,
	pub r#cvd_num_c_19_of_mech_vent_pats: Vec<CvdNumC19OfMechVentPatsProperty>,
	pub r#cvd_num_c_19_overflow_pats: Vec<CvdNumC19OverflowPatsProperty>,
	pub r#cvd_num_icu_beds: Vec<CvdNumIcuBedsProperty>,
	pub r#cvd_num_icu_beds_occ: Vec<CvdNumIcuBedsOccProperty>,
	pub r#cvd_num_tot_beds: Vec<CvdNumTotBedsProperty>,
	pub r#cvd_num_vent: Vec<CvdNumVentProperty>,
	pub r#cvd_num_vent_use: Vec<CvdNumVentUseProperty>,
	pub r#date_posted: Vec<DatePostedProperty>,
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
				!Vec::is_empty(&self.r#additional_type) as usize,
				!Vec::is_empty(&self.r#alternate_name) as usize,
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
				AdditionalType,
				AlternateName,
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
						"additionalType" => Ok(Field::AdditionalType),
						"alternateName" => Ok(Field::AlternateName),
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
						b"additionalType" => Ok(Field::AdditionalType),
						b"alternateName" => Ok(Field::AlternateName),
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
				type Value = CdcpmdRecord;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema CDCPMDRecord")
				}
				fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
				where
					A: de::MapAccess<'de>,
				{
					let mut r#additional_type_property = None;
					let mut r#alternate_name_property = None;
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
					Ok(CdcpmdRecord {
						r#additional_type: r#additional_type_property.unwrap_or_default(),
						r#alternate_name: r#alternate_name_property.unwrap_or_default(),
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
				"additionalType",
				"alternateName",
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
