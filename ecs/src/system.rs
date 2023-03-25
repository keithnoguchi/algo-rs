use crate::EcsStore;
use crate::data::*;

pub fn move_sys<D: EcsStore<Dir>, P: EcsStore<Pos>>(dd: &D, pp: &mut P) {
    pp.for_each_mut(|g, p| {
        if let Some(d) = dd.get(g) {
            p.x += d.vx;
            p.y += d.vy;
        }
    });
}
