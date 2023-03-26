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
}
