//! Static 5×5 greyscale and black-and-white images.

use tiny_led_matrix::{Render, MAX_BRIGHTNESS};

/// A 5×5 image supporting the full range of brightnesses for each LED.
///
/// Uses 25 bytes of storage.
#[derive(Copy, Clone, Debug)]
pub struct GreyscaleImage([[u8; 5]; 5]);

impl GreyscaleImage {
    /// Constructs a GreyscaleImage from an array of brightnesses.
    ///
    /// The data should be an array of 5 rows (top first), each of which is an
    /// array of 5 brightness values (left first).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use microbit_common as microbit;
    /// # use microbit::display::nonblocking::GreyscaleImage;
    /// const GREY_HEART: GreyscaleImage = GreyscaleImage::new(&[
    ///     [0, 9, 0, 9, 0],
    ///     [9, 5, 9, 5, 9],
    ///     [9, 5, 5, 5, 9],
    ///     [0, 9, 5, 9, 0],
    ///     [0, 0, 9, 0, 0],
    /// ]);
    /// ```
    pub const fn new(data: &[[u8; 5]; 5]) -> GreyscaleImage {
        GreyscaleImage(*data)
    }

    /// Construct a GreyscaleImage with all LEDs turned off.
    pub const fn blank() -> GreyscaleImage {
        GreyscaleImage([[0; 5]; 5])
    }
}

impl Render for GreyscaleImage {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        self.0[y][x]
    }
}

impl Render for &GreyscaleImage {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        GreyscaleImage::brightness_at(self, x, y)
    }
}

/// A 5×5 image supporting only two levels of brightness (on and off).
///
/// Uses 5 bytes of storage.
///
/// For display, each pixel is treated as having brightness either 0 or
/// MAX_BRIGHTNESS.
#[derive(Copy, Clone, Debug)]
pub struct BitImage([u8; 5]);

impl BitImage {
    /// Constructs a BitImage from an array of brightnesses.
    ///
    /// The data should be an array of 5 rows (top first), each of which is an
    /// array of 5 values (left first). Each value should be either 0 or 1.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use microbit_common as microbit;
    /// # use microbit::display::nonblocking::BitImage;
    /// const HEART: BitImage = BitImage::new(&[
    ///     [0, 1, 0, 1, 0],
    ///     [1, 0, 1, 0, 1],
    ///     [1, 0, 0, 0, 1],
    ///     [0, 1, 0, 1, 0],
    ///     [0, 0, 1, 0, 0],
    /// ]);
    /// ```
    pub const fn new(im: &[[u8; 5]; 5]) -> BitImage {
        // FIXME: can we reject values other than 0 or 1?
        const fn row_byte(row: [u8; 5]) -> u8 {
            row[0] | row[1] << 1 | row[2] << 2 | row[3] << 3 | row[4] << 4
        }
        BitImage([
            row_byte(im[0]),
            row_byte(im[1]),
            row_byte(im[2]),
            row_byte(im[3]),
            row_byte(im[4]),
        ])
    }

    /// Returns a new blank BitImage.
    ///
    /// All pixel values are 0.
    pub const fn blank() -> BitImage {
        BitImage([0; 5])
    }
}

impl Render for BitImage {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        let rowdata = self.0[y];
        if rowdata & (1 << x) != 0 {
            MAX_BRIGHTNESS
        } else {
            0
        }
    }
}

impl Render for &BitImage {
    fn brightness_at(&self, x: usize, y: usize) -> u8 {
        BitImage::brightness_at(self, x, y)
    }
}
