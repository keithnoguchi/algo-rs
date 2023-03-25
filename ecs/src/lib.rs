//! An ECS systems and ID generator.

#![forbid(unsafe_code, missing_debug_implementations)]

pub mod data;
pub mod system;

pub trait EcsStore<T> {
    fn add(&mut self, g: GenData, t: T);
    fn get(&self, g: GenData) -> Option<&T>;
    fn get_mut(&mut self, g: GenData) -> Option<&mut T>;
    fn drop(&mut self, g: GenData);

    fn for_each<F: FnMut(GenData, &T)>(&self, f: F);
    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, f: F);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenData {
    pub pos: usize,
    pub gen: u64,
}

#[derive(Debug)]
pub struct EntityActive {
    active: bool,
    gen: u64,
}

#[derive(Debug, Default)]
pub struct GenManager {
    items: Vec<EntityActive>,
    drops: Vec<usize>,
}

impl GenManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn next_gen(&mut self) -> GenData {
        if let Some(loc) = self.drops.pop() {
            let entity = &mut self.items[loc];
            entity.active = true;
            entity.gen += 1;
            return GenData {
                pos: loc,
                gen: entity.gen,
            };
        }
        // nothing left in drops, add on the end.
        self.items.push(EntityActive {
            active: true,
            gen: 0,
        });
        GenData {
            gen: 0,
            pos: self.items.len() - 1,
        }
    }

    pub fn drop(&mut self, g: GenData) {
        if let Some(entity) = self.items.get_mut(g.pos) {
            if entity.active && entity.gen == g.gen {
                entity.active = false;
                self.drops.push(g.pos);
            }
        }
    }
}

#[derive(Debug)]
pub struct VecStore<T> {
    items: Vec<Option<(u64, T)>>,
}

impl<T> EcsStore<T> for VecStore<T> {
    fn add(&mut self, g: GenData, data: T) {
        while g.pos >= self.items.len() {
            self.items.push(None);
        }
        self.items[g.pos] = Some((g.gen, data));
    }

    fn get(&self, g: GenData) -> Option<&T> {
        if let Some(Some((gen, data))) = self.items.get(g.pos) {
            if *gen == g.gen {
                return Some(data);
            }
        }
        None
    }

    fn get_mut(&mut self, g: GenData) -> Option<&mut T> {
        if let Some(Some((gen, data))) = self.items.get_mut(g.pos) {
            if *gen == g.gen {
                return Some(data);
            }
        }
        None
    }

    fn drop(&mut self, g: GenData) {
        if let Some(Some((gen, _))) = self.items.get(g.pos) {
            if *gen == g.gen {
                self.items[g.pos] = None;
            }
        }
    }

    fn for_each<F: FnMut(GenData, &T)>(&self, mut f: F) {
        for (n, x) in self.items.iter().enumerate() {
            if let Some((g, d)) = x {
                f(GenData { gen: *g, pos: n }, d);
            }
        }
    }

    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut f: F) {
        for (n, x) in self.items.iter_mut().enumerate() {
            if let Some((g, d)) = x {
                f(GenData { gen: *g, pos: n }, d);
            }
        }
    }
}

impl<T> Default for VecStore<T> {
    fn default() -> Self {
        Self { items: vec![] }
    }
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod test;
