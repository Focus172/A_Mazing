#![allow(non_snake_case)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[path ="renderer/screen.rs"]
mod screen;

use opengl_graphics::OpenGL;
use screen::Screen;


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let mut screen = Screen::new(opengl);

    while screen.is_running {
        std::thread::sleep(std::time::Duration::from_millis(5));
        screen.update();
    }
}