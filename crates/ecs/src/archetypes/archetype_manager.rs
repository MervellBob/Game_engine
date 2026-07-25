use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    archetypes::archetype::{Archetype, ArchetypeMatch, DeletionResult},
    components::component_column::AnyColumn,
    entities::entity::{Entity, EntityLocation},
};

pub(crate) struct ArchetypeManager {
    last_id: u32,
    entities_locations: HashMap<Entity, EntityLocation>, //maps each entity to its archetype id.
    archetypes_signatures: HashMap<u32, Vec<TypeId>>,
    archetypes: HashMap<u32, Archetype>,
}

impl ArchetypeManager {
    pub(crate) fn new() -> Self {
        let empty_archetype = Archetype::new(&vec![], &HashMap::new());
        Self {
            last_id: 0,
            entities_locations: HashMap::new(),
            archetypes_signatures: HashMap::from([(0, vec![])]), //initialize archetype for empty entities
            archetypes: HashMap::from([(0, empty_archetype)]),
        }
    }

    pub(crate) fn create_archetype(
        &mut self,
        arch_signature: Vec<TypeId>,
        cp_factories: &HashMap<TypeId, fn() -> Box<dyn AnyColumn>>,
    ) -> u32 {
        self.last_id += 1;
        let new_arch = Archetype::new(&arch_signature, cp_factories);
        self.archetypes.insert(self.last_id, new_arch);
        self.archetypes_signatures
            .insert(self.last_id, arch_signature);
        self.last_id
    }

    pub(crate) fn add_entity_to_archetype(
        &mut self,
        entity: Entity,
        archetype_id: u32,
        components: Vec<(TypeId, Box<dyn Any>)>,
    ) {
        let archetype = self
            .archetypes
            .get_mut(&archetype_id)
            .expect("archetype not found for entity adding");
        archetype.add_entity(entity, components);

        let ent_location = EntityLocation::new(archetype_id, archetype.len() - 1);
        self.entities_locations.insert(entity, ent_location);
    }

    pub(crate) fn delete_entity(&mut self, entity: Entity) -> DeletionResult {
        let entity_location = self.entities_locations()[&entity];
        let archetype = self
            .archetypes
            .get_mut(&entity_location.arch_id())
            .expect("archetype not found for entity removal");
        self.entities_locations.remove(&entity);

        archetype.swap_remove_entity(entity_location.row())
    }

    pub(crate) fn update_entity_row(&mut self, entity: Entity, new_row: usize) {
        let entity_location = self
            .entities_locations
            .get_mut(&entity)
            .expect("Entity not found in entities_locations");

        entity_location.set_row(new_row);
    }

    // region: --- Getters

    pub(crate) fn entities_locations(&self) -> &HashMap<Entity, EntityLocation> {
        &self.entities_locations
    }

    pub(crate) fn get_archetype_from_signature(&self, signature: &Vec<TypeId>) -> Option<u32> {
        if let Some((key, _)) = self
            .archetypes_signatures
            .iter()
            .find(|(_, v)| *v == signature)
        {
            return Some(*key);
        } else {
            return None;
        }
    }

    pub(crate) fn archetypes_signatures(&self) -> &HashMap<u32, Vec<TypeId>> {
        &self.archetypes_signatures
    }

    pub(crate) fn get_archetype_match(
        &self,
        archetypes: &Vec<u32>,
        components: &Vec<TypeId>,
    ) -> Vec<ArchetypeMatch> {
        let mut matching_archetypes = vec![];
        for arch_id in archetypes {
            let arch = self
                .archetypes
                .get(arch_id)
                .expect("Incorrect archetype id while retrieving components pointers");

            let mut pointers = vec![];
            for cp_id in components.iter() {
                let cp_index = arch
                    .components_index()
                    .get(&cp_id)
                    .expect("Incorrect component id while retrieving components pointers");

                let pointer = arch.components()[*cp_index].data_ptr();
                pointers.push(pointer);
            }
            let arch_match = ArchetypeMatch::new(arch.len(), pointers);
            matching_archetypes.push(arch_match);
        }
        matching_archetypes
    }
    // endregion
}
