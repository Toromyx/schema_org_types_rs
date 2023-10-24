use json_number::NumberBuf;

#[cfg(feature = "serde")]
mod serde;

#[derive(Debug, Clone)]
pub struct Number(pub NumberBuf);

pub type Integer = Number;
