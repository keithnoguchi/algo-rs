use super::Store;

use std::fs;

#[test]
fn test_create_file() {
    let file = "test_create_file";
    fs::remove_file(file).ok();
    let bs = Store::new_or_open(file, 1000, 10).unwrap();
    let blocksize = bs.block_size;

    let bs2 = Store::open(file).unwrap();
    assert_eq!(bs2.block_size, blocksize);
}

#[test]
fn test_insert_only() {
    let file = "test_insert_only";
    fs::remove_file(file).ok();
    let bs = Store::new_or_open(file, 1000, 10).unwrap();
    let blocksize = bs.block_size;

    let mut bs2 = Store::open(file).unwrap();
    assert_eq!(bs2.block_size, blocksize);

    bs2.insert_only("fish", "so long and thanks for all the fish")
        .unwrap();
    bs2.insert_only(23, "a big number for small counters")
        .unwrap();
    bs2.insert_only("green", "is a colour i guess").unwrap();
    bs2.insert_only("happy", "is friends with sleepy").unwrap();
    drop(bs2);

    let mut b3 = Store::open(file).unwrap();
    assert_eq!(
        b3.get::<&str, String>(&"green")
            .unwrap()
            .get_v::<String>()
            .unwrap(),
        "is a colour i guess".to_string(),
    );
    b3.remove(&"green").ok();
    assert!(b3.get::<&str, String>(&"green").is_err());
    assert!(b3.get::<&str, String>(&"fish").is_ok());
}
