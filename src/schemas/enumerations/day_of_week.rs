/// The day of the week, e.g. used to specify to which day the opening hours of an OpeningHoursSpecification refer.
///
/// Originally, URLs from [GoodRelations](http://purl.org/goodrelations/v1) were used (for [[Monday]], [[Tuesday]], [[Wednesday]], [[Thursday]], [[Friday]], [[Saturday]], [[Sunday]] plus a special entry for [[PublicHolidays]]); these have now been integrated directly into schema.org.
///
///
/// https://schema.org/DayOfWeek
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DayOfWeek {
    /// The day of the week between Thursday and Saturday.
    ///
    /// https://schema.org/Friday
    Friday,
    /// The day of the week between Sunday and Tuesday.
    ///
    /// https://schema.org/Monday
    Monday,
    /// This stands for any day that is a public holiday; it is a placeholder for all official public holidays in some particular location. While not technically a "day of the week", it can be used with [[OpeningHoursSpecification]]. In the context of an opening hours specification it can be used to indicate opening hours on public holidays, overriding general opening hours for the day of the week on which a public holiday occurs.
    ///
    /// https://schema.org/PublicHolidays
    PublicHolidays,
    /// The day of the week between Friday and Sunday.
    ///
    /// https://schema.org/Saturday
    Saturday,
    /// The day of the week between Saturday and Monday.
    ///
    /// https://schema.org/Sunday
    Sunday,
    /// The day of the week between Wednesday and Friday.
    ///
    /// https://schema.org/Thursday
    Thursday,
    /// The day of the week between Monday and Wednesday.
    ///
    /// https://schema.org/Tuesday
    Tuesday,
    /// The day of the week between Tuesday and Thursday.
    ///
    /// https://schema.org/Wednesday
    Wednesday,
}
