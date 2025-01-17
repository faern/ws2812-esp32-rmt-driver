use crate::driver::color::LedPixelColor;
use crate::driver::color::LedPixelColorGrb24;

/// WS2812 ESP32 RMT Driver error.
#[derive(thiserror::Error, Debug)]
#[error("mock Ws2812Esp32RmtDriverError")]
pub struct Ws2812Esp32RmtDriverError;

/// Mock of Low-level WS2812 ESP32 RMT driver.
///
/// If the target vendor does not equals to "espressif", this mock is used instead of genuine
/// Low-level WS2812 ESP32 RMT driver.
pub struct Ws2812Esp32RmtDriver {
    /// Pixel binary array to be written
    pub grb_pixels: Option<Vec<u8>>,
    /// Whether wait for tx done (does not work on the mock!)
    pub wait_tx_done: bool,
}

impl Ws2812Esp32RmtDriver {
    /// Creates a mock of `Ws2812Esp32RmtDriver`.
    /// All arguments shall be ignored and always returns `Ok(_)`.
    pub fn new(_channel_num: u8, _gpio_num: u32) -> Result<Self, Ws2812Esp32RmtDriverError> {
        Ok(Self {
            grb_pixels: None,
            wait_tx_done: true,
        })
    }

    /// Writes GRB pixel binary slice.
    pub fn write(&mut self, grb_pixels: &[u8]) -> Result<(), Ws2812Esp32RmtDriverError> {
        self.grb_pixels = Some(grb_pixels.to_vec());
        Ok(())
    }

    /// Writes GRB pixel binary with converting into `LedPixelColorGrg24`.
    pub fn write_colors<I>(&mut self, iterator: I) -> Result<(), Ws2812Esp32RmtDriverError>
    where
        I: IntoIterator<Item = LedPixelColorGrb24>,
    {
        let mut vec = Vec::new();
        for color in iterator {
            for v in color.as_ref() {
                vec.push(*v);
            }
        }
        self.grb_pixels = Some(vec);
        Ok(())
    }
}

#[test]
fn test_ws2812_esp32_rmt_driver_mock() {
    let sample_data: [u8; 6] = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05];

    let mut driver = Ws2812Esp32RmtDriver::new(0, 27).unwrap();
    assert_eq!(driver.grb_pixels, None);
    driver.write(&sample_data).unwrap();
    assert_eq!(driver.grb_pixels.unwrap(), &sample_data);

    let mut driver = Ws2812Esp32RmtDriver::new(0, 27).unwrap();
    let colors = [
        LedPixelColorGrb24::new_with_rgb(1, 2, 3),
        LedPixelColorGrb24::new_with_rgb(4, 5, 6),
    ];
    driver.write_colors(colors).unwrap();
    assert_eq!(driver.grb_pixels.unwrap(), vec![2, 1, 3, 5, 4, 6]);
}
