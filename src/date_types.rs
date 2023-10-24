#[cfg(feature = "serde")]
mod serde;

#[derive(Debug, Clone)]
pub struct Date(pub speedate::Date);

#[derive(Debug, Clone)]
pub struct Time(pub speedate::Time);

#[derive(Debug, Clone)]
pub struct DateTime(pub speedate::DateTime);
