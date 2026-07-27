use crate::entities::entity::Entity;

pub(crate) struct EntityManager {
    next_id: u32,
    free_ids: Vec<u32>,
    generations: Vec<u32>,
}

impl EntityManager {
    pub(crate) fn new() -> Self {
        Self {
            next_id: 0,
            free_ids: Vec::new(),
            generations: Vec::new(),
        }
    }

    pub(crate) fn create_entity(&mut self) -> Entity {
        if !self.free_ids.is_empty() {
            let reuse_id = self.free_ids.pop().expect("free_ids should not be empty");
            self.generations[reuse_id as usize] += 1;
            let new_entity = Entity::new(reuse_id, self.generations[reuse_id as usize]);
            return new_entity;
        } else {
            self.generations.push(0);
            let e = Entity::new(self.next_id, 0);
            self.next_id += 1;
            e
        }
    }

    pub(crate) fn recycle_entity(&mut self, entity: Entity) {
        self.free_ids.push(entity.e_id());
    }

    pub(crate) fn has_correct_gen(&self, entity: Entity) -> bool {
        self.generations[entity.e_id() as usize] == entity.e_gen()
    }
}
