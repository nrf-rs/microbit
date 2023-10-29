use crate::hal;

/// Adc alias to unify v1 and v2 names
pub type Adc = hal::Adc;
/// AdcConfig alias to unify v1 and v2 names
pub type AdcConfig = hal::adc::AdcConfig;

/// Same resolution for v1 and v2
pub trait Default {
    /// v1 is limited to 10 bit
    fn default_10bit() -> Self;
}

impl Default for AdcConfig {
    fn default_10bit() -> Self {
        AdcConfig::default()
    }
}
