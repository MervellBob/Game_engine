use crate::queries::{query::Query, query_items::QueryItems};

pub trait System<P: QueryItems> {
    fn run(query: &Query<P>);
}
