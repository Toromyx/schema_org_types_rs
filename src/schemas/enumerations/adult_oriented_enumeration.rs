/// Enumeration of considerations that make a product relevant or potentially restricted for adults only.
///
/// <https://schema.org/AdultOrientedEnumeration>
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum AdultOrientedEnumeration {
    /// Item contains alcohol or promotes alcohol consumption.
    ///
    /// <https://schema.org/AlcoholConsideration>
    AlcoholConsideration,
    /// The item is dangerous and requires careful handling and/or special training of the user. See also the [UN Model Classification](https://unece.org/DAM/trans/danger/publi/unrec/rev17/English/02EREv17_Part2.pdf) defining the 9 classes of dangerous goods such as explosives, gases, flammables, and more.
    ///
    /// <https://schema.org/DangerousGoodConsideration>
    DangerousGoodConsideration,
    /// Item is a pharmaceutical (e.g., a prescription or OTC drug) or a restricted medical device.
    ///
    /// <https://schema.org/HealthcareConsideration>
    HealthcareConsideration,
    /// Item is a narcotic as defined by the [1961 UN convention](https://www.incb.org/incb/en/narcotic-drugs/Yellowlist/yellow-list.html), for example marijuana or heroin.
    ///
    /// <https://schema.org/NarcoticConsideration>
    NarcoticConsideration,
    /// A general code for cases where relevance to children is reduced, e.g. adult education, mortgages, retirement-related products, etc.
    ///
    /// <https://schema.org/ReducedRelevanceForChildrenConsideration>
    ReducedRelevanceForChildrenConsideration,
    /// The item contains sexually oriented content such as nudity, suggestive or explicit material, or related online services, or is intended to enhance sexual activity. Examples: Erotic videos or magazine, sexual enhancement devices, sex toys.
    ///
    /// <https://schema.org/SexualContentConsideration>
    SexualContentConsideration,
    /// Item contains tobacco and/or nicotine, for example cigars, cigarettes, chewing tobacco, e-cigarettes, or hookahs.
    ///
    /// <https://schema.org/TobaccoNicotineConsideration>
    TobaccoNicotineConsideration,
    /// The item is suitable only for adults, without indicating why. Due to widespread use of "adult" as a euphemism for "sexual", many such items are likely suited also for the SexualContentConsideration code.
    ///
    /// <https://schema.org/UnclassifiedAdultConsideration>
    UnclassifiedAdultConsideration,
    /// Item shows or promotes violence.
    ///
    /// <https://schema.org/ViolenceConsideration>
    ViolenceConsideration,
    /// The item is intended to induce bodily harm, for example guns, mace, combat knives, brass knuckles, nail or other bombs, and spears.
    ///
    /// <https://schema.org/WeaponConsideration>
    WeaponConsideration,
}
