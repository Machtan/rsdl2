extern crate sdl2;
use sdl2::Rect;

fn main() {
    println!("Hello World");
    let context = sdl2::init().everything().finish().expect("init failed");
    let mut event_context = context.events().expect("Event subsystem not initialized");
    let video_context = context.video().expect("Video subsystem not initialized");
    let window = video_context.build_window()
        .title("SDL Game")
        .unwrap()
        .center(true, true)
        .finish()
        .expect("Could not create window");
    let renderer = window.build_renderer().finish().expect("Could not build renderer");
    let clear_color = (255, 200, 220);
    let cornflower = (154, 206, 235);
    let rect = Rect::new(100, 100, 100, 100);
    'main: loop {
        use sdl2::events::EventKind::*;
        for event in event_context.events() {
            match event.kind {
                Quit => {
                    println!("User-requested Quit!");
                    break 'main;
                }
                FingerMotion { .. } |
                MouseMotion(_) => {}
                event => {
                    println!("Event: {:?}", event);
                }
            }
        }
        renderer.color(clear_color).clear().unwrap();
        renderer.color(cornflower).fill_rect(rect).unwrap();


        renderer.present();
    }
}
