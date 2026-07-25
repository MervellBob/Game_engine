use crate::{components::component_column::AnyColumn, entities::entity::Entity};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    ptr::NonNull,
};

pub(crate) struct Archetype {
    entities: Vec<Entity>,
    components_index: HashMap<TypeId, usize>,
    components: Vec<Box<dyn AnyColumn>>,
}

impl Archetype {
    pub(crate) fn new(
        signature: &Vec<TypeId>,
        cp_factories: &HashMap<TypeId, fn() -> Box<dyn AnyColumn>>,
    ) -> Self {
        let mut cp_index: HashMap<TypeId, usize> = HashMap::new();
        let mut cp: Vec<Box<dyn AnyColumn>> = Vec::new();
        for (i, type_id) in signature.iter().enumerate() {
            cp_index.insert(*type_id, i);
            cp.push(cp_factories[type_id]());
        }

        Self {
            entities: Vec::new(),
            components_index: cp_index,
            components: cp,
        }
    }

    pub(crate) fn add_entity(&mut self, entity: Entity, components: Vec<(TypeId, Box<dyn Any>)>) {
        for (type_id, boxed_value) in components {
            let comp_indx = self
                .components_index
                .get(&type_id)
                .expect("Bundle contains a component absent from the archetype signature");

            self.components[*comp_indx].push_box(boxed_value)
        }
        self.entities.push(entity);
    }

    pub(crate) fn swap_remove_entity(&mut self, row: usize) -> DeletionResult {
        self.entities.swap_remove(row);
        let mut components: Vec<(TypeId, Box<dyn Any>)> = Vec::with_capacity(self.components.len());
        for elt in self.components.iter_mut() {
            let cp_type = elt.component_type_id();
            components.push((cp_type, elt.swap_remove(row)));
        }

        let swapped_entity = self.entities.get(row).copied().map(|e| (e, row));
        DeletionResult {
            removed_components: components,
            swapped_entity,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.entities.len()
    }

    pub(crate) fn components_index(&self) -> &HashMap<TypeId, usize> {
        &self.components_index
    }

    pub(crate) fn components(&self) -> &Vec<Box<dyn AnyColumn>> {
        &self.components
    }
}

pub(crate) struct DeletionResult {
    pub(crate) removed_components: Vec<(TypeId, Box<dyn Any>)>,
    pub(crate) swapped_entity: Option<(Entity, usize)>,
}

pub struct ArchetypeMatch {
    arch_len: usize,
    columns: Vec<NonNull<()>>,
}

impl ArchetypeMatch {
    pub fn new(arch_len: usize, columns: Vec<NonNull<()>>) -> Self {
        Self { arch_len, columns }
    }

    pub fn arch_len(&self) -> usize {
        self.arch_len
    }
    pub fn columns(&self) -> &Vec<NonNull<()>> {
        &self.columns
    }
}
