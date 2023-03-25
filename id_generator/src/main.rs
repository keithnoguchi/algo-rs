//! An ID generator.

#![forbid(unsafe_code, missing_debug_implementations)]

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenData {
    pos: usize,
    gen: u64,
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

    pub fn drop(&mut self, data: GenData) {
        if let Some(entity) = self.items.get_mut(data.pos) {
            if entity.active && entity.gen == data.gen {
                entity.active = false;
                self.drops.push(data.pos);
            }
        }
    }
}

#[cfg(test)]
mod test;

fn main() {}
