use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
use embedded_hal::spi::SpiDevice;
use crate::config::{NrfConfig, PayloadSize};
use crate::error::TransceiverError;
use crate::register_acces::Register;

pub struct Sync;
pub struct Async;

/// The nRF24L01 driver type. This struct encapsulates all functionality.
///
/// For the different configuration options see: [`NrfConfig`].
///
/// # Examples
/// ```
/// use nrf24::Nrf24l01;
/// use nrf24::config::NrfConfig;
///
/// // Initialize the chip with default configuration.
/// let nrf24 = Nrf24l01::new(spi, ce, &mut delay, NrfConfig::default()).unwrap();
///
/// ```
pub struct Nrf24l01<SPI, CE, Mode> {
    spi: SPI,
    ce: CE,
    // Config Register
    config_reg: u8,
    // Payload size
    payload_size: PayloadSize,
    _mode: PhantomData<Mode>,
}

// Associated type alias to simplify our result types.
#[cfg(not(feature = "async"))]
type NrfResult<T, SPI, CE> =
Result<T, TransceiverError<<SPI as embedded_hal::spi::ErrorType>::Error, <CE as embedded_hal::digital::ErrorType>::Error>>;

#[cfg(feature = "async")]
type NrfResult<T, SPI, CE> =
Result<T, TransceiverError<<SPI as embedded_hal_async::spi::ErrorType>::Error, <CE as embedded_hal::digital::ErrorType>::Error>>;



#[cfg(not(feature = "async"))]
pub mod sync;
#[cfg(feature = "async")]
pub mod asynch;

impl<SPI, CE> Nrf24l01<SPI, CE, Sync>
where
    SPI: SpiDevice,
    CE: OutputPin
{
 
}