use crate::hal;

/// Adc alias to unify v1 and v2 names
pub type Adc = hal::Saadc;
/// AdcConfig alias to unify v1 and v2 names
pub type AdcConfig = hal::saadc::SaadcConfig;

/// Same resolution for v1 and v2
pub trait Default {
    /// v1 is limited to 10 bit
    fn default_10bit() -> Self;
}

impl Default for AdcConfig {
    fn default_10bit() -> Self {
        AdcConfig {
            resolution: hal::saadc::Resolution::_10BIT,
            ..AdcConfig::default()
        }
    }
}
