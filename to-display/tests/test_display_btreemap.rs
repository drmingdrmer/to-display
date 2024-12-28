use std::collections::BTreeMap;

use to_display::DisplayConfig;
use to_display::ToDisplay;

#[test]
fn test_display_btreemap() {
    let mut map = BTreeMap::new();
    map.insert(Some(1u64), vec![1u32]);
    map.insert(Some(2), vec![2]);
    map.insert(None, vec![3]);

    let display = map.display().verbose();
    assert_eq!(
        display.to_string(),
        "{None: [3], Some(1): [1], Some(2): [2]}"
    );

    let display = display.limit_items(2);
    assert_eq!(display.to_string(), "{None: [3], Some(1): [1], ...}");
}
