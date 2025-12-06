use log::info;
use std::sync::Mutex;

pub static ALL_EVENTS: Mutex<Vec<fn() -> i64>> = Mutex::new(Vec::new());

#[cfg(not(feature = "override"))]
pub mod events {

    use super::*;

    #[cfg(not(feature = "override"))]
    pub fn add(event: fn() -> i64) -> () {
        let mut events = ALL_EVENTS.lock().unwrap();
        events.push(event);
        let name = std::any::type_name_of_val(&event);
        info!("{name} event added");
        ()
    }

    #[cfg(not(feature = "override"))]
    pub fn remove(index: usize) -> () {
        let mut events = ALL_EVENTS.lock().unwrap();
        events.remove(index);
        ()
    }

    #[cfg(not(feature = "override"))]
    pub fn apply_all() -> Vec<i64> {
        let events = ALL_EVENTS.lock().unwrap();
        events.iter().map(|f| f()).collect()
    }
}
