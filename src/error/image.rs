use crate::consts::msg;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, Error, PartialEq)]
pub enum Error {
    #[error("{}: {} x {}", msg::ERR_IMAGE_TOO_LARGE, 0, 1)]
    ImagePixelCountOverflow(u32, u32),
    #[error(transparent)]
    TryFromIntError(#[from] std::num::TryFromIntError),
    #[error("{}", msg::ERR_ZERO_HEIGHT_IMAGE)]
    ZeroHeightImage,
    #[error("{}", msg::ERR_ZERO_WIDTH_IMAGE)]
    ZeroWidthImage,
}
