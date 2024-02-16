use embassy_nrf::saadc;

/// Adc alias to unify v1 and v2 names
pub type Adc = saadc::Saadc<'static, 1>;
/// AdcConfig alias to unify v1 and v2 names
pub type AdcConfig = saadc::Config;
