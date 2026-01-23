use std::collections::VecDeque;
use sdl3::event::Event as SdlEvent;
use sdl3::event::WindowEvent as SdlWindowEvent;
use std::time::Duration;

use sdl3::mouse::MouseState;
use sdl3::mouse::MouseButton;
use sdl3::mouse::MouseWheelDirection;
use sdl3::keyboard::Mod;




#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
pub enum InputEvent{
    MouseMotion {
        timestamp: u64,
        window_id: u32,
        which: u32,
        mousestate: MouseState,
        x: f32,
        y: f32,
        xrel: f32,
        yrel: f32,
    },
    MouseButtonDown {
        timestamp: u64,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: f32,
        y: f32,
    },
    MouseButtonUp {
        timestamp: u64,
        window_id: u32,
        which: u32,
        mouse_btn: MouseButton,
        clicks: u8,
        x: f32,
        y: f32,
    },
    MouseWheel {
        timestamp: u64,
        window_id: u32,
        which: u32,
        x: f32,
        y: f32,
        direction: MouseWheelDirection,
        mouse_x: f32,
        mouse_y: f32,
    },
    KeyDown {
        timestamp: u64,
        window_id: u32,
        //keycode: Option<Keycode>, //après interpretation OS (langue & all)
       // scancode: Option<Scancode>,
        //keymod: Mod,
        repeat: bool,
        which: u32, //which periphérique
        raw: u16,
    },
    KeyUp {
        timestamp: u64,
        window_id: u32,
        //keycode: Option<Keycode>,
        //scancode: Option<Scancode>,
        //keymod: Mod,
        repeat: bool,
        which: u32,
        raw: u16,
    }
}

#[derive(Debug)]
pub enum Keycode {
    ScancodeMask = 1_073_741_824,
    Unknown = 0,
    Return = 13,
    Escape = 27,
    Backspace = 8,
    Tab = 9,
    Space = 32,
    Exclaim = 33,
    DblApostrophe = 34,
    Hash = 35,
    Dollar = 36,
    Percent = 37,
    Ampersand = 38,
    Apostrophe = 39,
    LeftParen = 40,
    RightParen = 41,
    Asterisk = 42,
    Plus = 43,
    Comma = 44,
    Minus = 45,
    Period = 46,
    Slash = 47,
    _0 = 48,
    _1 = 49,
    _2 = 50,
    _3 = 51,
    _4 = 52,
    _5 = 53,
    _6 = 54,
    _7 = 55,
    _8 = 56,
    _9 = 57,
    Colon = 58,
    Semicolon = 59,
    Less = 60,
    Equals = 61,
    Greater = 62,
    Question = 63,
    At = 64,
    LeftBracket = 91,
    Backslash = 92,
    RightBracket = 93,
    Caret = 94,
    Underscore = 95,
    Grave = 96,
    A = 97,
    B = 98,
    C = 99,
    D = 100,
    E = 101,
    F = 102,
    G = 103,
    H = 104,
    I = 105,
    J = 106,
    K = 107,
    L = 108,
    M = 109,
    N = 110,
    O = 111,
    P = 112,
    Q = 113,
    R = 114,
    S = 115,
    T = 116,
    U = 117,
    V = 118,
    W = 119,
    X = 120,
    Y = 121,
    Z = 122,
    LeftBrace = 123,
    Pipe = 124,
    RightBrace = 125,
    Tilde = 126,
    Delete = 127,
    PlusMinus = 177,
    CapsLock = 1_073_741_881,
    F1 = 1_073_741_882,
    F2 = 1_073_741_883,
    F3 = 1_073_741_884,
    F4 = 1_073_741_885,
    F5 = 1_073_741_886,
    F6 = 1_073_741_887,
    F7 = 1_073_741_888,
    F8 = 1_073_741_889,
    F9 = 1_073_741_890,
    F10 = 1_073_741_891,
    F11 = 1_073_741_892,
    F12 = 1_073_741_893,
    PrintScreen = 1_073_741_894,
    ScrollLock = 1_073_741_895,
    Pause = 1_073_741_896,
    Insert = 1_073_741_897,
    Home = 1_073_741_898,
    PageUp = 1_073_741_899,
    End = 1_073_741_901,
    PageDown = 1_073_741_902,
    Right = 1_073_741_903,
    Left = 1_073_741_904,
    Down = 1_073_741_905,
    Up = 1_073_741_906,
    NumLockClear = 1_073_741_907,
    KpDivide = 1_073_741_908,
    KpMultiply = 1_073_741_909,
    KpMinus = 1_073_741_910,
    KpPlus = 1_073_741_911,
    KpEnter = 1_073_741_912,
    Kp1 = 1_073_741_913,
    Kp2 = 1_073_741_914,
    Kp3 = 1_073_741_915,
    Kp4 = 1_073_741_916,
    Kp5 = 1_073_741_917,
    Kp6 = 1_073_741_918,
    Kp7 = 1_073_741_919,
    Kp8 = 1_073_741_920,
    Kp9 = 1_073_741_921,
    Kp0 = 1_073_741_922,
    KpPeriod = 1_073_741_923,
    Application = 1_073_741_925,
    Power = 1_073_741_926,
    KpEquals = 1_073_741_927,
    F13 = 1_073_741_928,
    F14 = 1_073_741_929,
    F15 = 1_073_741_930,
    F16 = 1_073_741_931,
    F17 = 1_073_741_932,
    F18 = 1_073_741_933,
    F19 = 1_073_741_934,
    F20 = 1_073_741_935,
    F21 = 1_073_741_936,
    F22 = 1_073_741_937,
    F23 = 1_073_741_938,
    F24 = 1_073_741_939,
    Execute = 1_073_741_940,
    Help = 1_073_741_941,
    Menu = 1_073_741_942,
    Select = 1_073_741_943,
    Stop = 1_073_741_944,
    Again = 1_073_741_945,
    Undo = 1_073_741_946,
    Cut = 1_073_741_947,
    Copy = 1_073_741_948,
    Paste = 1_073_741_949,
    Find = 1_073_741_950,
    Mute = 1_073_741_951,
    VolumeUp = 1_073_741_952,
    VolumeDown = 1_073_741_953,
    KpComma = 1_073_741_957,
    KpEqualsAs400 = 1_073_741_958,
    AltErase = 1_073_741_977,
    SysReq = 1_073_741_978,
    Cancel = 1_073_741_979,
    Clear = 1_073_741_980,
    Prior = 1_073_741_981,
    Return2 = 1_073_741_982,
    Separator = 1_073_741_983,
    Out = 1_073_741_984,
    Oper = 1_073_741_985,
    ClearAgain = 1_073_741_986,
    CrSel = 1_073_741_987,
    ExSel = 1_073_741_988,
    Kp00 = 1_073_742_000,
    Kp000 = 1_073_742_001,
    ThousandsSeparator = 1_073_742_002,
    DecimalSeparator = 1_073_742_003,
    CurrencyUnit = 1_073_742_004,
    CurrencySubunit = 1_073_742_005,
    KpLeftParen = 1_073_742_006,
    KpRightParen = 1_073_742_007,
    KpLeftBrace = 1_073_742_008,
    KpRightBrace = 1_073_742_009,
    KpTab = 1_073_742_010,
    KpBackspace = 1_073_742_011,
    KpA = 1_073_742_012,
    KpB = 1_073_742_013,
    KpC = 1_073_742_014,
    KpD = 1_073_742_015,
    KpE = 1_073_742_016,
    KpF = 1_073_742_017,
    KpXor = 1_073_742_018,
    KpPower = 1_073_742_019,
    KpPercent = 1_073_742_020,
    KpLess = 1_073_742_021,
    KpGreater = 1_073_742_022,
    KpAmpersand = 1_073_742_023,
    KpDblAmpersand = 1_073_742_024,
    KpVerticalBar = 1_073_742_025,
    KpDblVerticalBar = 1_073_742_026,
    KpColon = 1_073_742_027,
    KpHash = 1_073_742_028,
    KpSpace = 1_073_742_029,
    KpAt = 1_073_742_030,
    KpExclam = 1_073_742_031,
    KpMemStore = 1_073_742_032,
    KpMemRecall = 1_073_742_033,
    KpMemClear = 1_073_742_034,
    KpMemAdd = 1_073_742_035,
    KpMemSubtract = 1_073_742_036,
    KpMemMultiply = 1_073_742_037,
    KpMemDivide = 1_073_742_038,
    KpPlusMinus = 1_073_742_039,
    KpClear = 1_073_742_040,
    KpClearEntry = 1_073_742_041,
    KpBinary = 1_073_742_042,
    KpOctal = 1_073_742_043,
    KpDecimal = 1_073_742_044,
    KpHexadecimal = 1_073_742_045,
    LCtrl = 1_073_742_048,
    LShift = 1_073_742_049,
    LAlt = 1_073_742_050,
    LGui = 1_073_742_051,
    RCtrl = 1_073_742_052,
    RShift = 1_073_742_053,
    RAlt = 1_073_742_054,
    RGui = 1_073_742_055,
    Mode = 1_073_742_081,
    Sleep = 1_073_742_082,
    Wake = 1_073_742_083,
    ChannelIncrement = 1_073_742_084,
    ChannelDecrement = 1_073_742_085,
    MediaPlay = 1_073_742_086,
    MediaPause = 1_073_742_087,
    MediaRecord = 1_073_742_088,
    MediaFastForward = 1_073_742_089,
    MediaRewind = 1_073_742_090,
    MediaNextTrack = 1_073_742_091,
    MediaPreviousTrack = 1_073_742_092,
    MediaStop = 1_073_742_093,
    MediaEject = 1_073_742_094,
    MediaPlayPause = 1_073_742_095,
    MediaSelect = 1_073_742_096,
    AcNew = 1_073_742_097,
    AcOpen = 1_073_742_098,
    AcClose = 1_073_742_099,
    AcExit = 1_073_742_100,
    AcSave = 1_073_742_101,
    AcPrint = 1_073_742_102,
    AcProperties = 1_073_742_103,
    AcSearch = 1_073_742_104,
    AcHome = 1_073_742_105,
    AcBack = 1_073_742_106,
    AcForward = 1_073_742_107,
    AcStop = 1_073_742_108,
    AcRefresh = 1_073_742_109,
    AcBookmarks = 1_073_742_110,
    SoftLeft = 1_073_742_111,
    SoftRight = 1_073_742_112,
    Call = 1_073_742_113,
    EndCall = 1_073_742_114,
}

#[derive(Debug)]
pub enum Scancode {
    Unknown = 0,
    A = 4,
    B = 5,
    C = 6,
    D = 7,
    E = 8,
    F = 9,
    G = 10,
    H = 11,
    I = 12,
    J = 13,
    K = 14,
    L = 15,
    M = 16,
    N = 17,
    O = 18,
    P = 19,
    Q = 20,
    R = 21,
    S = 22,
    T = 23,
    U = 24,
    V = 25,
    W = 26,
    X = 27,
    Y = 28,
    Z = 29,
    _1 = 30,
    _2 = 31,
    _3 = 32,
    _4 = 33,
    _5 = 34,
    _6 = 35,
    _7 = 36,
    _8 = 37,
    _9 = 38,
    _0 = 39,
    Return = 40,
    Escape = 41,
    Backspace = 42,
    Tab = 43,
    Space = 44,
    Minus = 45,
    Equals = 46,
    LeftBracket = 47,
    RightBracket = 48,
    Backslash = 49,
    NonUsHash = 50,
    Semicolon = 51,
    Apostrophe = 52,
    Grave = 53,
    Comma = 54,
    Period = 55,
    Slash = 56,
    CapsLock = 57,
    F1 = 58,
    F2 = 59,
    F3 = 60,
    F4 = 61,
    F5 = 62,
    F6 = 63,
    F7 = 64,
    F8 = 65,
    F9 = 66,
    F10 = 67,
    F11 = 68,
    F12 = 69,
    PrintScreen = 70,
    ScrollLock = 71,
    Pause = 72,
    Insert = 73,
    Home = 74,
    PageUp = 75,
    Delete = 76,
    End = 77,
    PageDown = 78,
    Right = 79,
    Left = 80,
    Down = 81,
    Up = 82,
    NumLockClear = 83,
    KpDivide = 84,
    KpMultiply = 85,
    KpMinus = 86,
    KpPlus = 87,
    KpEnter = 88,
    Kp1 = 89,
    Kp2 = 90,
    Kp3 = 91,
    Kp4 = 92,
    Kp5 = 93,
    Kp6 = 94,
    Kp7 = 95,
    Kp8 = 96,
    Kp9 = 97,
    Kp0 = 98,
    KpPeriod = 99,
    NonUsBackslash = 100,
    Application = 101,
    Power = 102,
    KpEquals = 103,
    F13 = 104,
    F14 = 105,
    F15 = 106,
    F16 = 107,
    F17 = 108,
    F18 = 109,
    F19 = 110,
    F20 = 111,
    F21 = 112,
    F22 = 113,
    F23 = 114,
    F24 = 115,
    Execute = 116,
    Help = 117,
    Menu = 118,
    Select = 119,
    Stop = 120,
    Again = 121,
    Undo = 122,
    Cut = 123,
    Copy = 124,
    Paste = 125,
    Find = 126,
    Mute = 127,
    VolumeUp = 128,
    VolumeDown = 129,
    KpComma = 133,
    KpEqualsAs400 = 134,
    International1 = 135,
    International2 = 136,
    International3 = 137,
    International4 = 138,
    International5 = 139,
    International6 = 140,
    International7 = 141,
    International8 = 142,
    International9 = 143,
    Lang1 = 144,
    Lang2 = 145,
    Lang3 = 146,
    Lang4 = 147,
    Lang5 = 148,
    Lang6 = 149,
    Lang7 = 150,
    Lang8 = 151,
    Lang9 = 152,
    AltErase = 153,
    SysReq = 154,
    Cancel = 155,
    Clear = 156,
    Prior = 157,
    Return2 = 158,
    Separator = 159,
    Out = 160,
    Oper = 161,
    ClearAgain = 162,
    CrSel = 163,
    ExSel = 164,
    Kp00 = 176,
    Kp000 = 177,
    ThousandsSeparator = 178,
    DecimalSeparator = 179,
    CurrencyUnit = 180,
    CurrencySubunit = 181,
    KpLeftParen = 182,
    KpRightParen = 183,
    KpLeftBrace = 184,
    KpRightBrace = 185,
    KpTab = 186,
    KpBackspace = 187,
    KpA = 188,
    KpB = 189,
    KpC = 190,
    KpD = 191,
    KpE = 192,
    KpF = 193,
    KpXor = 194,
    KpPower = 195,
    KpPercent = 196,
    KpLess = 197,
    KpGreater = 198,
    KpAmpersand = 199,
    KpDblAmpersand = 200,
    KpVerticalBar = 201,
    KpDblVerticalBar = 202,
    KpColon = 203,
    KpHash = 204,
    KpSpace = 205,
    KpAt = 206,
    KpExclam = 207,
    KpMemStore = 208,
    KpMemRecall = 209,
    KpMemClear = 210,
    KpMemAdd = 211,
    KpMemSubtract = 212,
    KpMemMultiply = 213,
    KpMemDivide = 214,
    KpPlusMinus = 215,
    KpClear = 216,
    KpClearEntry = 217,
    KpBinary = 218,
    KpOctal = 219,
    KpDecimal = 220,
    KpHexadecimal = 221,
    LCtrl = 224,
    LShift = 225,
    LAlt = 226,
    LGui = 227,
    RCtrl = 228,
    RShift = 229,
    RAlt = 230,
    RGui = 231,
    Mode = 257,
    Sleep = 258,
    Wake = 259,
    ChannelIncrement = 260,
    ChannelDecrement = 261,
    MediaPlay = 262,
    MediaPause = 263,
    MediaRecord = 264,
    MediaFastForward = 265,
    MediaRewind = 266,
    MediaNextTrack = 267,
    MediaPreviousTrack = 268,
    MediaStop = 269,
    MediaEject = 270,
    MediaPlayPause = 271,
    MediaSelect = 272,
    AcNew = 273,
    AcOpen = 274,
    AcClose = 275,
    AcExit = 276,
    AcSave = 277,
    AcPrint = 278,
    AcProperties = 279,
    AcSearch = 280,
    AcHome = 281,
    AcBack = 282,
    AcForward = 283,
    AcStop = 284,
    AcRefresh = 285,
    AcBookmarks = 286,
    SoftLeft = 287,
    SoftRight = 288,
    Call = 289,
    EndCall = 290,
    Reserved = 400,
    Count = 512,
}

pub struct EventQueue<T> {
    queue: VecDeque<T>,
}

impl<T> EventQueue<T> {
    pub fn new() -> Self { Self { queue: VecDeque::new() } }

    pub fn push(&mut self, event:T) {self.queue.push_back(event)}

    pub fn pop(&mut self) -> Option<T> {self.queue.pop_front()}

    pub fn is_empty(&mut self) -> bool {self.queue.is_empty()}

    pub fn get(&self,index:usize) -> Option<&T> {self.queue.get(index)}

    pub fn get_mut(&mut self,index:usize) -> Option<&mut T> {self.queue.get_mut(index)}

}


pub struct EventManager {
    pub app_events_queue:EventQueue<AppEvent>,
    pub window_events_queue:EventQueue<WindowEvent>,
    pub input_events_queue:EventQueue<InputEvent>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            app_events_queue:EventQueue::new(),
            window_events_queue:EventQueue::new(),
            input_events_queue:EventQueue::new()
        }
    }

    pub fn start_game_loop(&mut self, sdl_context:sdl3::Sdl, mut canvas:sdl3::render::WindowCanvas) {
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut game_running = true;
        while game_running {

            for event in event_pump.poll_iter() {
                game_running = game_running && self.match_event_types(event);
            }

            let event = self.window_events_queue.pop();
            match event {
                None => {}
                _ => { println!(" I am cathcing this event muy boy{:#?}",event)}
            }

            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
            // Sleep is not accurate in timing it has a 1ms to 15 ms overshoot apparently, and 1/20th of a sec is ~50 ms 
        }
    }

    fn match_event_types(&mut self, event:sdl3::event::Event) -> bool {
        match event {
            SdlEvent::Window {timestamp,window_id,win_event} => {
                return self.match_window_events(timestamp,window_id,win_event);
            }
            
            SdlEvent::Quit { timestamp } => {
                let app_event:AppEvent = AppEvent::Quit{
                    timestamp,
                };
                self.app_events_queue.push(app_event)
            }
            SdlEvent::AppTerminating { timestamp } => {
                let app_event:AppEvent = AppEvent::AppTerminating{
                    timestamp,
                };
                self.app_events_queue.push(app_event)

            }
            SdlEvent::AppLowMemory { timestamp } => {
                let app_event:AppEvent = AppEvent::AppLowMemory{
                    timestamp,
                };
                self.app_events_queue.push(app_event)
            }
            SdlEvent::AppWillEnterBackground { timestamp } => {
                let app_event:AppEvent = AppEvent::AppWillEnterBackground{
                    timestamp,
                };
                self.app_events_queue.push(app_event)
            }
            SdlEvent::AppDidEnterBackground { timestamp } => {
                let app_event:AppEvent = AppEvent::AppDidEnterBackground{
                    timestamp,
                };
                self.app_events_queue.push(app_event)
            }
            SdlEvent::AppWillEnterForeground { timestamp } => {
                let app_event:AppEvent = AppEvent::AppWillEnterForeground{
                    timestamp,
                };
                self.app_events_queue.push(app_event)
            }
            SdlEvent::AppDidEnterForeground { timestamp } => {
                let app_event:AppEvent = AppEvent::AppDidEnterForeground{
                    timestamp,
                };
                self.app_events_queue.push(app_event)
            }

            SdlEvent::MouseMotion {timestamp,window_id,which,mousestate,x,y,xrel,yrel} => {
                let input_event:InputEvent = InputEvent::MouseMotion{
                    timestamp,
                    window_id,
                    which,
                    mousestate,
                    x,
                    y,
                    xrel,
                    yrel,
                };
                self.input_events_queue.push(input_event)
            }
            SdlEvent::MouseButtonDown {timestamp,window_id,which,mouse_btn,clicks,x,y} => {
                let input_event:InputEvent = InputEvent::MouseButtonDown{
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                };
                self.input_events_queue.push(input_event)
            }
            SdlEvent::MouseButtonUp {timestamp,window_id,which,mouse_btn,clicks,x,y} => {
                let input_event:InputEvent = InputEvent::MouseButtonUp{
                    timestamp,
                    window_id,
                    which,
                    mouse_btn,
                    clicks,
                    x,
                    y,
                };
                self.input_events_queue.push(input_event)
            }
            SdlEvent::MouseWheel {timestamp,window_id,which,x,y,direction,mouse_x,mouse_y} => {
                let input_event:InputEvent = InputEvent::MouseWheel{
                    timestamp,
                    window_id,
                    which,
                    x,
                    y,
                    direction,
                    mouse_x,
                    mouse_y,
                };
                self.input_events_queue.push(input_event)
            }
            SdlEvent::KeyDown {timestamp,window_id,keycode,scancode,keymod,repeat,which,raw} => {
                let input_event:InputEvent = InputEvent::KeyDown{
                    timestamp,
                    window_id,
                    //keycode, //après interpretation OS (langue & all)
                    //scancode,
                    //keymod,
                    repeat,
                    which, //which periphérique
                    raw,
                };
                self.input_events_queue.push(input_event)
            }
            SdlEvent::KeyUp {timestamp,window_id,keycode,scancode,keymod,repeat,which,raw} => {
                let input_event:InputEvent = InputEvent::KeyUp{
                    timestamp,
                    window_id,
                    //keycode, //après interpretation OS (langue & all)
                    //scancode,
                    //keymod,
                    repeat,
                    which, //which periphérique
                    raw,
                };
                self.input_events_queue.push(input_event)
            }
            _ => {}
        }
        return true
    }

    fn match_window_events(&mut self, timestamp:u64, window_id:u32, window_event:SdlWindowEvent) -> bool {
        match window_event {
            SdlWindowEvent::None => { 
                let window_event:WindowEvent = WindowEvent::None { 
                    timestamp,
                    window_id,
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::Shown => { 
                let window_event:WindowEvent = WindowEvent::Shown { 
                    timestamp,
                    window_id,
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::Hidden => { 
                let window_event:WindowEvent = WindowEvent::Hidden { 
                    timestamp,
                    window_id,
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::Exposed => { 
                let window_event:WindowEvent = WindowEvent::Exposed { 
                    timestamp,
                    window_id,
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::Moved (x,y) => { 
                let window_event:WindowEvent = WindowEvent::Moved { 
                    timestamp,
                    window_id,
                    x,
                    y,
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::Resized (width,height) => { 
                let window_event:WindowEvent = WindowEvent::Resized { 
                    timestamp,
                    window_id,
                    width,
                    height,
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::PixelSizeChanged (width,height) => { 
                let window_event:WindowEvent = WindowEvent::PixelSizeChanged { 
                    timestamp,
                    window_id,
                    width,
                    height,
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::Minimized => { 
                let window_event:WindowEvent = WindowEvent::Minimized { 
                    timestamp,
                    window_id,
 
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::Maximized => { 
                let window_event:WindowEvent = WindowEvent::Maximized { 
                    timestamp,
                    window_id,
 
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::Restored => { 
                let window_event:WindowEvent = WindowEvent::Restored { 
                    timestamp,
                    window_id,
 
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::MouseEnter => { 
                let window_event:WindowEvent = WindowEvent::MouseEnter { 
                    timestamp,
                    window_id,
 
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::MouseLeave => { 
                let window_event:WindowEvent = WindowEvent::MouseLeave { 
                    timestamp,
                    window_id,
 
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::FocusGained => { 
                let window_event:WindowEvent = WindowEvent::FocusGained { 
                    timestamp,
                    window_id,
 
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::FocusLost => { 
                let window_event:WindowEvent = WindowEvent::FocusLost { 
                    timestamp,
                    window_id,
 
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::CloseRequested => { 
                let window_event:WindowEvent = WindowEvent::CloseRequested { 
                    timestamp,
                    window_id,
 
                };
            self.window_events_queue.push(window_event);
            return false //TEMPORARY
            }
            SdlWindowEvent::HitTest (x,y) => { 
                let window_event:WindowEvent = WindowEvent::HitTest { 
                    timestamp,
                    window_id,
                    x,
                    y, 
                };
            self.window_events_queue.push(window_event)
            }
            SdlWindowEvent::DisplayChanged (display_index) => { 
                let window_event:WindowEvent = WindowEvent::DisplayChanged { 
                    timestamp,
                    window_id,
                    display_index, 
                };
            self.window_events_queue.push(window_event)
            }
            _ => {}
        }
        return true
    }
}
