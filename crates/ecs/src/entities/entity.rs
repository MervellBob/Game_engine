// region: --- Entity ---
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Entity {
    e_id: u32,
    e_gen: u32,
}

impl Entity {
    pub(crate) fn new(id: u32, generation: u32) -> Self {
        Self {
            e_id: id,
            e_gen: generation,
        }
    }
    pub fn e_id(&self) -> u32 {
        self.e_id
    }
    pub(crate) fn e_gen(&self) -> u32 {
        self.e_gen
    }
}

// endregion

// region: --- EntityLocation
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub(crate) struct EntityLocation {
    arch_id: u32,
    row: usize,
}

impl EntityLocation {
    pub(crate) fn new(arch_id: u32, entity_row: usize) -> Self {
        Self {
            arch_id: arch_id,
            row: entity_row,
        }
    }
    pub(crate) fn arch_id(&self) -> u32 {
        self.arch_id
    }
    pub(crate) fn row(&self) -> usize {
        self.row
    }
    pub(crate) fn set_row(&mut self, new_row: usize) {
        self.row = new_row
    }
}

// endregion
