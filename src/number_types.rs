use json_number::NumberBuf;

#[cfg(feature = "serde")]
mod serde;

#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[cfg_attr(feature = "derive-clone", derive(Clone))]
pub struct Number(pub NumberBuf);

pub type Integer = Number;
