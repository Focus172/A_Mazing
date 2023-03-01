#![allow(non_snake_case)]

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[path ="renderer/screen.rs"]
mod screen;

#[path ="renderer/renderer.rs"]
mod renderer;

#[path ="solver/solver.rs"]
mod solver;

#[path ="maze/maze.rs"]
mod maze;

#[path ="maze/node.rs"]
mod node;

use opengl_graphics::OpenGL;
use screen::Screen;
use std::thread::sleep;
use std::time::Duration;

const TILE_SIZE: i32 = 2;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let image_path = "./inp2.png";
    let mut screen = Screen::new(opengl, image_path)
        .expect("Error creating screen");

    while screen.is_running {
        sleep(Duration::from_millis(1));
        let res = screen.update();
        
        match res {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e);
                screen.is_running = false;
            }
        }
    }
}