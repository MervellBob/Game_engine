use std::any::{Any, TypeId};

use crate::{components::bundle::Bundle, entities::entity::Entity, world::world::World};

pub struct EntityEditor<'a> {
    world: &'a mut World,
    entity: Entity,
    add: Vec<(TypeId, Box<dyn Any>)>,
    remove: Vec<TypeId>,
}

impl<'a> EntityEditor<'a> {
    pub(crate) fn new(entity: Entity, world: &'a mut World) -> Self {
        Self {
            entity,
            world,
            add: Vec::new(),
            remove: Vec::new(),
        }
    }

    pub fn insert<B: Bundle>(mut self, bundle: B) -> Self {
        B::register_components(&mut |type_id, factory| {
            self.world.register_component(type_id, factory);
        });
        bundle.get_components(&mut |type_id, comp| {
            self.add.push((type_id, comp));
        });
        self
    }

    pub fn remove_components<T: Bundle>(mut self) -> Self {
        self.remove = T::get_signature();
        self
    }

    pub fn apply(self) {
        self.world.modify_entity(self.entity, self.add, self.remove);
    }

    pub fn delete(self) {
        self.world.delete_entity(self.entity);
    }
}
