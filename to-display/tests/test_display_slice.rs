use to_display::DisplayConfig;
use to_display::ToDisplay;

#[test]
fn test_display_slice() {
    let foo = [1, 2, 3];

    assert_eq!(foo.display().to_string(), "[1, 2, 3]");
    assert_eq!(foo.display().limit_items(2).to_string(), "[1, 2, ...]");

    let foo = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, //
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, //
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, //
        1, 2, 3,
    ];
    assert_eq!(foo.display().to_string(), "[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 1, 2, ...]");
}

#[test]
fn test_display_vec() {
    let v = vec![1];
    assert_eq!(v.display().to_string(), "[1]");
}
