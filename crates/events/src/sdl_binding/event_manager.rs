use std::collections::VecDeque;
use sdl3::event::Event as SdlEvent;
use sdl3::event::WindowEvent as SdlWindowEvent;

use super::app_events::AppEvent; 
use super::window_events::WindowEvent;
use super::input_events::InputEvent; 
use super::input_events::Keycode;
use super::input_events::Keymod;
use super::input_events::Scancode; 
use super::input_events::MouseWheelDirection; 
use super::input_events::MouseButtonState; 
use super::input_events::MouseButton; 


/// # Temporary Event Storage
///
/// This struct is used for temporarily storing events in a `VecDeque`.
#[derive(Debug)]
pub struct EventQueue<T> {
    queue: VecDeque<T>,
}

impl<T> EventQueue<T> {
    pub fn new() -> Self { Self { queue: VecDeque::new() } }

    pub fn push(&mut self, event:T) {self.queue.push_back(event)}

    pub fn pop(&mut self) -> Option<T> {self.queue.pop_front()}

    pub fn is_empty(&mut self) -> bool {self.queue.is_empty()}

    // pub fn get(&self,index:usize) -> Option<&T> {self.queue.get(index)}

    // pub fn get_mut(&mut self,index:usize) -> Option<&mut T> {self.queue.get_mut(index)}
}

/// # Event Manager
///
/// This struct manages events for SQL bindings by mapping `sql3` events
/// to the game engine's internal event system and storing them in a dedicated event queue.
pub struct EventManager {
    app_events_queue:EventQueue<AppEvent>,
    window_events_queue:EventQueue<WindowEvent>,
    input_events_queue:EventQueue<InputEvent>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            app_events_queue:EventQueue::new(),
            window_events_queue:EventQueue::new(),
            input_events_queue:EventQueue::new()
        }
    }

    // ----- GETTERS -----
    pub fn app_events_mut(&mut self) -> &mut EventQueue<AppEvent> {
        &mut self.app_events_queue
    }
    pub fn window_events_mut(&mut self) -> &mut EventQueue<WindowEvent> {
        &mut self.window_events_queue
    }
    pub fn input_events_mut(&mut self) -> &mut EventQueue<InputEvent> {
        &mut self.input_events_queue
    }

    /// # Event Matching Function
    ///
    /// This method matches events from SQL to the game engine's internal event system.
    pub fn match_event_types(&mut self, event_pump:&mut sdl3::EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                SdlEvent::Window {timestamp,window_id,win_event} => {
                    match win_event {
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
                        mouse_buttons: self.map_sdl_mousestate(&mousestate),
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
                        mouse_button: MouseButton::try_from(mouse_btn as u8).unwrap_or(MouseButton::Unknown),
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
                        mouse_button: MouseButton::try_from(mouse_btn as u8).unwrap_or(MouseButton::Unknown),
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
                        direction: self.map_sdl_mousewheel_direction(direction),
                        mouse_x,
                        mouse_y,
                    };
                    self.input_events_queue.push(input_event)
                }
                SdlEvent::KeyDown {timestamp,window_id,keycode,scancode,keymod,repeat,which,raw} => {
                    let input_event:InputEvent = InputEvent::KeyDown{
                        timestamp,
                        window_id,
                        keycode : self.map_sdl_keycode(keycode, timestamp), //après interpretation OS (langue & all)
                        scancode : self.map_sdl_scancode(scancode, timestamp),
                        keymod: Keymod::from_bits_retain(keymod.bits()),
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
                        keycode : self.map_sdl_keycode(keycode, timestamp), //après interpretation OS (langue & all)
                        scancode : self.map_sdl_scancode(scancode, timestamp),
                        keymod: Keymod::from_bits_retain(keymod.bits()),
                        repeat,
                        which, //which periphérique
                        raw,
                    };
                    self.input_events_queue.push(input_event)
                }
                _ => {}
            }
        }
    }


    /// # Mouse State Mapping Method
    ///
    /// This method maps SDL mouse states to the game engine's internal mouse button states.
    fn map_sdl_mousestate(& self, ms: &sdl3::mouse::MouseState) -> MouseButtonState {
        MouseButtonState {
            left: ms.left(),
            right: ms.right(),
            middle: ms.middle(),
            x1: ms.x1(),
            x2: ms.x2(),
        }
    }

    /// # Mouse Wheel Direction Mapping Method
    ///
    /// This method maps SDL wheel directions to the game engine's internal wheel direction.
    fn map_sdl_mousewheel_direction(& self ,direction: sdl3::mouse::MouseWheelDirection) -> MouseWheelDirection {
        match direction {
            sdl3::mouse::MouseWheelDirection::Normal => MouseWheelDirection::Normal,
            sdl3::mouse::MouseWheelDirection::Flipped => MouseWheelDirection::Flipped,
            _ => MouseWheelDirection::Unknown,
        }
    }

 
    /// # Keycode Mapping Method
    ///
    /// This method maps SDL keycodes to the game engine's internal keycodes.
    /// 
    /// **Current paradigm:** If an SDL keycode does not have a corresponding internal value,
    /// it is mapped to an `Unknown` keycode.
    fn map_sdl_keycode(& self, sdl_keycode: Option<sdl3::keyboard::Keycode> , timestamp:u64) -> Keycode {
        match sdl_keycode {
            Some(k) => Keycode::try_from(k as u32).unwrap_or(Keycode::Unknown),
            None => {
                println!("key pressed without keycode, timestamp = {:?}", timestamp);
                Keycode::Unknown
            }
        }
    }

    /// # Scancode Mapping Method
    ///
    /// This method maps SDL scancodes to the game engine's internal scancodes.
    ///
    /// **Current paradigm:** If an SDL scancode does not have a corresponding internal value,
    /// it is mapped to `Unknown`.
    fn map_sdl_scancode(& self, sdl_scancode: Option<sdl3::keyboard::Scancode>, timestamp:u64) -> Scancode {
        match sdl_scancode {
            Some(k) => Scancode::try_from(k as u32).unwrap_or(Scancode::Unknown),
            None => {
                println!("key pressed without scancode, timestamp = {:?}", timestamp);
                Scancode::Unknown
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use sdl3::keyboard::Scancode as SdlScancode;
    use sdl3::keyboard::Keycode as SdlKeycode;
    use sdl3::mouse::MouseWheelDirection as SdlMouseDirection;

    #[test]
    fn map_sdl_scancode_valid_code() {
        let manager = EventManager::new();
        assert_eq!(manager.map_sdl_scancode(Some(SdlScancode::A),123), Scancode::A,"Mapping of scancode A failed");
        assert_eq!(manager.map_sdl_scancode(Some(SdlScancode::CapsLock),456), Scancode::CapsLock,"Mapping of scancode CapsLock failed");
    }
    #[test]
    fn map_sdl_scancode_none() {
        let manager = EventManager::new();
        assert_eq!(manager.map_sdl_scancode(None,111), Scancode::Unknown,"Mapping of scancode 'None' failed");
    }



    #[test]
    fn map_sdl_keycode_valid_code() {
        let manager = EventManager::new();
        assert_eq!(manager.map_sdl_keycode(Some(SdlKeycode::A),123), Keycode::A,"Mapping of Keycode A failed");
        assert_eq!(manager.map_sdl_keycode(Some(SdlKeycode::CapsLock),456), Keycode::CapsLock,"Mapping of Keycode CapsLock failed");
    }
    #[test]
    fn map_sdl_keycode_none() {
        let manager = EventManager::new();
        assert_eq!(manager.map_sdl_keycode(None,111), Keycode::Unknown,"Mapping of Keycode 'None' failed");
    }



    #[test]
    fn map_sdl_mousewheel_direction_normal() {
        let manager = EventManager::new();
        assert_eq!(manager.map_sdl_mousewheel_direction(SdlMouseDirection::Normal), MouseWheelDirection::Normal,"Mapping of mousewheel direction normal failed");
    }
    #[test]
    fn map_sdl_mousewheel_direction_flipped() {
        let manager = EventManager::new();
        assert_eq!(manager.map_sdl_mousewheel_direction(SdlMouseDirection::Flipped), MouseWheelDirection::Flipped,"Mapping of mousewheel direction flipped failed");
    }
    #[test]
    fn map_sdl_mousewheel_direction_unknown() {
        let manager = EventManager::new();
        assert_eq!(manager.map_sdl_mousewheel_direction(SdlMouseDirection::Unknown(12)), MouseWheelDirection::Unknown,"Mapping of mousewheel direction unknown failed");
    }
}