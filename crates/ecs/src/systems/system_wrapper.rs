use crate::{
    queries::{
        query::{Query, QueryDescriptor, QueryFilter},
        query_items::QueryItems,
    },
    systems::system::System,
};

pub enum ScheduleType {
    Disabled,
    StartUp,
    Shutdown,
    Frame,
    Physics,
}

pub(crate) fn system_runner<P: QueryItems, S: System<P>>(query_descriptor: &QueryDescriptor) {
    let query = Query::<P>::from_descriptor(&query_descriptor);
    S::run(&query);
}

pub struct SystemWrapper {
    run: fn(&QueryDescriptor),
    query_descriptor: QueryDescriptor,
    schedule_type: ScheduleType,
}

impl SystemWrapper {
    pub(crate) fn new<P, S>() -> Self
    where
        P: QueryItems,
        S: System<P> + 'static,
    {
        Self {
            run: system_runner::<P, S>,
            query_descriptor: QueryDescriptor::new(P::checked_params()),
            schedule_type: ScheduleType::Disabled,
        }
    }

    pub fn schedule_type(&self) -> &ScheduleType {
        &self.schedule_type
    }

    pub fn set_filter(&mut self) -> &mut QueryFilter {
        self.query_descriptor.replace_filter()
    }

    pub fn set_schedule_type(mut self, new_schedule: ScheduleType) -> SystemWrapper {
        self.schedule_type = new_schedule;
        self
    }
}
