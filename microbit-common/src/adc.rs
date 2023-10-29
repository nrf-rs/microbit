//! ADC
#[cfg(feature = "v1")]
pub use crate::v1::adc::*;

#[cfg(feature = "v2")]
pub use crate::v2::adc::*;
