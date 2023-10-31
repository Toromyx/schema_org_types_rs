use super::*;
/// <https://schema.org/CDCPMDRecord>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct CdcpmdRecord {
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
			feature = "cvd-collection-date-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_collection_date: Vec<CvdCollectionDateProperty>,
	#[cfg(any(
		any(
			feature = "cvd-facility-county-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_facility_county: Vec<CvdFacilityCountyProperty>,
	#[cfg(any(
		any(
			feature = "cvd-facility-id-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_facility_id: Vec<CvdFacilityIdProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-beds-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_beds: Vec<CvdNumBedsProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-beds-occ-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_beds_occ: Vec<CvdNumBedsOccProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-c-19-died-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_c_19_died: Vec<CvdNumC19DiedProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-c-19-ho-pats-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_c_19_ho_pats: Vec<CvdNumC19HoPatsProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-c-19-hosp-pats-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_c_19_hosp_pats: Vec<CvdNumC19HospPatsProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-c-19-mech-vent-pats-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_c_19_mech_vent_pats: Vec<CvdNumC19MechVentPatsProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_c_19_of_mech_vent_pats: Vec<CvdNumC19OfMechVentPatsProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-c-19-overflow-pats-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_c_19_overflow_pats: Vec<CvdNumC19OverflowPatsProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-icu-beds-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_icu_beds: Vec<CvdNumIcuBedsProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-icu-beds-occ-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_icu_beds_occ: Vec<CvdNumIcuBedsOccProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-tot-beds-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_tot_beds: Vec<CvdNumTotBedsProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-vent-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_vent: Vec<CvdNumVentProperty>,
	#[cfg(any(
		any(
			feature = "cvd-num-vent-use-property-schema",
			feature = "pending-schema-section"
		),
		doc
	))]
	pub r#cvd_num_vent_use: Vec<CvdNumVentUseProperty>,
	#[cfg(any(
		any(
			feature = "date-posted-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#date_posted: Vec<DatePostedProperty>,
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
		any(feature = "name-property-schema", feature = "general-schema-section"),
		doc
	))]
	pub r#name: Vec<NameProperty>,
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
			feature = "same-as-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#same_as: Vec<SameAsProperty>,
	#[cfg(any(
		any(
			feature = "subject-of-property-schema",
			feature = "general-schema-section"
		),
		doc
	))]
	pub r#subject_of: Vec<SubjectOfProperty>,
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
	impl Serialize for CdcpmdRecord {
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
						feature = "cvd-collection-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_collection_date) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-facility-county-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_facility_county) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-facility-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_facility_id) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_beds) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-beds-occ-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_beds_occ) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-c-19-died-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_c_19_died) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-c-19-ho-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_c_19_ho_pats) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-c-19-hosp-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_c_19_hosp_pats) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-c-19-mech-vent-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_c_19_mech_vent_pats) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_c_19_of_mech_vent_pats) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-c-19-overflow-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_c_19_overflow_pats) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-icu-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_icu_beds) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-icu-beds-occ-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_icu_beds_occ) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-tot-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_tot_beds) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-vent-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_vent) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "cvd-num-vent-use-property-schema",
						feature = "pending-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#cvd_num_vent_use) as usize
				} else {
					0
				},
				if cfg!(any(
					any(
						feature = "date-posted-property-schema",
						feature = "general-schema-section"
					),
					doc
				)) {
					!Vec::is_empty(&self.r#date_posted) as usize
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
				Serializer::serialize_struct(serializer, "CdcpmdRecord", len)?;
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
					feature = "cvd-collection-date-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-facility-county-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-facility-id-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-beds-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-beds-occ-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-c-19-died-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-c-19-ho-pats-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-c-19-hosp-pats-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-c-19-mech-vent-pats-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-c-19-overflow-pats-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-icu-beds-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-icu-beds-occ-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-tot-beds-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-vent-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "cvd-num-vent-use-property-schema",
					feature = "pending-schema-section"
				),
				doc
			))]
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
			#[cfg(any(
				any(
					feature = "date-posted-property-schema",
					feature = "general-schema-section"
				),
				doc
			))]
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
	impl<'de> Deserialize<'de> for CdcpmdRecord {
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
						feature = "cvd-collection-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdCollectionDate,
				#[cfg(any(
					any(
						feature = "cvd-facility-county-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdFacilityCounty,
				#[cfg(any(
					any(
						feature = "cvd-facility-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdFacilityId,
				#[cfg(any(
					any(
						feature = "cvd-num-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumBeds,
				#[cfg(any(
					any(
						feature = "cvd-num-beds-occ-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumBedsOcc,
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-died-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumC19Died,
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-ho-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumC19HoPats,
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-hosp-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumC19HospPats,
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-mech-vent-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumC19MechVentPats,
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumC19OfMechVentPats,
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-overflow-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumC19OverflowPats,
				#[cfg(any(
					any(
						feature = "cvd-num-icu-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumIcuBeds,
				#[cfg(any(
					any(
						feature = "cvd-num-icu-beds-occ-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumIcuBedsOcc,
				#[cfg(any(
					any(
						feature = "cvd-num-tot-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumTotBeds,
				#[cfg(any(
					any(
						feature = "cvd-num-vent-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumVent,
				#[cfg(any(
					any(
						feature = "cvd-num-vent-use-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				CvdNumVentUse,
				#[cfg(any(
					any(
						feature = "date-posted-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				DatePosted,
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
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				Name,
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
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SameAs,
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				SubjectOf,
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
								feature = "cvd-collection-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdCollectionDate" => Ok(Field::CvdCollectionDate),
						#[cfg(any(
							any(
								feature = "cvd-facility-county-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdFacilityCounty" => Ok(Field::CvdFacilityCounty),
						#[cfg(any(
							any(
								feature = "cvd-facility-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdFacilityId" => Ok(Field::CvdFacilityId),
						#[cfg(any(
							any(
								feature = "cvd-num-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumBeds" => Ok(Field::CvdNumBeds),
						#[cfg(any(
							any(
								feature = "cvd-num-beds-occ-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumBedsOcc" => Ok(Field::CvdNumBedsOcc),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-died-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumC19Died" => Ok(Field::CvdNumC19Died),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-ho-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumC19HOPats" => Ok(Field::CvdNumC19HoPats),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-hosp-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumC19HospPats" => Ok(Field::CvdNumC19HospPats),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-mech-vent-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumC19MechVentPats" => Ok(Field::CvdNumC19MechVentPats),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumC19OFMechVentPats" => Ok(Field::CvdNumC19OfMechVentPats),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-overflow-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumC19OverflowPats" => Ok(Field::CvdNumC19OverflowPats),
						#[cfg(any(
							any(
								feature = "cvd-num-icu-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumICUBeds" => Ok(Field::CvdNumIcuBeds),
						#[cfg(any(
							any(
								feature = "cvd-num-icu-beds-occ-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumICUBedsOcc" => Ok(Field::CvdNumIcuBedsOcc),
						#[cfg(any(
							any(
								feature = "cvd-num-tot-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumTotBeds" => Ok(Field::CvdNumTotBeds),
						#[cfg(any(
							any(
								feature = "cvd-num-vent-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumVent" => Ok(Field::CvdNumVent),
						#[cfg(any(
							any(
								feature = "cvd-num-vent-use-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						"cvdNumVentUse" => Ok(Field::CvdNumVentUse),
						#[cfg(any(
							any(
								feature = "date-posted-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"datePosted" => Ok(Field::DatePosted),
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
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"name" => Ok(Field::Name),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						"sameAs" => Ok(Field::SameAs),
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
								feature = "cvd-collection-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdCollectionDate" => Ok(Field::CvdCollectionDate),
						#[cfg(any(
							any(
								feature = "cvd-facility-county-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdFacilityCounty" => Ok(Field::CvdFacilityCounty),
						#[cfg(any(
							any(
								feature = "cvd-facility-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdFacilityId" => Ok(Field::CvdFacilityId),
						#[cfg(any(
							any(
								feature = "cvd-num-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumBeds" => Ok(Field::CvdNumBeds),
						#[cfg(any(
							any(
								feature = "cvd-num-beds-occ-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumBedsOcc" => Ok(Field::CvdNumBedsOcc),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-died-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumC19Died" => Ok(Field::CvdNumC19Died),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-ho-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumC19HOPats" => Ok(Field::CvdNumC19HoPats),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-hosp-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumC19HospPats" => Ok(Field::CvdNumC19HospPats),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-mech-vent-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumC19MechVentPats" => Ok(Field::CvdNumC19MechVentPats),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumC19OFMechVentPats" => Ok(Field::CvdNumC19OfMechVentPats),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-overflow-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumC19OverflowPats" => Ok(Field::CvdNumC19OverflowPats),
						#[cfg(any(
							any(
								feature = "cvd-num-icu-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumICUBeds" => Ok(Field::CvdNumIcuBeds),
						#[cfg(any(
							any(
								feature = "cvd-num-icu-beds-occ-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumICUBedsOcc" => Ok(Field::CvdNumIcuBedsOcc),
						#[cfg(any(
							any(
								feature = "cvd-num-tot-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumTotBeds" => Ok(Field::CvdNumTotBeds),
						#[cfg(any(
							any(
								feature = "cvd-num-vent-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumVent" => Ok(Field::CvdNumVent),
						#[cfg(any(
							any(
								feature = "cvd-num-vent-use-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						b"cvdNumVentUse" => Ok(Field::CvdNumVentUse),
						#[cfg(any(
							any(
								feature = "date-posted-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"datePosted" => Ok(Field::DatePosted),
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
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"name" => Ok(Field::Name),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						b"sameAs" => Ok(Field::SameAs),
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
				type Value = CdcpmdRecord;
				fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
					formatter.write_str("schema.org schema CDCPMDRecord")
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
							feature = "cvd-collection-date-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_collection_date_property = None;
					#[cfg(any(
						any(
							feature = "cvd-facility-county-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_facility_county_property = None;
					#[cfg(any(
						any(
							feature = "cvd-facility-id-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_facility_id_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-beds-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_beds_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-beds-occ-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_beds_occ_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-c-19-died-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_c_19_died_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-c-19-ho-pats-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_c_19_ho_pats_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-c-19-hosp-pats-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_c_19_hosp_pats_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-c-19-mech-vent-pats-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_c_19_mech_vent_pats_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_c_19_of_mech_vent_pats_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-c-19-overflow-pats-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_c_19_overflow_pats_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-icu-beds-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_icu_beds_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-icu-beds-occ-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_icu_beds_occ_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-tot-beds-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_tot_beds_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-vent-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_vent_property = None;
					#[cfg(any(
						any(
							feature = "cvd-num-vent-use-property-schema",
							feature = "pending-schema-section"
						),
						doc
					))]
					let mut r#cvd_num_vent_use_property = None;
					#[cfg(any(
						any(
							feature = "date-posted-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#date_posted_property = None;
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
						any(feature = "name-property-schema", feature = "general-schema-section"),
						doc
					))]
					let mut r#name_property = None;
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
							feature = "same-as-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#same_as_property = None;
					#[cfg(any(
						any(
							feature = "subject-of-property-schema",
							feature = "general-schema-section"
						),
						doc
					))]
					let mut r#subject_of_property = None;
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
									feature = "cvd-collection-date-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-facility-county-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-facility-id-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-beds-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-beds-occ-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-c-19-died-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-c-19-ho-pats-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-c-19-hosp-pats-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-c-19-mech-vent-pats-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-c-19-overflow-pats-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-icu-beds-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-icu-beds-occ-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-tot-beds-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-vent-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "cvd-num-vent-use-property-schema",
									feature = "pending-schema-section"
								),
								doc
							))]
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
							#[cfg(any(
								any(
									feature = "date-posted-property-schema",
									feature = "general-schema-section"
								),
								doc
							))]
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
					Ok(CdcpmdRecord {
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
								feature = "cvd-collection-date-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_collection_date: r#cvd_collection_date_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-facility-county-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_facility_county: r#cvd_facility_county_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-facility-id-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_facility_id: r#cvd_facility_id_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_beds: r#cvd_num_beds_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-beds-occ-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_beds_occ: r#cvd_num_beds_occ_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-died-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_c_19_died: r#cvd_num_c_19_died_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-ho-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_c_19_ho_pats: r#cvd_num_c_19_ho_pats_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-hosp-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_c_19_hosp_pats: r#cvd_num_c_19_hosp_pats_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-mech-vent-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_c_19_mech_vent_pats: r#cvd_num_c_19_mech_vent_pats_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_c_19_of_mech_vent_pats: r#cvd_num_c_19_of_mech_vent_pats_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-c-19-overflow-pats-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_c_19_overflow_pats: r#cvd_num_c_19_overflow_pats_property
							.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-icu-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_icu_beds: r#cvd_num_icu_beds_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-icu-beds-occ-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_icu_beds_occ: r#cvd_num_icu_beds_occ_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-tot-beds-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_tot_beds: r#cvd_num_tot_beds_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-vent-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_vent: r#cvd_num_vent_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "cvd-num-vent-use-property-schema",
								feature = "pending-schema-section"
							),
							doc
						))]
						r#cvd_num_vent_use: r#cvd_num_vent_use_property.unwrap_or_default(),
						#[cfg(any(
							any(
								feature = "date-posted-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#date_posted: r#date_posted_property.unwrap_or_default(),
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
								feature = "name-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#name: r#name_property.unwrap_or_default(),
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
								feature = "same-as-property-schema",
								feature = "general-schema-section"
							),
							doc
						))]
						r#same_as: r#same_as_property.unwrap_or_default(),
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
						feature = "cvd-collection-date-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdCollectionDate",
				#[cfg(any(
					any(
						feature = "cvd-facility-county-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdFacilityCounty",
				#[cfg(any(
					any(
						feature = "cvd-facility-id-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdFacilityId",
				#[cfg(any(
					any(
						feature = "cvd-num-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumBeds",
				#[cfg(any(
					any(
						feature = "cvd-num-beds-occ-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumBedsOcc",
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-died-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumC19Died",
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-ho-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumC19HOPats",
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-hosp-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumC19HospPats",
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-mech-vent-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumC19MechVentPats",
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumC19OFMechVentPats",
				#[cfg(any(
					any(
						feature = "cvd-num-c-19-overflow-pats-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumC19OverflowPats",
				#[cfg(any(
					any(
						feature = "cvd-num-icu-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumICUBeds",
				#[cfg(any(
					any(
						feature = "cvd-num-icu-beds-occ-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumICUBedsOcc",
				#[cfg(any(
					any(
						feature = "cvd-num-tot-beds-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumTotBeds",
				#[cfg(any(
					any(
						feature = "cvd-num-vent-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumVent",
				#[cfg(any(
					any(
						feature = "cvd-num-vent-use-property-schema",
						feature = "pending-schema-section"
					),
					doc
				))]
				"cvdNumVentUse",
				#[cfg(any(
					any(
						feature = "date-posted-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"datePosted",
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
					any(feature = "name-property-schema", feature = "general-schema-section"),
					doc
				))]
				"name",
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
						feature = "same-as-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"sameAs",
				#[cfg(any(
					any(
						feature = "subject-of-property-schema",
						feature = "general-schema-section"
					),
					doc
				))]
				"subjectOf",
				#[cfg(any(
					any(feature = "url-property-schema", feature = "general-schema-section"),
					doc
				))]
				"url",
			];
			deserializer.deserialize_struct("CdcpmdRecord", FIELDS, ClassVisitor)
		}
	}
}
