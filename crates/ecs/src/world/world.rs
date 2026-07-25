use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::{
    archetypes::{archetype::ArchetypeMatch, archetype_manager::ArchetypeManager},
    components::{component::ComponentRegistry, component_column::AnyColumn},
    entities::{entity::Entity, entity_editor::EntityEditor, entity_manager::EntityManager},
};

pub struct World {
    arch_manager: ArchetypeManager,
    ent_manager: EntityManager,
    cp_registry: ComponentRegistry,
}
impl World {
    pub fn new() -> Self {
        Self {
            arch_manager: ArchetypeManager::new(),
            ent_manager: EntityManager::new(),
            cp_registry: ComponentRegistry::new(),
        }
    }

    // region: --- Entity access ---
    pub fn new_entity(&mut self) -> EntityEditor<'_> {
        let new_entity = self.ent_manager.create_entity();
        self.arch_manager
            .add_entity_to_archetype(new_entity, 0, vec![]);
        EntityEditor::new(new_entity, self)
    }

    pub fn entity(&mut self, entity: Entity) -> EntityEditor<'_> {
        assert!(self.is_valid(entity), "invalid entity");
        EntityEditor::new(entity, self)
    }

    // endregion

    // region: --- Entity modification ---

    pub(crate) fn modify_entity(
        &mut self,
        entity: Entity,
        add_cp: Vec<(TypeId, Box<dyn Any>)>,
        remove_cp: Vec<TypeId>,
    ) {
        let source_cp = self.extract_entity(entity);
        let dest_cp = self.get_destination_components(source_cp, add_cp, remove_cp);
        let dest_sign = self.get_destination_signature(&dest_cp);

        let dest_arch_id = self
            .arch_manager
            .get_archetype_from_signature(&dest_sign)
            .unwrap_or_else(|| {
                self.arch_manager
                    .create_archetype(dest_sign, self.cp_registry.cp_factories())
            });

        self.arch_manager
            .add_entity_to_archetype(entity, dest_arch_id, dest_cp);
    }

    pub(crate) fn delete_entity(
        &mut self,
        entity: Entity,
    ) -> Vec<(TypeId, Box<dyn Any + 'static>)> {
        assert!(self.is_valid(entity), "invalid entity");

        let removed_components = self.extract_entity(entity);
        self.ent_manager.recycle_entity(entity);
        removed_components
    }

    fn extract_entity(&mut self, entity: Entity) -> Vec<(TypeId, Box<dyn Any + 'static>)> {
        let deletion_result = self.arch_manager.delete_entity(entity);
        if let Some((swapped, new_row)) = deletion_result.swapped_entity {
            self.arch_manager.update_entity_row(swapped, new_row);
        }
        deletion_result.removed_components
    }

    fn get_destination_components(
        &self,
        source_cp: Vec<(TypeId, Box<dyn Any>)>,
        add_cp: Vec<(TypeId, Box<dyn Any>)>,
        remove_cp: Vec<TypeId>,
    ) -> Vec<(TypeId, Box<dyn Any>)> {
        let mut dest_cp = source_cp;

        for type_id in remove_cp {
            if let Some(i) = dest_cp.iter().position(|x| x.0 == type_id) {
                dest_cp.swap_remove(i);
            } else {
                //TODO log and good message instead of assert "remove_item not present"
            }
        }
        for (type_id, cp) in add_cp {
            if !dest_cp.iter().any(|(id, _)| *id == type_id) {
                dest_cp.push((type_id, cp));
            } else {
                //TODO log and good message instead of assert "add item already present"
            }
        }
        dest_cp
    }

    fn get_destination_signature(&self, dest_cp: &Vec<(TypeId, Box<dyn Any>)>) -> Vec<TypeId> {
        let mut signature: Vec<TypeId> = dest_cp.iter().map(|(type_id, _)| *type_id).collect();

        signature.sort();

        signature
    }

    // endregion

    // region: --- Component registry ---
    pub(crate) fn register_component(
        &mut self,
        type_id: TypeId,
        factory: fn() -> Box<dyn AnyColumn>,
    ) {
        self.cp_registry.register(type_id, factory);
    }
    // endregion

    // region: --- Validation ---
    fn is_valid(&self, entity: Entity) -> bool {
        self.ent_manager.has_correct_gen(entity)
            && self.arch_manager.entities_locations().contains_key(&entity)
    }
    // endregion

    pub(crate) fn get_archetypes_signatures(&self) -> &HashMap<u32, Vec<TypeId>> {
        self.arch_manager.archetypes_signatures()
    }

    pub(crate) fn get_archetype_match(
        &self,
        archetypes: &Vec<u32>,
        components: &Vec<TypeId>,
    ) -> Vec<ArchetypeMatch> {
        self.arch_manager
            .get_archetype_match(archetypes, components)
    }
}
