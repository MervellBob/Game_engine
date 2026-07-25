use crate::components::component::Component;
use std::{
    any::{Any, TypeId},
    ptr::NonNull,
};

pub(crate) struct Column<T: Component> {
    pub data: Vec<T>,
}

impl<T: Component> Column<T> {
    pub(crate) fn create_any() -> Box<dyn AnyColumn> {
        Box::new(Self { data: Vec::new() })
    }
}

pub trait AnyColumn {
    fn len(&self) -> usize;
    fn component_type_id(&self) -> TypeId;
    fn swap_remove(&mut self, index: usize) -> Box<dyn Any>;
    fn push_box(&mut self, component: Box<dyn Any>);
    fn data_ptr(&self) -> NonNull<()>;
}

impl<T: Component> AnyColumn for Column<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
    fn component_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn swap_remove(&mut self, index: usize) -> Box<dyn Any> {
        Box::new(self.data.swap_remove(index))
    }

    fn push_box(&mut self, component: Box<dyn Any>) {
        let new_entry = component.downcast::<T>().unwrap();

        self.data.push(*new_entry);
    }

    fn data_ptr(&self) -> NonNull<()> {
        NonNull::new(self.data.as_ptr() as *mut ()).unwrap()
    }
}
