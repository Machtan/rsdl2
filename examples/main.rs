extern crate sdl2;

fn main() {
    println!("Hello World");
    let context = sdl2::init().everything().finish().expect("init failed");
    let mut event_context = context.events().expect("Event subsystem not initialized");
    'main: loop {
        use sdl2::events::*;
        use sdl2::events::EventKind::*;
        for event in event_context.events() {
            match event.kind {
                Quit => {
                    println!("User-requested Quit!");
                    break 'main;
                }
                event => {
                    println!("Event: {:?}", event);
                }
            }
        }
    }
}
