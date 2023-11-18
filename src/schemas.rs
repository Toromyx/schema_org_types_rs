#![allow(unused)]
#![allow(clippy::enum_variant_names)]
// TODO does this cause no errors in using crates?
#![allow(deprecated)]

mod classes;
mod data_types;
mod enumerations;
mod properties;

pub use classes::*;
pub use data_types::*;
pub use enumerations::*;
pub use properties::*;
