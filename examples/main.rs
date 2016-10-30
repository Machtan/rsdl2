extern crate sdl2;

fn main() {
    println!("Hello World");
    let context = sdl2::init().everything().expect("init failed");
    println!("Context: {:?}", context);
}
