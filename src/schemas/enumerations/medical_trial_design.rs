/// Design models for medical trials. Enumerated type.
///
/// https://schema.org/MedicalTrialDesign
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum MedicalTrialDesign {
    /// A trial design in which neither the researcher nor the patient knows the details of the treatment the patient was randomly assigned to.
    ///
    /// https://schema.org/DoubleBlindedTrial
    DoubleBlindedTrial,
    /// An international trial.
    ///
    /// https://schema.org/InternationalTrial
    InternationalTrial,
    /// A trial that takes place at multiple centers.
    ///
    /// https://schema.org/MultiCenterTrial
    MultiCenterTrial,
    /// A trial design in which the researcher knows the full details of the treatment, and so does the patient.
    ///
    /// https://schema.org/OpenTrial
    OpenTrial,
    /// A placebo-controlled trial design.
    ///
    /// https://schema.org/PlaceboControlledTrial
    PlaceboControlledTrial,
    /// A randomized trial design.
    ///
    /// https://schema.org/RandomizedTrial
    RandomizedTrial,
    /// A trial design in which the researcher knows which treatment the patient was randomly assigned to but the patient does not.
    ///
    /// https://schema.org/SingleBlindedTrial
    SingleBlindedTrial,
    /// A trial that takes place at a single center.
    ///
    /// https://schema.org/SingleCenterTrial
    SingleCenterTrial,
    /// A trial design in which neither the researcher, the person administering the therapy nor the patient knows the details of the treatment the patient was randomly assigned to.
    ///
    /// https://schema.org/TripleBlindedTrial
    TripleBlindedTrial,
}
