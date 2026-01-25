use std::collections::VecDeque;
use sdl3::event::Event as SdlEvent;
use sdl3::event::WindowEvent as SdlWindowEvent;
use std::time::Duration;


use super::app_events::AppEvent; 
use super::window_events::WindowEvent;
use super::input_events::InputEvent; 
use super::input_events::Keycode;
use super::input_events::Keymod;
use super::input_events::Scancode; 
use super::input_events::MouseWheelDirection; 
use super::input_events::MouseButtonState; 
use super::input_events::MouseButton; 



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
            self.input_events_queue.pop();
            self.app_events_queue.pop();
            self.window_events_queue.pop();

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

    fn map_sdl_mousestate(& self, ms: &sdl3::mouse::MouseState) -> MouseButtonState {
        MouseButtonState {
            left: ms.left(),
            right: ms.right(),
            middle: ms.middle(),
            x1: ms.x1(),
            x2: ms.x2(),
        }
    }

    pub fn map_sdl_mousewheel_direction(& self ,direction: sdl3::mouse::MouseWheelDirection) -> MouseWheelDirection {
        match direction {
            sdl3::mouse::MouseWheelDirection::Normal => MouseWheelDirection::Normal,
            sdl3::mouse::MouseWheelDirection::Flipped => MouseWheelDirection::Flipped,
            _ => MouseWheelDirection::Unknown,
        }
    }

    //Paradigm choice for now: When there is no keycode value -> mapped to unknown value
    fn map_sdl_keycode(& self, sdl_keycode: Option<sdl3::keyboard::Keycode> , timestamp:u64) -> Keycode {
        match sdl_keycode {
            Some(k) => Keycode::try_from(k as u32).unwrap_or(Keycode::Unknown),
            None => {
                println!("key pressed without keycode, timestamp = {:?}", timestamp);
                Keycode::Unknown
            }
        }
    }

    //Paradigm choice for now: When there is no scancode value -> mapped to unknown value
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
