use std::marker::PhantomData;

use crate::{archetypes::archetype::ArchetypeMatch, queries::query_items::QueryItems};

pub struct QueryIter<'q, P: QueryItems> {
    matches: &'q [ArchetypeMatch],
    current_archetype: usize,
    current_row: usize,
    _marker: PhantomData<P>,
}

impl<'q, P: QueryItems> QueryIter<'q, P> {
    pub fn new(matches: &'q [ArchetypeMatch]) -> Self {
        Self {
            matches,
            current_archetype: 0,
            current_row: 0,
            _marker: PhantomData,
        }
    }
}

impl<'q, P: QueryItems> Iterator for QueryIter<'q, P> {
    type Item = P::Item<'q>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_archetype >= self.matches.len() {
            return None;
        }

        let arch = &self.matches[self.current_archetype];

        if self.current_row >= arch.arch_len() {
            self.current_row = 0;
            self.current_archetype += 1;
            return self.next();
        }
        let item = unsafe { <P as QueryItems>::fetch(arch.columns(), self.current_row) };
        self.current_row += 1;

        Some(item)
    }
}
