use crate::{
    consts::msg,
    error::image::{Error, Result},
    Color,
};
use interpol::format;
use std::convert::TryFrom;
use std::num::NonZeroU32;

#[cfg(test)]
mod unit_tests;

#[derive(Clone, Debug, PartialEq)]
pub struct Image {
    buffer: Vec<Color>,
    height: NonZeroU32,
    pixel_count: usize,
    width: NonZeroU32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let pixel_count = usize::try_from(
            width
                .checked_mul(height)
                .ok_or(Error::ImagePixelCountOverflow(width, height))?,
        )
        .expect(msg::ERR_16_BIT_PROCESSOR);

        Ok(Self {
            height: NonZeroU32::new(height).ok_or(Error::ZeroHeightImage)?,
            width: NonZeroU32::new(width).ok_or(Error::ZeroWidthImage)?,
            buffer: vec![Color::default(); pixel_count],
            pixel_count,
        })
    }

    #[must_use]
    #[inline]
    pub fn height(&self) -> u32 {
        self.height.into()
    }

    // x and y are constrained to the bounds of the `Image` and the `Image`'s constructor assures
    // `width` x `height` does not overflow a `u32`, so `x` and `y` are safe to multiply & index.
    #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
    #[inline]
    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> &mut Self {
        debug_assert!(x < self.width.into());
        debug_assert!(y < self.height.into());

        let idx = usize::try_from(self.width() * y + x).expect(msg::ERR_16_BIT_PROCESSOR);
        self.buffer[idx] = color;
        self
    }

    #[must_use]
    #[inline]
    pub fn width(&self) -> u32 {
        self.width.into()
    }

    // x and y are constrained to the bounds of the `Image` and the `Image`'s constructor assures
    // `width` x `height` does not overflow a `u32`, so `x` and `y` are safe to multiply & index.
    #[allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
    #[must_use]
    pub fn serialize_to_ppm(&self) -> String {
        let mut builder = format!("P3\n{self.width()} {self.height()}\n255");

        (0..self.height()).for_each(|y| {
            (0..self.width()).for_each(|x| {
                let pixel = self.buffer
                    [usize::try_from(self.width() * y + x).expect(msg::ERR_16_BIT_PROCESSOR)];
                builder += &format!("\n{pixel}");
            })
        });
        builder
    }
}
