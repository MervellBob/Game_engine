use std::{any::TypeId, collections::HashMap, marker::PhantomData, vec};

use crate::{
    archetypes::archetype::ArchetypeMatch,
    components::bundle::Bundle,
    queries::{
        query_items::{QueryItemInfo, QueryItems},
        query_iter::QueryIter,
    },
    world::world::World,
};


//not specified by rust code because unsafe but; A quuery access data owned infine by world, thus a query must not outlive world.
// This is an invariant undocumented yet.
pub(crate) struct Query<'a, P: QueryItems> {
    descriptor: &'a QueryDescriptor,
    _marker: PhantomData<P>,
}

impl<'a, P: QueryItems> Query<'a, P> {
    pub(crate) fn from_descriptor(descriptor: &'a QueryDescriptor) -> Self {
        Self {
            descriptor,
            _marker: PhantomData,
        }
    }

    pub fn iter(&self) -> QueryIter<'_, P> {
        QueryIter::new(self.descriptor.matches())
    }
}

pub struct QueryFilter {
    include: Vec<TypeId>,
    exclude: Vec<TypeId>,
}

impl QueryFilter {
    pub fn new() -> Self {
        Self {
            include: vec![],
            exclude: vec![],
        }
    }

    pub fn with<B: Bundle>(mut self) -> Self {
        let types = B::get_signature();

        for elt in types {
            if self.exclude.contains(&elt) {
                //TODO log elemnt to include already in exclude -> ignoring
            } else if self.include.contains(&elt) {
                //TODO log elemnt to include already in include -> ignoring
            } else {
                self.include.push(elt);
            }
        }
        self
    }

    pub fn without<B: Bundle>(mut self) -> Self {
        let types = B::get_signature();
        for elt in types {
            if self.exclude.contains(&elt) {
                //TODO log elemnt to include already in exclude -> ignoring
            } else if self.include.contains(&elt) {
                //TODO log elemnt to include already in include -> ignoring
            } else {
                self.exclude.push(elt);
            }
        }
        self
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum BorrowKind {
    Shared,
    Exclusive,
}

pub struct QueryDescriptor {
    components: Vec<TypeId>,
    mutables: Vec<BorrowKind>,
    filter: QueryFilter,
    matches: Vec<ArchetypeMatch>,
}

impl QueryDescriptor {
    pub fn new(param_info: Vec<QueryItemInfo>) -> Self {
        let mut components = vec![];
        let mut mutables = vec![];
        for elt in param_info {
            components.push(elt.type_id());
            mutables.push(elt.mutable());
        }
        Self {
            components,
            mutables,
            filter: QueryFilter::new(),
            matches: vec![],
        }
    }

    pub fn replace_filter(&mut self) -> &mut QueryFilter {
        self.filter = QueryFilter::new();
        &mut self.filter
    }

    pub fn build(&mut self, world: &World) {
        let arch_signatures = world.get_archetypes_signatures();
        let matching_arch = self.get_matching_archetypes(arch_signatures);
        self.matches = world.get_archetype_match(&matching_arch, &self.components);
    }

    fn get_matching_archetypes(&self, arch_signatures: &HashMap<u32, Vec<TypeId>>) -> Vec<u32> {
        let mut matching_arch = vec![];
        for (arch_id, signature) in arch_signatures.iter() {
            let mut push_arch = true;

            for type_id in self.components.iter() {
                if !signature.contains(&type_id) {
                    push_arch = false;
                }
            }

            for type_id in self.filter.include.iter() {
                if !signature.contains(&type_id) {
                    push_arch = false;
                }
            }

            for type_id in self.filter.exclude.iter() {
                if signature.contains(&type_id) {
                    push_arch = false;
                }
            }
            if push_arch {
                matching_arch.push(*arch_id);
            }
        }
        matching_arch
    }

    pub(crate) fn matches(&self) -> &Vec<ArchetypeMatch> {
        &self.matches
    }
}
