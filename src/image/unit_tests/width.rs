use super::{assert, *};

#[test]
fn width__accessor_returns_expected_width() {
    // Given
    let height = 42;
    let width = 999;
    let img = Image::new(width, height).unwrap();
    let sut = || img.width();

    // When
    let res = sut();

    // Then
    assert!(res == width)
}
