pub const ERR_16_BIT_PROCESSOR: &str =
    "Error: 16-bit processors are not supported by this application";
pub const ERR_ARG_NOT_CONVERTIBLE_TO_UTF_8: &str =
    "Error: supplied command-line argument not convertible to UTF-8";
pub const ERR_IMAGE_TOO_LARGE: &str = "Error: The product of `width` x `height` exceeds the capacity of a `u32`.  Please choose smaller dimensions";
pub const ERR_INDEX_OUT_OF_BOUNDS: &str = "Error: Provided index is out of bounds";
pub const ERR_INTERNAL_MUST_BE_NON_ZERO: &str = "Internal Error: `NonZero` type received a 0 value";
pub const ERR_ZERO_HEIGHT_IMAGE: &str =
    "Error: Attempted to created image with zero height.  Height must be > 0";
pub const ERR_ZERO_WIDTH_IMAGE: &str =
    "Error: Attempted to created image with zero width.  Width must be > 0";
