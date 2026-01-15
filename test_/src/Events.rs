use std::collections::VecDeque;
use sdl3::event::Event as SdlEvent;
use sdl3::event::WindowEvent as SdlWindowEvent;
use std::time::Duration;

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
    pub window_queue:EventQueue<WindowEvent>,
}

impl EventManager {
    pub fn new() -> Self {Self {window_queue:EventQueue::new()  }}

    pub fn start_game_loop(&mut self, sdl_context:sdl3::Sdl, mut canvas:sdl3::render::WindowCanvas) {
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut keep_going = true;
        while keep_going {

            for event in event_pump.poll_iter() {
                keep_going = keep_going && self.match_event_types(event);
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
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::Shown => { 
                let window_event:WindowEvent = WindowEvent::Shown { 
                    timestamp,
                    window_id,
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::Hidden => { 
                let window_event:WindowEvent = WindowEvent::Hidden { 
                    timestamp,
                    window_id,
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::Exposed => { 
                let window_event:WindowEvent = WindowEvent::Exposed { 
                    timestamp,
                    window_id,
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::Moved (x,y) => { 
                let window_event:WindowEvent = WindowEvent::Moved { 
                    timestamp,
                    window_id,
                    x,
                    y,
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::Resized (width,height) => { 
                let window_event:WindowEvent = WindowEvent::Resized { 
                    timestamp,
                    window_id,
                    width,
                    height,
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::PixelSizeChanged (width,height) => { 
                let window_event:WindowEvent = WindowEvent::PixelSizeChanged { 
                    timestamp,
                    window_id,
                    width,
                    height,
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::Minimized => { 
                let window_event:WindowEvent = WindowEvent::Minimized { 
                    timestamp,
                    window_id,
 
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::Maximized => { 
                let window_event:WindowEvent = WindowEvent::Maximized { 
                    timestamp,
                    window_id,
 
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::Restored => { 
                let window_event:WindowEvent = WindowEvent::Restored { 
                    timestamp,
                    window_id,
 
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::MouseEnter => { 
                let window_event:WindowEvent = WindowEvent::MouseEnter { 
                    timestamp,
                    window_id,
 
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::MouseLeave => { 
                let window_event:WindowEvent = WindowEvent::MouseLeave { 
                    timestamp,
                    window_id,
 
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::FocusGained => { 
                let window_event:WindowEvent = WindowEvent::FocusGained { 
                    timestamp,
                    window_id,
 
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::FocusLost => { 
                let window_event:WindowEvent = WindowEvent::FocusLost { 
                    timestamp,
                    window_id,
 
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::CloseRequested => { 
                let window_event:WindowEvent = WindowEvent::CloseRequested { 
                    timestamp,
                    window_id,
 
                };
            self.window_queue.push(window_event);
            return false //TEMPORARY
            }

            SdlWindowEvent::HitTest (x,y) => { 
                let window_event:WindowEvent = WindowEvent::HitTest { 
                    timestamp,
                    window_id,
                    x,
                    y, 
                };
            self.window_queue.push(window_event)
            }

            SdlWindowEvent::DisplayChanged (display_index) => { 
                let window_event:WindowEvent = WindowEvent::DisplayChanged { 
                    timestamp,
                    window_id,
                    display_index, 
                };
            self.window_queue.push(window_event)
            }
            _ => {}
        }
        return true
    }
}
