/// # App Event Enum
///
/// This enum defines all possible events related to the application itself,
/// such as closing, entering the foreground, or moving to the background.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum AppEvent{
    Quit {
        timestamp: u64,
    },
    AppTerminating {
        timestamp: u64,
    },
    AppLowMemory {
        timestamp: u64,
    },
    AppWillEnterBackground {
        timestamp: u64,
    },
    AppDidEnterBackground {
        timestamp: u64,
    },
    AppWillEnterForeground {
        timestamp: u64,
    },
    AppDidEnterForeground {
        timestamp: u64,
    }
}
