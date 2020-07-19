use super::{assert, *};

use crate::error::image::Error;
use std::convert::TryInto;

#[test]
fn new__with_positive_dimensions_succeeds() {
    // Given
    let width = 37;
    let height = 1;
    let sut = Image::new;

    // When
    let res = sut(width, height);

    // Then
    assert!(
        res == Ok(Image {
            height: NonZeroU32::new(height).unwrap(),
            width: NonZeroU32::new(width).unwrap(),
            buffer: Vec::with_capacity((width * height).try_into().unwrap()),
            pixel_count: usize::try_from(width * height).unwrap(),
        })
    )
}

#[test]
fn new__with_zero_dimensions_fails() {
    // Given
    let width = 37;
    let height = 0;
    let sut = Image::new;

    // When
    let res = sut(width, height);

    // Then
    assert!(res.is_err())
}

#[test]
fn new__with_zero_height_fails() {
    // Given
    let width = 37;
    let height = 0;
    let sut = Image::new;

    // When
    let res = sut(width, height);

    // Then
    assert!(res == Err(Error::ZeroHeightImage))
}

#[test]
fn new__with_zero_width_fails() {
    // Given
    let width = 0;
    let height = 42;
    let sut = Image::new;

    // When
    let res = sut(width, height);

    // Then
    assert!(res == Err(Error::ZeroWidthImage))
}
