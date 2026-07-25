use std::{
    any::{TypeId, type_name},
    collections::HashMap,
};

use crate::{
    queries::query_items::QueryItems,
    systems::{system::System, system_wrapper::SystemWrapper},
};

pub struct Plugin {
    registered_systems: HashMap<TypeId, SystemWrapper>,
}

impl Plugin {
    pub fn new() -> Self {
        Self {
            registered_systems: HashMap::new(),
        }
    }

    pub fn register<P, S>(&mut self) -> &mut SystemWrapper
    where
        P: QueryItems,
        S: System<P> + 'static,
    {
        let sys_type_id = TypeId::of::<S>();
        self.registered_systems
            .insert(TypeId::of::<S>(), SystemWrapper::new::<P, S>());
        self.registered_systems.get_mut(&sys_type_id).unwrap()
    }

    pub fn get_system<S: 'static>(&mut self) -> &mut SystemWrapper {
        self.registered_systems
            .get_mut(&TypeId::of::<S>())
            .unwrap_or_else(|| panic!("System '{}' is not registered.", type_name::<S>()))
    }
}