
/// # Window Event Enum
///
/// This enum represents all possible types of events related to the application window.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WindowEvent {
    None {
        timestamp: u64,
        window_id: u32,
    },
    Shown {
        timestamp: u64,
        window_id: u32,
    },
    Hidden {
        timestamp: u64,
        window_id: u32,
    },
    Exposed {
        timestamp: u64,
        window_id: u32,
    },
    Moved {
        timestamp: u64,
        window_id: u32,
        x: i32,
        y: i32,
    },
    Resized {
        timestamp: u64,
        window_id: u32,
        width: i32,
        height: i32,
    },
    PixelSizeChanged {
        timestamp: u64,
        window_id: u32,
        width: i32,
        height: i32,
    },
    Minimized {
        timestamp: u64,
        window_id: u32,
    },
    Maximized {
        timestamp: u64,
        window_id: u32,
    },
    Restored {
        timestamp: u64,
        window_id: u32,
    },
    MouseEnter {
        timestamp: u64,
        window_id: u32,
    },
    MouseLeave {
        timestamp: u64,
        window_id: u32,
    },
    FocusGained {
        timestamp: u64,
        window_id: u32,
    },
    FocusLost {
        timestamp: u64,
        window_id: u32,
    },
    CloseRequested {
        timestamp: u64,
        window_id: u32,
    },
    HitTest {
        timestamp: u64,
        window_id: u32,
        x: i32,
        y: i32,
    },
    DisplayChanged {
        timestamp: u64,
        window_id: u32,
        display_index: i32,
    },
}
