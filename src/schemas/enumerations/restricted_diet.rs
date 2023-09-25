/// A diet restricted to certain foods or preparations for cultural, religious, health or lifestyle reasons.
///
/// https://schema.org/RestrictedDiet
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum RestrictedDiet {
    /// A diet appropriate for people with diabetes.
    ///
    /// https://schema.org/DiabeticDiet
    DiabeticDiet,
    /// A diet exclusive of gluten.
    ///
    /// https://schema.org/GlutenFreeDiet
    GlutenFreeDiet,
    /// A diet conforming to Islamic dietary practices.
    ///
    /// https://schema.org/HalalDiet
    HalalDiet,
    /// A diet conforming to Hindu dietary practices, in particular, beef-free.
    ///
    /// https://schema.org/HinduDiet
    HinduDiet,
    /// A diet conforming to Jewish dietary practices.
    ///
    /// https://schema.org/KosherDiet
    KosherDiet,
    /// A diet focused on reduced calorie intake.
    ///
    /// https://schema.org/LowCalorieDiet
    LowCalorieDiet,
    /// A diet focused on reduced fat and cholesterol intake.
    ///
    /// https://schema.org/LowFatDiet
    LowFatDiet,
    /// A diet appropriate for people with lactose intolerance.
    ///
    /// https://schema.org/LowLactoseDiet
    LowLactoseDiet,
    /// A diet focused on reduced sodium intake.
    ///
    /// https://schema.org/LowSaltDiet
    LowSaltDiet,
    /// A diet exclusive of all animal products.
    ///
    /// https://schema.org/VeganDiet
    VeganDiet,
    /// A diet exclusive of animal meat.
    ///
    /// https://schema.org/VegetarianDiet
    VegetarianDiet,
}
