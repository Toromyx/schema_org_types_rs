use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::date_types::{Date, DateTime, Time};

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Date {
    type Err = speedate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(speedate::Date::parse_str_rfc3339(s)?))
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Time {
    type Err = speedate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(speedate::Time::parse_str(s)?))
    }
}

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for DateTime {
    type Err = speedate::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(speedate::DateTime::parse_str_rfc3339(s)?))
    }
}
