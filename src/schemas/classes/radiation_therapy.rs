use super::*;
/// A process of care using radiation aimed at improving a health condition.
///
/// <https://schema.org/RadiationTherapy>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub struct RadiationTherapy {
    #[cfg(any(
        any(
            feature = "additional-type-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "additionalType"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#additional_type: Vec<AdditionalTypeProperty>,
    #[cfg(any(
        any(
            feature = "adverse-outcome-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "adverseOutcome"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#adverse_outcome: Vec<AdverseOutcomeProperty>,
    #[cfg(any(
        any(
            feature = "alternate-name-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "alternateName"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#alternate_name: Vec<AlternateNameProperty>,
    #[cfg(any(
        any(
            feature = "body-location-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "bodyLocation"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#body_location: Vec<BodyLocationProperty>,
    #[cfg(any(
        any(
            feature = "code-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "code"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#code: Vec<CodeProperty>,
    #[cfg(any(
        any(
            feature = "contraindication-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "contraindication"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#contraindication: Vec<ContraindicationProperty>,
    #[cfg(any(
        any(
            feature = "description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "description"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#description: Vec<DescriptionProperty>,
    #[cfg(any(
        any(
            feature = "disambiguating-description-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(rename = "disambiguatingDescription")
    )]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#disambiguating_description: Vec<DisambiguatingDescriptionProperty>,
    #[cfg(any(
        any(
            feature = "dose-schedule-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "doseSchedule"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#dose_schedule: Vec<DoseScheduleProperty>,
    #[cfg(any(
        any(
            feature = "drug-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "drug"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#drug: Vec<DrugProperty>,
    #[cfg(any(
        any(
            feature = "duplicate-therapy-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "duplicateTherapy"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#duplicate_therapy: Vec<DuplicateTherapyProperty>,
    #[cfg(any(
        any(
            feature = "followup-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "followup"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#followup: Vec<FollowupProperty>,
    #[cfg(any(
        any(
            feature = "funding-property-schema",
            feature = "pending-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "funding"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#funding: Vec<FundingProperty>,
    #[cfg(any(
        any(
            feature = "guideline-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "guideline"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#guideline: Vec<GuidelineProperty>,
    #[cfg(any(
        any(
            feature = "how-performed-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "howPerformed"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#how_performed: Vec<HowPerformedProperty>,
    #[cfg(any(
        any(
            feature = "identifier-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "identifier"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#identifier: Vec<IdentifierProperty>,
    #[cfg(any(
        any(feature = "image-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "image"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#image: Vec<ImageProperty>,
    #[cfg(any(
        any(
            feature = "legal-status-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "legalStatus"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#legal_status: Vec<LegalStatusProperty>,
    #[cfg(any(
        any(
            feature = "main-entity-of-page-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "mainEntityOfPage"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#main_entity_of_page: Vec<MainEntityOfPageProperty>,
    #[cfg(any(
        any(
            feature = "medicine-system-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "medicineSystem"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#medicine_system: Vec<MedicineSystemProperty>,
    #[cfg(any(
        any(feature = "name-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "name"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#name: Vec<NameProperty>,
    #[cfg(any(
        any(
            feature = "potential-action-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "potentialAction"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#potential_action: Vec<PotentialActionProperty>,
    #[cfg(any(
        any(
            feature = "preparation-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "preparation"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#preparation: Vec<PreparationProperty>,
    #[cfg(any(
        any(
            feature = "procedure-type-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "procedureType"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#procedure_type: Vec<ProcedureTypeProperty>,
    #[cfg(any(
        any(
            feature = "recognizing-authority-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "recognizingAuthority"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#recognizing_authority: Vec<RecognizingAuthorityProperty>,
    #[cfg(any(
        any(
            feature = "relevant-specialty-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "relevantSpecialty"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#relevant_specialty: Vec<RelevantSpecialtyProperty>,
    #[cfg(any(
        any(
            feature = "same-as-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "sameAs"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#same_as: Vec<SameAsProperty>,
    #[cfg(any(
        any(
            feature = "serious-adverse-outcome-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "seriousAdverseOutcome"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#serious_adverse_outcome: Vec<SeriousAdverseOutcomeProperty>,
    #[cfg(any(
        any(
            feature = "status-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "status"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#status: Vec<StatusProperty>,
    #[cfg(any(
        any(
            feature = "study-property-schema",
            feature = "health-lifesci-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "study"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#study: Vec<StudyProperty>,
    #[cfg(any(
        any(
            feature = "subject-of-property-schema",
            feature = "general-schema-section"
        ),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "subjectOf"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#subject_of: Vec<SubjectOfProperty>,
    #[cfg(any(
        any(feature = "url-property-schema", feature = "general-schema-section"),
        doc
    ))]
    #[cfg_attr(any(feature = "serde", doc), serde(rename = "url"))]
    #[cfg_attr(any(feature = "serde", doc), serde(default))]
    #[cfg_attr(
        any(feature = "serde", doc),
        serde(skip_serializing_if = "Vec::is_empty")
    )]
    pub r#url: Vec<UrlProperty>,
}
