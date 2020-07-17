use crate::consts::*;
use std::num::NonZeroU32;

#[derive(Debug)]
pub struct Image {}

impl Image {
    #[inline]
    #[must_use]
    pub fn height() -> NonZeroU32 {
        NonZeroU32::new(256).expect(msg::ERR_INTERNAL_MUST_BE_NON_ZERO)
    }

    #[inline]
    #[must_use]
    pub fn width() -> NonZeroU32 {
        NonZeroU32::new(256).expect(msg::ERR_INTERNAL_MUST_BE_NON_ZERO)
    }
}
