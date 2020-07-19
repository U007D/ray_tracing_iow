use super::{assert, *};

#[test]
fn height__accessor_returns_expected_height() {
    // Given
    let height = 42;
    let width = 1;
    let img = Image::new(width, height).unwrap();
    let sut = || img.height();

    // When
    let res = sut();

    // Then
    assert!(res == height)
}
