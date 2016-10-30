extern crate sdl2_sys;
use sdl2_sys as sys;
use std::ffi::CString;

pub fn main() {
    unsafe {
        //sys::SDL_Init(sys::SDL_INIT_EVERYTHING);
        let title = CString::new("Hello, world!").unwrap();
        let window = sys::SDL_CreateWindow(
            title.as_ptr() as *const i8, 
            0, 0, 500, 500, 
            0
        );
        println!("Window: {:?}", window);
        let renderer = sys::SDL_CreateRenderer(
            window, -1, 0
        );
        println!("Renderer: {:?}", renderer);
    
        sys::SDL_SetRenderDrawColor(renderer, 255, 255, 255, 255);
    
        let mut e = sys::SDL_Event { data: [0;56] };
        let mut running = true;
        while running {
            while sys::SDL_PollEvent(&mut e) != 0 {
                let etype = *e.type_();
                println!("Event: {}", etype);
                if (etype == sys::SDL_QUIT) {
                    running = false;
                    break;
                }
            }

            sys::SDL_RenderClear(renderer);
            sys::SDL_RenderPresent(renderer);
        
        }
        sys::SDL_DestroyRenderer(renderer);
        sys::SDL_DestroyWindow(window);
        //sys::SDL_Quit();
    }
}  
