use to_display::DisplayConfig;
use to_display::ToDisplay;

#[test]
fn test_display_result() {
    let result: Result<i32, &str> = Ok(42);
    assert_eq!(result.display().to_string(), "Ok(42)");

    let result: Result<i32, &str> = Err("error");
    assert_eq!(result.display().to_string(), "Err(error)");

    let result: Result<Option<i32>, &str> = Ok(Some(42));
    assert_eq!(result.display().to_string(), "Ok(42)");

    let result: Result<Option<i32>, &str> = Ok(Some(42));
    assert_eq!(result.display().verbose().to_string(), "Ok(Some(42))");
}
