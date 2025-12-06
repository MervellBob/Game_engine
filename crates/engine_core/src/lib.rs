pub use events::events::*;
use log::info;

#[cfg(not(feature = "override"))]
pub fn entry() -> () {
    info!("Loop");
    events::events::apply_all();
    ()
}
