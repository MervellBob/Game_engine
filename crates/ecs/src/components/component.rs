use std::{any::TypeId, collections::HashMap};

use crate::components::component_column::AnyColumn;

pub trait Component: 'static {}

pub(crate) struct ComponentRegistry {
    cp_factories: HashMap<TypeId, fn() -> Box<dyn AnyColumn>>,
}

impl ComponentRegistry {
    pub(crate) fn new() -> Self {
        Self {
            cp_factories: HashMap::new(),
        }
    }

    pub(crate) fn register(&mut self, type_id: TypeId, factory: fn() -> Box<dyn AnyColumn>) {
        if self.cp_factories.contains_key(&type_id) {
            return;
        }
        self.cp_factories.insert(type_id, factory);
    }

    pub(crate) fn cp_factories(&self) -> &HashMap<TypeId, fn() -> Box<dyn AnyColumn>> {
        &self.cp_factories
    }
}
