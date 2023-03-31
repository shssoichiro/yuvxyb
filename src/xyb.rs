use v_frame::prelude::Pixel;

use crate::{rgb_xyb::linear_rgb_to_xyb, ConversionError, CreationError, LinearRgb, Rgb, Yuv};

#[derive(Debug, Clone)]
pub struct Xyb {
    data: Vec<[f32; 3]>,
    width: usize,
    height: usize,
}

impl Xyb {
    /// # Errors
    /// - If data length does not match `width * height`
    pub fn new(data: Vec<[f32; 3]>, width: usize, height: usize) -> Result<Self, CreationError> {
        if data.len() != width * height {
            return Err(CreationError::ResolutionMismatch);
        }

        Ok(Self {
            data,
            width,
            height,
        })
    }

    #[must_use]
    #[inline(always)]
    pub fn data(&self) -> &[[f32; 3]] {
        &self.data
    }

    #[must_use]
    #[inline(always)]
    pub fn data_mut(&mut self) -> &mut [[f32; 3]] {
        &mut self.data
    }

    #[must_use]
    #[inline(always)]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_data(self) -> Vec<[f32; 3]> {
        self.data
    }

    #[must_use]
    #[inline(always)]
    pub const fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    #[inline(always)]
    pub const fn height(&self) -> usize {
        self.height
    }
}

impl<T: Pixel> TryFrom<Yuv<T>> for Xyb {
    type Error = ConversionError;

    fn try_from(yuv: Yuv<T>) -> Result<Self, Self::Error> {
        Self::try_from(&yuv)
    }
}

impl<T: Pixel> TryFrom<&Yuv<T>> for Xyb {
    type Error = ConversionError;

    fn try_from(yuv: &Yuv<T>) -> Result<Self, Self::Error> {
        let rgb = Rgb::try_from(yuv)?;
        Self::try_from(rgb)
    }
}

impl TryFrom<Rgb> for Xyb {
    type Error = ConversionError;

    fn try_from(rgb: Rgb) -> Result<Self, Self::Error> {
        let lrgb = LinearRgb::try_from(rgb)?;
        Ok(Self::from(lrgb))
    }
}

impl From<LinearRgb> for Xyb {
    fn from(lrgb: LinearRgb) -> Self {
        let width = lrgb.width();
        let height = lrgb.height();
        let data = linear_rgb_to_xyb(lrgb.into_data());

        Self {
            data,
            width,
            height,
        }
    }
}
