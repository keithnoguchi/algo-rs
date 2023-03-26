use super::Blob;

use std::fs;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Point<T> {
    x: T,
    y: T,
}

#[test]
fn test_write_read_str() {
    let test = "t_read_write_str";
    fs::remove_file(test).ok();
    let k = 87_i32;
    let v = "hello world";
    let blob = Blob::from(&k, &v).unwrap();
    {
        let mut fout = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(test)
            .unwrap();
        blob.write(&mut fout).unwrap();
    }
    let mut fin = fs::File::open(test).unwrap();
    let b2 = Blob::read(&mut fin).unwrap();
    let v2: String = b2.get_v().unwrap();
    assert_eq!(v2, v);

    let p: Point<i32> = b2.get_v().unwrap();
    assert_eq!(p, Point { x: 11, y: 0 });
}
