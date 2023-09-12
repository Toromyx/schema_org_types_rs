/// Categories of physical activity, organized by physiologic classification.
///
/// https://schema.org/PhysicalActivityCategory
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PhysicalActivityCategory {
    /// Physical activity of relatively low intensity that depends primarily on the aerobic energy-generating process; during activity, the aerobic metabolism uses oxygen to adequately meet energy demands during exercise.
    ///
    /// https://schema.org/AerobicActivity
    AerobicActivity,
    /// Physical activity that is of high-intensity which utilizes the anaerobic metabolism of the body.
    ///
    /// https://schema.org/AnaerobicActivity
    AnaerobicActivity,
    /// Physical activity that is engaged to help maintain posture and balance.
    ///
    /// https://schema.org/Balance
    Balance,
    /// Physical activity that is engaged in to improve joint and muscle flexibility.
    ///
    /// https://schema.org/Flexibility
    Flexibility,
    /// Any physical activity engaged in for recreational purposes. Examples may include ballroom dancing, roller skating, canoeing, fishing, etc.
    ///
    /// https://schema.org/LeisureTimeActivity
    LeisureTimeActivity,
    /// Any physical activity engaged in for job-related purposes. Examples may include waiting tables, maid service, carrying a mailbag, picking fruits or vegetables, construction work, etc.
    ///
    /// https://schema.org/OccupationalActivity
    OccupationalActivity,
    /// Physical activity that is engaged in to improve muscle and bone strength. Also referred to as resistance training.
    ///
    /// https://schema.org/StrengthTraining
    StrengthTraining,
}
