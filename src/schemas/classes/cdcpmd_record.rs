use super::*;
/// <https://schema.org/CDCPMDRecord>
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CdcpmdRecord {
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "additionalType"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "alternateName"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "cvd-collection-date-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdCollectionDate"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_collection_date: Vec<CvdCollectionDateProperty>,
    #[cfg(any(
        any(
            feature = "cvd-facility-county-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdFacilityCounty"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_facility_county: Vec<CvdFacilityCountyProperty>,
    #[cfg(any(
        any(
            feature = "cvd-facility-id-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdFacilityId"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_facility_id: Vec<CvdFacilityIdProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-beds-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumBeds"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_beds: Vec<CvdNumBedsProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-beds-occ-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumBedsOcc"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_beds_occ: Vec<CvdNumBedsOccProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-c-19-died-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumC19Died"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_c_19_died: Vec<CvdNumC19DiedProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-c-19-ho-pats-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumC19HOPats"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_c_19_ho_pats: Vec<CvdNumC19HoPatsProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-c-19-hosp-pats-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumC19HospPats"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_c_19_hosp_pats: Vec<CvdNumC19HospPatsProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-c-19-mech-vent-pats-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumC19MechVentPats"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_c_19_mech_vent_pats: Vec<CvdNumC19MechVentPatsProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-c-19-of-mech-vent-pats-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumC19OFMechVentPats"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_c_19_of_mech_vent_pats: Vec<CvdNumC19OfMechVentPatsProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-c-19-overflow-pats-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumC19OverflowPats"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_c_19_overflow_pats: Vec<CvdNumC19OverflowPatsProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-icu-beds-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumICUBeds"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_icu_beds: Vec<CvdNumIcuBedsProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-icu-beds-occ-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumICUBedsOcc"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_icu_beds_occ: Vec<CvdNumIcuBedsOccProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-tot-beds-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumTotBeds"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_tot_beds: Vec<CvdNumTotBedsProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-vent-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumVent"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_vent: Vec<CvdNumVentProperty>,
    #[cfg(any(
        any(
            feature = "cvd-num-vent-use-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "cvdNumVentUse"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#cvd_num_vent_use: Vec<CvdNumVentUseProperty>,
    #[cfg(any(
        any(
            feature = "date-posted-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "datePosted"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#date_posted: Vec<DatePostedProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "description"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "disambiguatingDescription"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "identifier"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "image"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "name"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "potentialAction"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "sameAs"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "subjectOf"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(feature = "serde", serde(rename = "url"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub r#url: Vec<UrlProperty>,
}
