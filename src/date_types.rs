#[cfg(feature = "serde")]
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Date(pub speedate::Date);

#[cfg(feature = "serde")]
impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl FromStr for Date {
    type Err = speedate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(speedate::Date::parse_str_rfc3339(s)?))
    }
}

#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Time(pub speedate::Time);

#[cfg(feature = "serde")]
impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl FromStr for Time {
    type Err = speedate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(speedate::Time::parse_str(s)?))
    }
}

#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct DateTime(pub speedate::DateTime);

#[cfg(feature = "serde")]
impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl FromStr for DateTime {
    type Err = speedate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(speedate::DateTime::parse_str_rfc3339(s)?))
    }
}
