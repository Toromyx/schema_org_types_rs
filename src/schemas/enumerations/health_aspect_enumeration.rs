/// HealthAspectEnumeration enumerates several aspects of health content online, each of which might be described using [[hasHealthAspect]] and [[HealthTopicContent]].
///
/// https://schema.org/HealthAspectEnumeration
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum HealthAspectEnumeration {
    /// Content about the allergy-related aspects of a health topic.
    ///
    /// https://schema.org/AllergiesHealthAspect
    AllergiesHealthAspect,
    /// Content about the benefits and advantages of usage or utilization of topic.
    ///
    /// https://schema.org/BenefitsHealthAspect
    BenefitsHealthAspect,
    /// Information about the causes and main actions that gave rise to the topic.
    ///
    /// https://schema.org/CausesHealthAspect
    CausesHealthAspect,
    /// Content about contagion mechanisms and contagiousness information over the topic.
    ///
    /// https://schema.org/ContagiousnessHealthAspect
    ContagiousnessHealthAspect,
    /// Content about the effectiveness-related aspects of a health topic.
    ///
    /// https://schema.org/EffectivenessHealthAspect
    EffectivenessHealthAspect,
    /// Content that discusses practical and policy aspects for getting access to specific kinds of healthcare (e.g. distribution mechanisms for vaccines).
    ///
    /// https://schema.org/GettingAccessHealthAspect
    GettingAccessHealthAspect,
    /// Content that discusses and explains how a particular health-related topic works, e.g. in terms of mechanisms and underlying science.
    ///
    /// https://schema.org/HowItWorksHealthAspect
    HowItWorksHealthAspect,
    /// Information about how or where to find a topic. Also may contain location data that can be used for where to look for help if the topic is observed.
    ///
    /// https://schema.org/HowOrWhereHealthAspect
    HowOrWhereHealthAspect,
    /// Content discussing ingredients-related aspects of a health topic.
    ///
    /// https://schema.org/IngredientsHealthAspect
    IngredientsHealthAspect,
    /// Information about coping or life related to the topic.
    ///
    /// https://schema.org/LivingWithHealthAspect
    LivingWithHealthAspect,
    /// Related topics may be treated by a Topic.
    ///
    /// https://schema.org/MayTreatHealthAspect
    MayTreatHealthAspect,
    /// Content about common misconceptions and myths that are related to a topic.
    ///
    /// https://schema.org/MisconceptionsHealthAspect
    MisconceptionsHealthAspect,
    /// Overview of the content. Contains a summarized view of the topic with the most relevant information for an introduction.
    ///
    /// https://schema.org/OverviewHealthAspect
    OverviewHealthAspect,
    /// Content about the real life experience of patients or people that have lived a similar experience about the topic. May be forums, topics, Q-and-A and related material.
    ///
    /// https://schema.org/PatientExperienceHealthAspect
    PatientExperienceHealthAspect,
    /// Content discussing pregnancy-related aspects of a health topic.
    ///
    /// https://schema.org/PregnancyHealthAspect
    PregnancyHealthAspect,
    /// Information about actions or measures that can be taken to avoid getting the topic or reaching a critical situation related to the topic.
    ///
    /// https://schema.org/PreventionHealthAspect
    PreventionHealthAspect,
    /// Typical progression and happenings of life course of the topic.
    ///
    /// https://schema.org/PrognosisHealthAspect
    PrognosisHealthAspect,
    /// Other prominent or relevant topics tied to the main topic.
    ///
    /// https://schema.org/RelatedTopicsHealthAspect
    RelatedTopicsHealthAspect,
    /// Information about the risk factors and possible complications that may follow a topic.
    ///
    /// https://schema.org/RisksOrComplicationsHealthAspect
    RisksOrComplicationsHealthAspect,
    /// Content about the safety-related aspects of a health topic.
    ///
    /// https://schema.org/SafetyHealthAspect
    SafetyHealthAspect,
    /// Content about how to screen or further filter a topic.
    ///
    /// https://schema.org/ScreeningHealthAspect
    ScreeningHealthAspect,
    /// Information about questions that may be asked, when to see a professional, measures before seeing a doctor or content about the first consultation.
    ///
    /// https://schema.org/SeeDoctorHealthAspect
    SeeDoctorHealthAspect,
    /// Self care actions or measures that can be taken to sooth, health or avoid a topic. This may be carried at home and can be carried/managed by the person itself.
    ///
    /// https://schema.org/SelfCareHealthAspect
    SelfCareHealthAspect,
    /// Side effects that can be observed from the usage of the topic.
    ///
    /// https://schema.org/SideEffectsHealthAspect
    SideEffectsHealthAspect,
    /// Stages that can be observed from a topic.
    ///
    /// https://schema.org/StagesHealthAspect
    StagesHealthAspect,
    /// Symptoms or related symptoms of a Topic.
    ///
    /// https://schema.org/SymptomsHealthAspect
    SymptomsHealthAspect,
    /// Treatments or related therapies for a Topic.
    ///
    /// https://schema.org/TreatmentsHealthAspect
    TreatmentsHealthAspect,
    /// Categorization and other types related to a topic.
    ///
    /// https://schema.org/TypesHealthAspect
    TypesHealthAspect,
    /// Content about how, when, frequency and dosage of a topic.
    ///
    /// https://schema.org/UsageOrScheduleHealthAspect
    UsageOrScheduleHealthAspect,
}
