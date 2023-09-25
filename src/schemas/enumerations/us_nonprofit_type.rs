/// USNonprofitType: Non-profit organization type originating from the United States.
///
/// https://schema.org/USNonprofitType
#[cfg_attr(any(feature = "derive-debug", doc), derive(Debug))]
#[cfg_attr(any(feature = "derive-clone", doc), derive(Clone))]
#[cfg_attr(
    any(feature = "serde", doc),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum UsNonprofitType {
    /// Nonprofit501a: Non-profit type referring to Farmers’ Cooperative Associations.
    ///
    /// https://schema.org/Nonprofit501a
    Nonprofit501A,
    /// Nonprofit501c1: Non-profit type referring to Corporations Organized Under Act of Congress, including Federal Credit Unions and National Farm Loan Associations.
    ///
    /// https://schema.org/Nonprofit501c1
    Nonprofit501C1,
    /// Nonprofit501c10: Non-profit type referring to Domestic Fraternal Societies and Associations.
    ///
    /// https://schema.org/Nonprofit501c10
    Nonprofit501C10,
    /// Nonprofit501c11: Non-profit type referring to Teachers' Retirement Fund Associations.
    ///
    /// https://schema.org/Nonprofit501c11
    Nonprofit501C11,
    /// Nonprofit501c12: Non-profit type referring to Benevolent Life Insurance Associations, Mutual Ditch or Irrigation Companies, Mutual or Cooperative Telephone Companies.
    ///
    /// https://schema.org/Nonprofit501c12
    Nonprofit501C12,
    /// Nonprofit501c13: Non-profit type referring to Cemetery Companies.
    ///
    /// https://schema.org/Nonprofit501c13
    Nonprofit501C13,
    /// Nonprofit501c14: Non-profit type referring to State-Chartered Credit Unions, Mutual Reserve Funds.
    ///
    /// https://schema.org/Nonprofit501c14
    Nonprofit501C14,
    /// Nonprofit501c15: Non-profit type referring to Mutual Insurance Companies or Associations.
    ///
    /// https://schema.org/Nonprofit501c15
    Nonprofit501C15,
    /// Nonprofit501c16: Non-profit type referring to Cooperative Organizations to Finance Crop Operations.
    ///
    /// https://schema.org/Nonprofit501c16
    Nonprofit501C16,
    /// Nonprofit501c17: Non-profit type referring to Supplemental Unemployment Benefit Trusts.
    ///
    /// https://schema.org/Nonprofit501c17
    Nonprofit501C17,
    /// Nonprofit501c18: Non-profit type referring to Employee Funded Pension Trust (created before 25 June 1959).
    ///
    /// https://schema.org/Nonprofit501c18
    Nonprofit501C18,
    /// Nonprofit501c19: Non-profit type referring to Post or Organization of Past or Present Members of the Armed Forces.
    ///
    /// https://schema.org/Nonprofit501c19
    Nonprofit501C19,
    /// Nonprofit501c2: Non-profit type referring to Title-holding Corporations for Exempt Organizations.
    ///
    /// https://schema.org/Nonprofit501c2
    Nonprofit501C2,
    /// Nonprofit501c20: Non-profit type referring to Group Legal Services Plan Organizations.
    ///
    /// https://schema.org/Nonprofit501c20
    Nonprofit501C20,
    /// Nonprofit501c21: Non-profit type referring to Black Lung Benefit Trusts.
    ///
    /// https://schema.org/Nonprofit501c21
    Nonprofit501C21,
    /// Nonprofit501c22: Non-profit type referring to Withdrawal Liability Payment Funds.
    ///
    /// https://schema.org/Nonprofit501c22
    Nonprofit501C22,
    /// Nonprofit501c23: Non-profit type referring to Veterans Organizations.
    ///
    /// https://schema.org/Nonprofit501c23
    Nonprofit501C23,
    /// Nonprofit501c24: Non-profit type referring to Section 4049 ERISA Trusts.
    ///
    /// https://schema.org/Nonprofit501c24
    Nonprofit501C24,
    /// Nonprofit501c25: Non-profit type referring to Real Property Title-Holding Corporations or Trusts with Multiple Parents.
    ///
    /// https://schema.org/Nonprofit501c25
    Nonprofit501C25,
    /// Nonprofit501c26: Non-profit type referring to State-Sponsored Organizations Providing Health Coverage for High-Risk Individuals.
    ///
    /// https://schema.org/Nonprofit501c26
    Nonprofit501C26,
    /// Nonprofit501c27: Non-profit type referring to State-Sponsored Workers' Compensation Reinsurance Organizations.
    ///
    /// https://schema.org/Nonprofit501c27
    Nonprofit501C27,
    /// Nonprofit501c28: Non-profit type referring to National Railroad Retirement Investment Trusts.
    ///
    /// https://schema.org/Nonprofit501c28
    Nonprofit501C28,
    /// Nonprofit501c3: Non-profit type referring to Religious, Educational, Charitable, Scientific, Literary, Testing for Public Safety, Fostering National or International Amateur Sports Competition, or Prevention of Cruelty to Children or Animals Organizations.
    ///
    /// https://schema.org/Nonprofit501c3
    Nonprofit501C3,
    /// Nonprofit501c4: Non-profit type referring to Civic Leagues, Social Welfare Organizations, and Local Associations of Employees.
    ///
    /// https://schema.org/Nonprofit501c4
    Nonprofit501C4,
    /// Nonprofit501c5: Non-profit type referring to Labor, Agricultural and Horticultural Organizations.
    ///
    /// https://schema.org/Nonprofit501c5
    Nonprofit501C5,
    /// Nonprofit501c6: Non-profit type referring to Business Leagues, Chambers of Commerce, Real Estate Boards.
    ///
    /// https://schema.org/Nonprofit501c6
    Nonprofit501C6,
    /// Nonprofit501c7: Non-profit type referring to Social and Recreational Clubs.
    ///
    /// https://schema.org/Nonprofit501c7
    Nonprofit501C7,
    /// Nonprofit501c8: Non-profit type referring to Fraternal Beneficiary Societies and Associations.
    ///
    /// https://schema.org/Nonprofit501c8
    Nonprofit501C8,
    /// Nonprofit501c9: Non-profit type referring to Voluntary Employee Beneficiary Associations.
    ///
    /// https://schema.org/Nonprofit501c9
    Nonprofit501C9,
    /// Nonprofit501d: Non-profit type referring to Religious and Apostolic Associations.
    ///
    /// https://schema.org/Nonprofit501d
    Nonprofit501D,
    /// Nonprofit501e: Non-profit type referring to Cooperative Hospital Service Organizations.
    ///
    /// https://schema.org/Nonprofit501e
    Nonprofit501E,
    /// Nonprofit501f: Non-profit type referring to Cooperative Service Organizations.
    ///
    /// https://schema.org/Nonprofit501f
    Nonprofit501F,
    /// Nonprofit501k: Non-profit type referring to Child Care Organizations.
    ///
    /// https://schema.org/Nonprofit501k
    Nonprofit501K,
    /// Nonprofit501n: Non-profit type referring to Charitable Risk Pools.
    ///
    /// https://schema.org/Nonprofit501n
    Nonprofit501N,
    /// Nonprofit501q: Non-profit type referring to Credit Counseling Organizations.
    ///
    /// https://schema.org/Nonprofit501q
    Nonprofit501Q,
    /// Nonprofit527: Non-profit type referring to political organizations.
    ///
    /// https://schema.org/Nonprofit527
    Nonprofit527,
}
