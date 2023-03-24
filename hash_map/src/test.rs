use super::HMap;

#[test]
fn test_get_right_values() {
    let mut map = HMap::new();
    map.insert("keith".to_string(), 18);
    map.insert("dave".to_string(), 1);
    map.insert("andy".to_string(), 9);
    map.insert("pete".to_string(), 88);
    map.insert("jane".to_string(), 22);
    map.insert("sam".to_string(), 2);
    map.insert("andrew".to_string(), 99);

    assert_eq!(map.get("andrew"), Some(&99));
}

#[test]
fn test_update_right_values() {
    let mut map = HMap::new();
    map.insert("keith".to_string(), 18);
    map.insert("dave".to_string(), 1);
    map.insert("andy".to_string(), 9);
    map.insert("pete".to_string(), 88);
    map.insert("jane".to_string(), 22);
    map.insert("sam".to_string(), 2);
    map.insert("andrew".to_string(), 99);

    assert_eq!(map.get("keith"), Some(&18));
    map.insert("keith".to_string(), 27);
    assert_eq!(map.get("keith"), Some(&27));
}

#[test]
fn test_map_len() {
    let mut map = HMap::new();
    map.insert("keith".to_string(), 18);
    map.insert("dave".to_string(), 1);
    map.insert("andy".to_string(), 9);
    map.insert("pete".to_string(), 88);
    map.insert("jane".to_string(), 22);
    map.insert("sam".to_string(), 2);
    map.insert("andrew".to_string(), 99);

    assert_eq!(map.len(), 7);
}

#[test]
fn test_lots_of_numbers() {
    let mut map = HMap::new();
    for x in 0..10_000 {
        map.insert(x, x + 250);
    }

    assert_eq!(map.len(), 10_000);
    assert_eq!(map.get(&500), Some(&750));

    for (i, b) in map.main.buckets.iter().enumerate() {
        assert!(b.len() < 10, "main bucket[{i}] too big: {}", b.len());
    }
    for (i, b) in map.grow.buckets.iter().enumerate() {
        assert!(b.len() < 10, "grow bucket[{i}] too big: {}", b.len());
    }
}
