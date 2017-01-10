extern crate rsdl2;
use rsdl2::{Rect, Surface};


fn main() {
    println!("Hello World");
    let context = rsdl2::init().everything().finish().expect("init failed");
    let mut event_context = context.events().expect("Event subsystem not initialized");
    let video_context = context.video().expect("Video subsystem not initialized");
    let window = video_context.build_window()
        .title("SDL Game")
        .center(true, true)
        .finish()
        .expect("Could not create window");
    let renderer = window.build_renderer().finish().expect("Could not build renderer");
    // This is where I should've initialized SDL_image
    let clear_color = (255, 200, 220);
    let cornflower = (154, 206, 235);
    let rect = Rect::new(100, 100, 100, 100);
    let sloth_path = "resources/sloth.bmp";
    let sloth_surf = Surface::load_from_bmp(sloth_path).expect("Could not load bmp");
    let sloth_tex = renderer.create_texture_from_surface(&sloth_surf)
        .expect("Could not create texture");
    let sloth_pos = Rect::new(200, 100, 100, 100);
    //let coony_path = "resources/coony.png";
    //let coony_tex = renderer.load_image(coony_path).expect("Could not load coony.png");
    //let coony_pos = Rect::new(100, 200, 100, 100);
    'main: loop {
        use rsdl2::events::EventKind::*;
        for event in event_context.events() {
            match event.kind {
                Quit => {
                    println!("User-requested Quit!");
                    break 'main;
                }
                FingerMotion { .. } |
                MouseMotion(_) => {}
                KeyDown(sym) => {
                    println!("Key: {:?}", sym.keycode);
                }
                event => {
                    println!("Event: {:?}", event);
                }
            }
        }
        renderer.color(clear_color).clear().unwrap();
        renderer.color(cornflower).fill_rect(rect).unwrap();
        renderer.copy(&sloth_tex, None, Some(sloth_pos)).unwrap();
        //renderer.copy(&coony_tex, None, Some(coony_pos)).unwrap();

        renderer.present();
    }
}
