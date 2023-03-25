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

#[test]
fn store_can_drop() {
    let mut gm = GenManager::new();
    let mut store = VecStore::new();

    store.add(gm.next_gen(), 5);
    store.add(gm.next_gen(), 3);
    store.add(gm.next_gen(), 2);

    let g4 = gm.next_gen();

    store.add(g4, 5);

    store.for_each_mut(|_g, d| *d += 2);

    assert_eq!(store.get(g4), Some(&7));

    store.drop(g4);

    assert_eq!(store.get(g4), None);
}
