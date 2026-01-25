// pub fn get_event(event: Option<sdl3::event::Event>) -> Option<crate::event::Event> {
//     match event {
//         None => None,
//         Some(sdl3::event::Event::Quit { .. }) => Some(crate::event::Event::Quit),
//         Some(sdl3::event::Event::AppTerminating { .. }) => Some(crate::event::Event::AppTerminating),
//         Some(sdl3::event::Event::KeyDown { .. }) => Some(crate::event::Event::KeyDown),
//         Some(sdl3::event::Event::KeyUp { .. }) => Some(crate::event::Event::KeyUp),
//         _ => Some(crate::event::Event::Unknow),
//     }
// }
