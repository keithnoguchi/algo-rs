use super::*;

#[test]
fn items_drop() {
    let mut gm = GenManager::new();

    let g = gm.next_gen();
    assert_eq!(g, GenData { gen: 0, pos: 0 });
    let g2 = gm.next_gen();
    gm.next_gen();
    gm.next_gen();
    gm.next_gen();
    gm.drop(g2);
    let g3 = gm.next_gen();
    assert_eq!(g3, GenData { gen: 1, pos: 1 });
}
